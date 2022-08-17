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
use crate::xsfprintln;

struct Options
{
    fun_flag: bool,
    var_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, _settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "fv");
    let mut opts = Options {
        fun_flag: false,
        var_flag: false,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('f', _))) => opts.fun_flag = true,
            Some(Ok(Opt('v', _))) => opts.var_flag = true,
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
    let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    for name in names {
        if opts.fun_flag && opts.var_flag {
            if env.read_only_var_attr(name.as_str()) {
                xsfprintln!(exec, 2, "{}: Is read only", name);
                return interp.exit(1, false);
            }
            env.unset_var(name.as_str());
            env.unset_fun(name.as_str());
        } else if opts.fun_flag {
            env.unset_fun(name.as_str());
        } else if opts.var_flag {
            if env.read_only_var_attr(name.as_str()) {
                xsfprintln!(exec, 2, "{}: Is read only", name);
                return interp.exit(1, false);
            }
            env.unset_var(name.as_str());
        } else {
            if env.var(name.as_str()).is_some() {
                if env.read_only_var_attr(name.as_str()) {
                    xsfprintln!(exec, 2, "{}: Is read only", name);
                    return interp.exit(1, false);
                }
                env.unset_var(name.as_str());
            } else {
                env.unset_fun(name.as_str());
            }
        }
    }
    0
}

#[cfg(test)]
mod tests
{
    use std::cell::*;
    use std::io::*;
    use std::rc::*;
    use super::*;
    use crate::io::*;
    use crate::lexer::*;
    use crate::parser::*;
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
    fn test_unset_builtin_function_removes_variables()
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
        env.set_unexported_var("VAR2", "def");
        env.unset_exported_var("VAR2");
        env.set_unexported_var("VAR3", "ghi");
        env.unset_exported_var("VAR3");        
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("VAR1"),
            String::from("VAR2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR1").is_none());
        assert!(env.exported_var("VAR1").is_none());
        assert!(env.unexported_var("VAR2").is_none());
        assert!(env.exported_var("VAR2").is_none());
        assert_eq!(Some(String::from("ghi")), env.unexported_var("VAR3"));
        assert!(env.exported_var("VAR3").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_variable()
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
        env.set_unexported_var("VAR2", "def");
        env.unset_exported_var("VAR2");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("VAR1")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR1").is_none());
        assert!(env.exported_var("VAR1").is_none());
        assert_eq!(Some(String::from("def")), env.unexported_var("VAR2"));
        assert!(env.exported_var("VAR2").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_variable_and_does_not_remove_function()
    {
        let s = "
f() {
    echo abc
}
g() {
    echo def
}
";
        let s2 = &s[1..];
        let mut cursor = Cursor::new(s2.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
        let mut parser = Parser::new();
        let settings = Settings::new();
        match parser.parse_logical_commands(&mut lexer, &settings) {
            Ok(logical_commands) => {
                let mut exec = Executor::new();
                let mut interp = Interpreter::new();
                let mut env = Environment::new();
                let mut settings = Settings::new();
                settings.arg0 = String::from("rsush");
                initialize_builtin_funs(&mut env);
                initialize_test_builtin_funs(&mut env);
                initialize_vars(&mut env);
                env.set_unexported_var("f", "abc");
                env.unset_exported_var("f");
                env.set_unexported_var("g", "def");
                env.unset_exported_var("g");
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("unset"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                assert_eq!(String::new(), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
                assert!(env.unexported_var("f").is_none());
                assert!(env.exported_var("f").is_none());
                assert!(env.fun("f").is_some());
                assert_eq!(Some(String::from("def")), env.unexported_var("g"));
                assert!(env.exported_var("g").is_none());
                assert!(env.fun("g").is_some());
            },
            _ => assert!(false),
        }
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_function()
    {
        let s = "
f() {
    echo abc
}
g() {
    echo def
}
";
        let s2 = &s[1..];
        let mut cursor = Cursor::new(s2.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
        let mut parser = Parser::new();
        let settings = Settings::new();
        match parser.parse_logical_commands(&mut lexer, &settings) {
            Ok(logical_commands) => {
                let mut exec = Executor::new();
                let mut interp = Interpreter::new();
                let mut env = Environment::new();
                let mut settings = Settings::new();
                settings.arg0 = String::from("rsush");
                initialize_builtin_funs(&mut env);
                initialize_test_builtin_funs(&mut env);
                initialize_vars(&mut env);
                env.unset_var("f");
                env.set_unexported_var("g", "def");
                env.unset_exported_var("g");
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("unset"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                assert_eq!(String::new(), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
                assert!(env.unexported_var("f").is_none());
                assert!(env.exported_var("f").is_none());
                assert!(env.fun("f").is_none());
                assert_eq!(Some(String::from("def")), env.unexported_var("g"));
                assert!(env.exported_var("g").is_none());
                assert!(env.fun("g").is_some());
            },
            _ => assert!(false),
        }
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_function_and_does_not_remove_variable_for_f_option()
    {
        let s = "
f() {
    echo abc
}
g() {
    echo def
}
";
        let s2 = &s[1..];
        let mut cursor = Cursor::new(s2.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
        let mut parser = Parser::new();
        let settings = Settings::new();
        match parser.parse_logical_commands(&mut lexer, &settings) {
            Ok(logical_commands) => {
                let mut exec = Executor::new();
                let mut interp = Interpreter::new();
                let mut env = Environment::new();
                let mut settings = Settings::new();
                settings.arg0 = String::from("rsush");
                initialize_builtin_funs(&mut env);
                initialize_test_builtin_funs(&mut env);
                initialize_vars(&mut env);
                env.set_unexported_var("f", "abc");
                env.unset_exported_var("f");
                env.set_unexported_var("g", "def");
                env.unset_exported_var("g");
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("unset"),
                    String::from("-f"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                assert_eq!(String::new(), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
                assert_eq!(Some(String::from("abc")), env.unexported_var("f"));
                assert!(env.exported_var("f").is_none());
                assert!(env.fun("f").is_none());
                assert_eq!(Some(String::from("def")), env.unexported_var("g"));
                assert!(env.exported_var("g").is_none());
                assert!(env.fun("g").is_some());
            },
            _ => assert!(false),
        }
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_variable_and_does_not_remove_function_for_v_option()
    {
        let s = "
f() {
    echo abc
}
g() {
    echo def
}
";
        let s2 = &s[1..];
        let mut cursor = Cursor::new(s2.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
        let mut parser = Parser::new();
        let settings = Settings::new();
        match parser.parse_logical_commands(&mut lexer, &settings) {
            Ok(logical_commands) => {
                let mut exec = Executor::new();
                let mut interp = Interpreter::new();
                let mut env = Environment::new();
                let mut settings = Settings::new();
                settings.arg0 = String::from("rsush");
                initialize_builtin_funs(&mut env);
                initialize_test_builtin_funs(&mut env);
                initialize_vars(&mut env);
                env.set_unexported_var("f", "abc");
                env.unset_exported_var("f");
                env.set_unexported_var("g", "def");
                env.unset_exported_var("g");
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("unset"),
                    String::from("-v"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                assert_eq!(String::new(), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
                assert!(env.unexported_var("f").is_none());
                assert!(env.exported_var("f").is_none());
                assert!(env.fun("f").is_some());
                assert_eq!(Some(String::from("def")), env.unexported_var("g"));
                assert!(env.exported_var("g").is_none());
                assert!(env.fun("g").is_some());
            },
            _ => assert!(false),
        }
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_function_and_variable_for_f_and_v_options()
    {
        let s = "
f() {
    echo abc
}
g() {
    echo def
}
";
        let s2 = &s[1..];
        let mut cursor = Cursor::new(s2.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
        let mut parser = Parser::new();
        let settings = Settings::new();
        match parser.parse_logical_commands(&mut lexer, &settings) {
            Ok(logical_commands) => {
                let mut exec = Executor::new();
                let mut interp = Interpreter::new();
                let mut env = Environment::new();
                let mut settings = Settings::new();
                settings.arg0 = String::from("rsush");
                initialize_builtin_funs(&mut env);
                initialize_test_builtin_funs(&mut env);
                initialize_vars(&mut env);
                env.set_unexported_var("f", "abc");
                env.unset_exported_var("f");
                env.set_unexported_var("g", "def");
                env.unset_exported_var("g");
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("unset"),
                    String::from("-fv"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                assert_eq!(String::new(), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
                assert!(env.unexported_var("f").is_none());
                assert!(env.exported_var("f").is_none());
                assert!(env.fun("f").is_none());
                assert_eq!(Some(String::from("def")), env.unexported_var("g"));
                assert!(env.exported_var("g").is_none());
                assert!(env.fun("g").is_some());
            },
            _ => assert!(false),
        }
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_unexported_variable()
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
        env.set_unexported_var("VAR2", "def");
        env.unset_exported_var("VAR2");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("VAR1")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR1").is_none());
        assert!(env.exported_var("VAR1").is_none());
        assert_eq!(Some(String::from("def")), env.unexported_var("VAR2"));
        assert!(env.exported_var("VAR2").is_none());
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_removes_exported_variable()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_unexported_var("VAR1");
        env.set_exported_var("VAR1", "abc");
        env.unset_unexported_var("VAR2");
        env.set_exported_var("VAR2", "def");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("VAR1")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("VAR1").is_none());
        assert!(env.exported_var("VAR1").is_none());
        assert!(env.unexported_var("VAR2").is_none());
        assert_eq!(Some(String::from("def")), env.exported_var("VAR2"));
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_complains_on_variable_is_read_only()
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
        env.set_read_only_var_attr("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("VAR")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_complains_on_variable_is_read_only_for_v_option()
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
        env.set_read_only_var_attr("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("-v"),
            String::from("VAR")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_unset_builtin_function_complains_on_variable_is_read_only_for_f_and_v_options()
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
        env.set_read_only_var_attr("VAR");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("unset"),
            String::from("-fv"),
            String::from("VAR")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("VAR"));
        assert!(env.exported_var("VAR").is_none());
    }
}
