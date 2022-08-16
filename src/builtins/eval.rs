//
// Rsush - Rust single unix shell.
// Copyright (C) 2022 Łukasz Szpakowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
use std::io::*;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::io::*;
use crate::lexer::*;
use crate::parser::*;
use crate::settings::*;
use crate::xsfprintln;

pub fn main(vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    for (name, value) in vars.iter() {
        if env.read_only_var_attr(name) {
            xsfprintln!(exec, 2, "{}: Is read only", name);
            return interp.exit(1, false);
        }
        env.unset_unexported_var(name.as_str());
        env.set_exported_var(name.as_str(), value.as_str());
    }
    if args.len() < 1 {
        xsfprintln!(exec, 2, "No built-in function name");
        return interp.exit(1, false);
    }
    let s = (&args[1..]).join(" ");
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("(evaluation)", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    parser.set_error_cont(false);
    match parser.parse_logical_commands(&mut lexer, settings) {
        Ok(commands) => interp.interpret_logical_commands(exec, commands.as_slice(), env, settings),
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            interp.exit(1, false)
        },
    }
}

#[cfg(test)]
mod tests
{
    use std::cell::*;
    use std::rc::*;
    use super::*;
    use crate::builtins::*;
    use crate::test_builtins::*;
    use crate::vars::*;
    use crate::test_helpers::*;
    use sealed_test::prelude::*;

    fn setup()
    { symlink_rsush_test(); }

    fn teardown()
    { remove_rsush_test(); }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_eval_builtin_function_interprets_command_from_string()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("eval"),
            String::from("echo abc def")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
abc def
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_eval_builtin_function_interprets_command_from_three_strings()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("eval"),
            String::from("echo abc def"),
            String::from("ghi"),
            String::from("jkl")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
abc def ghi jkl
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_eval_builtin_function_sets_variables()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let vars = vec![
            (String::from("VAR1"), String::from("def")),
            (String::from("VAR2"), String::from("ghi"))
        ];
        let args = vec![
            String::from("eval"),
            String::from("echo abc $VAR1 $VAR2")
        ];
        let status = main(vars.as_slice(), args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
abc def ghi
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("def")), env.exported_var("VAR1"));
        assert_eq!(Some(String::from("ghi")), env.exported_var("VAR2"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_eval_builtin_function_complains_on_parser_error()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        env.set_read_only_var_attr("VAR2");
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("eval"),
            String::from("echo abc )")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("(evaluation): 1.10: unexpected token\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_eval_builtin_function_complains_on_variable_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_read_only_var_attr("VAR2");
        env.unset_var("VAR2");
        env.unset_var("VAR3");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let vars = vec![
            (String::from("VAR1"), String::from("def")),
            (String::from("VAR2"), String::from("ghi")),
            (String::from("VAR3"), String::from("jkl"))
        ];
        let args = vec![
            String::from("eval"),
            String::from("echo abc $VAR1 $VAR2 $VAR3")
        ];
        let status = main(vars.as_slice(), args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("VAR2: Is read only\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("def")), env.exported_var("VAR1"));
        assert!(env.exported_var("VAR2").is_none());
        assert!(env.exported_var("VAR3").is_none());
    }
}
