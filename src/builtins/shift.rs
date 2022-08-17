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
use crate::xsfprintln;

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, settings: &mut Settings) -> i32
{
    let n = match args.get(1) {
        Some(arg) => {
            if args.len() > 2 {
                xsfprintln!(exec, 2, "Too many arguments");
                return interp.exit(1, false);
            }
            match arg.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    xsfprintln!(exec, 2, "Invalid number");
                    return interp.exit(1, false);
                },
            }
        },
        None => 1,
    };
    let current_args = settings.current_args_mut();
    if n <= current_args.args().len() {
        if n != 0 {
            current_args.set_args((&current_args.args()[n..]).to_vec());
        }
    } else {
        current_args.set_args(Vec::new());
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
    fn test_shift_builtin_function_shifts_arguments()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("shift")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args = vec![
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_shift_builtin_function_shifts_arguments_for_two()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("shift"),
            String::from("2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args = vec![
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_shift_builtin_function_shifts_arguments_for_three()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("shift"),
            String::from("3")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args: Vec<String> = Vec::new();
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_shift_builtin_function_shifts_arguments_for_four()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        let current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        settings.current_args_mut().set_args(current_args);
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("shift"),
            String::from("4")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args: Vec<String> = Vec::new();
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_shift_builtin_function_complains_on_too_many_arguments()
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
            String::from("shift"),
            String::from("2"),
            String::from("4")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("Too many arguments\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_shift_builtin_function_complains_on_invalid_number()
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
            String::from("shift"),
            String::from("xxx")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_exit_with(false));
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::from("Invalid number\n"), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
}
