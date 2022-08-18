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
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::xcfprintln;

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    match args.get(1) {
        Some(opts) => {
            match args.get(2) {
                Some(name) => {
                    if !is_name_str(name.as_str()) {
                        xcfprintln!(exec, 2, "{}: Invalid variable name", name);
                        return 1;
                    }
                    if env.read_only_var_attr(name.as_str()) {
                        xcfprintln!(exec, 2, "{}: Is read only", name);
                        return 1;
                    }
                    if env.read_only_var_attr("OPTARG") {
                        xcfprintln!(exec, 2, "OPTARG: Is read only");
                        return 1;
                    }
                    if env.read_only_var_attr("OPTIND") {
                        xcfprintln!(exec, 2, "OPTIND: Is read only");
                        return 1;
                    }
                    let args = if args.len() > 3 {
                        Some(&args[3..])
                    } else {
                        None
                    };
                    match settings.current_args_mut().get_option(opts.as_str(), args) {
                        Ok(None) => {
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            1
                        },
                        Ok(Some((c, opt_arg))) => {
                            env.set_var(name.as_str(), format!("{}", c).as_str(), settings);
                            match opt_arg {
                                Some(opt_arg) => env.set_var("OPTARG", format!("{}", opt_arg).as_str(), settings),
                                None => env.set_var("OPTARG", "", settings),
                            }
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            0
                        },
                        Err(err) => {
                            env.set_var(name.as_str(), "?", settings);
                            env.set_var("OPTARG", "", settings);
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            xcfprintln!(exec, 2, "{}", err);
                            0
                        },
                    }
                },
                None => {
                    xcfprintln!(exec, 2, "Too few arguments");
                    1
                },
            }
        },
        None => {
            xcfprintln!(exec, 2, "Too few arguments");
            1
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
    fn test_getopts_builtin_function_parses_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("-a")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("a")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("2")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_parses_option_with_argument()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("-b"),
            String::from("abc")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("b")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("abc")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("3")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_does_not_parse_argument()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("abc")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("var").is_none());
        assert!(env.exported_var("var").is_none());
        assert!(env.unexported_var("OPTARG").is_none());
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("1")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_parses_option_for_other_arguments()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var"),
            String::from("-a")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("a")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("2")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_parses_option_with_argument_for_other_arguments()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var"),
            String::from("-b"),
            String::from("abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("b")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("abc")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("3")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }
    
        #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_does_not_parse_argument_for_other_arguments()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var"),
            String::from("abc")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert!(env.unexported_var("var").is_none());
        assert!(env.exported_var("var").is_none());
        assert!(env.unexported_var("OPTARG").is_none());
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("1")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_unknown_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("-d")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("unknown option -- 'd'\n"), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("?")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("2")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_option_requires_argument()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("-b")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("option requires an argument -- 'b'\n"), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("?")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("2")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_returns_colon_character_for_option_requires_argument()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("-d")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from(":ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("?")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("OPTARG"));
        assert!(env.exported_var("OPTARG").is_none());
        assert_eq!(Some(String::from("2")), env.unexported_var("OPTIND"));
        assert!(env.exported_var("OPTIND").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_too_few_arguments()
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
            String::from("getopt"),
            String::from("ab:c")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("Too few arguments\n"), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_invalid_varianle_name()
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
            String::from("getopt"),
            String::from("ab:c"),
            String::from("!@#")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("!@#: Invalid variable name\n"), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_var_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.set_read_only_var_attr("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("var: Is read only\n"), read_file("stderr2.txt"));
        assert!(env.unexported_var("var").is_none());
        assert!(env.exported_var("var").is_none());
        assert!(env.unexported_var("OPTARG").is_none());
        assert!(env.exported_var("OPTARG").is_none());
        assert!(env.unexported_var("OPTIND").is_none());
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_optarg_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.set_read_only_var_attr("OPTARG");
        env.unset_var("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("OPTARG: Is read only\n"), read_file("stderr2.txt"));
        assert!(env.unexported_var("var").is_none());
        assert!(env.exported_var("var").is_none());
        assert!(env.unexported_var("OPTARG").is_none());
        assert!(env.exported_var("OPTARG").is_none());
        assert!(env.unexported_var("OPTIND").is_none());
        assert!(env.exported_var("OPTIND").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_getopts_builtin_function_complains_on_optind_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("var");
        env.unset_var("OPTARG");
        env.unset_var("OPTIND");
        env.set_read_only_var_attr("OPTIND");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("getopt"),
            String::from("ab:c"),
            String::from("var")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("OPTIND: Is read only\n"), read_file("stderr2.txt"));
        assert!(env.unexported_var("var").is_none());
        assert!(env.exported_var("var").is_none());
        assert!(env.unexported_var("OPTARG").is_none());
        assert!(env.exported_var("OPTARG").is_none());
        assert!(env.unexported_var("OPTIND").is_none());
        assert!(env.exported_var("OPTIND").is_none());
    }
}
