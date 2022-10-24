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
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::xcfprintln;
use crate::xsfprintln;

struct Options
{
    print_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "p");
    let mut opts = Options {
        print_flag: false,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('p', _))) => opts.print_flag = true,
            Some(Ok(Opt(c, _))) => {
                xsfprintln!(exec, 2, "unknown option -- {:?}", c);
                return interp.exit(1, false);
            },
            Some(Err(err)) => {
                xsfprintln!(exec, 2, "{}", err);
                return interp.exit(1, false);
            },
            None => break,
        }
    }
    let args: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    for arg in &args {
        match arg.split_once('=') {
            Some((name, value)) => {
                if !is_name_str(name) {
                    xsfprintln!(exec, 2, "{}: Invalid variable name", name);
                    return interp.exit(1, false);
                }
                if env.read_only_var_attr(name) {
                    xsfprintln!(exec, 2, "{}: Is read only", name);
                    return interp.exit(1, false);
                }
                env.set_var(name, value, settings);
                env.set_read_only_var_attr(name);
            },
            None => {
                if !is_name_str(arg.as_str()) {
                    xsfprintln!(exec, 2, "{}: Invalid variable name", arg);
                    return interp.exit(1, false);
                }
                env.set_read_only_var_attr(arg.as_str());
            },
        }
    }
    if args.is_empty() || opts.print_flag {
        for name in env.read_only_var_attrs().iter() {
            match env.var(name.as_str()) {
                Some(value) => xcfprintln!(exec, 1, "readonly {}={}", name, singly_quote_str(value.as_str())),
                None => xcfprintln!(exec, 1, "readonly {}", name),
            }
        }
    }
    0
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
    fn test_readonly_builtin_function_sets_read_only_variable_attributes()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("VAR1");
        env.unset_var("VAR2");
        env.unset_var("VAR3");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR1=abc"),
            String::from("VAR2=def"),
            String::from("VAR3=ghi")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR1"));
        assert!(env.exported_var("VAR1").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR1"));
        assert_eq!(Some(String::from("def")), env.unexported_var("VAR2"));
        assert!(env.exported_var("VAR2").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR2"));
        assert_eq!(Some(String::from("ghi")), env.unexported_var("VAR3"));
        assert!(env.exported_var("VAR3").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR3"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_sets_read_only_variable_attribute_and_sets_value()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_unexported_var("VAR", "def");
        env.unset_exported_var("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR=abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_sets_read_only_unset_variable_attribute_and_sets_value()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR=abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_sets_read_only_variable_attribute()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_unexported_var("VAR", "abc");
        env.unset_exported_var("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_sets_read_only_unset_variable_attribute()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR").is_none());
        assert!(env.exported_var("VAR").is_none());
        assert_eq!(true, env.read_only_var_attr("VAR"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_prints_read_only_variables()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_unexported_var("VAR1", "abc");
        env.unset_exported_var("VAR1");
        env.set_read_only_var_attr("VAR1");
        env.set_unexported_var("VAR2", "def");
        env.unset_exported_var("VAR2");
        env.set_read_only_var_attr("VAR2");
        env.unset_var("VAR3");
        env.set_read_only_var_attr("VAR3");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let stdout_content = read_file("stdout.txt");
        assert!(stdout_content.contains("readonly VAR1='abc'\n"));
        assert!(stdout_content.contains("readonly VAR2='def'\n"));
        assert!(stdout_content.contains("readonly VAR3\n"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_sets_read_only_variable_attributes_and_prints_read_only_variables()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_unexported_var("VAR1", "abc");
        env.unset_exported_var("VAR1");
        env.set_read_only_var_attr("VAR1");
        env.unset_var("VAR2");
        env.unset_var("VAR3");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("-p"),
            String::from("VAR2=def"),
            String::from("VAR3")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let stdout_content = read_file("stdout.txt");
        assert!(stdout_content.contains("readonly VAR1='abc'\n"));
        assert!(stdout_content.contains("readonly VAR2='def'\n"));
        assert!(stdout_content.contains("readonly VAR3\n"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_complains_on_invalid_variable_name()
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
            String::from("readonly"),
            String::from("!@#")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("!@#: Invalid variable name\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_complains_on_invalid_variable_name_for_value_setting()
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
            String::from("readonly"),
            String::from("!@#=abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("!@#: Invalid variable name\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_readonly_builtin_function_complains_on_variable_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_read_only_var_attr("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("readonly"),
            String::from("VAR=abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR").is_none());
        assert!(env.exported_var("VAR").is_none());
    }
}
