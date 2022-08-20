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
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;
use crate::xsfprintln;

fn on_or_off(b: bool) -> &'static str
{
    if b {
        "on"
    } else {
        "off"
    }
}

fn minus_or_plus(b: bool) -> char
{
    if b {
        '-'
    } else {
        '+'
    }
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    let res = settings.parse_options(args, |opt_type, c, settings| {
            match (opt_type, c) {
                (OptionType::Minus, 'o') => {
                    match exec.current_file(1) {
                        Some(stdout_file) => {
                            let mut stdout_file_r = stdout_file.borrow_mut();
                            let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                            fprintln!(&mut line_stdout, "allexport       {}", on_or_off(settings.allexport_flag));
                            fprintln!(&mut line_stdout, "errexit         {}", on_or_off(settings.errexit_flag));
                            fprintln!(&mut line_stdout, "ignoreeof       {}", on_or_off(settings.ignoreeof_flag));
                            fprintln!(&mut line_stdout, "monitor         {}", on_or_off(settings.monitor_flag));
                            fprintln!(&mut line_stdout, "noclobber       {}", on_or_off(settings.noclobber_flag));
                            fprintln!(&mut line_stdout, "noglob          {}", on_or_off(settings.noglob_flag));
                            fprintln!(&mut line_stdout, "noexec          {}", on_or_off(settings.noexec_flag));
                            fprintln!(&mut line_stdout, "nolog           {}", on_or_off(settings.nolog_flag));
                            fprintln!(&mut line_stdout, "notify          {}", on_or_off(settings.notify_flag));
                            fprintln!(&mut line_stdout, "nounset         {}", on_or_off(settings.nounset_flag));
                            fprintln!(&mut line_stdout, "verbose         {}", on_or_off(settings.verbose_flag));
                            fprintln!(&mut line_stdout, "vi              {}", on_or_off(settings.vi_flag));
                            fprintln!(&mut line_stdout, "emacs           {}", on_or_off(settings.emacs_flag));
                            fprintln!(&mut line_stdout, "xtrace          {}", on_or_off(settings.xtrace_flag));
                            fprintln!(&mut line_stdout, "strlossy        {}", on_or_off(settings.strlossy_flag));
                        },
                        None => xsfprintln!(exec, 2, "No standard output"),
                    }
                    true
                },
                (OptionType::Plus, 'o') => {
                    match exec.current_file(1) {
                        Some(stdout_file) => {
                            let mut stdout_file_r = stdout_file.borrow_mut();
                            let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                            fprintln!(&mut line_stdout, "set {}o allexport", minus_or_plus(settings.allexport_flag));
                            fprintln!(&mut line_stdout, "set {}o errexit", minus_or_plus(settings.errexit_flag));
                            fprintln!(&mut line_stdout, "set {}o ignoreeof", minus_or_plus(settings.ignoreeof_flag));
                            fprintln!(&mut line_stdout, "set {}o monitor", minus_or_plus(settings.monitor_flag));
                            fprintln!(&mut line_stdout, "set {}o noclobber", minus_or_plus(settings.noclobber_flag));
                            fprintln!(&mut line_stdout, "set {}o noglob", minus_or_plus(settings.noglob_flag));
                            fprintln!(&mut line_stdout, "set {}o noexec", minus_or_plus(settings.noexec_flag));
                            fprintln!(&mut line_stdout, "set {}o nolog", minus_or_plus(settings.nolog_flag));
                            fprintln!(&mut line_stdout, "set {}o notify", minus_or_plus(settings.notify_flag));
                            fprintln!(&mut line_stdout, "set {}o nounset", minus_or_plus(settings.nounset_flag));
                            fprintln!(&mut line_stdout, "set {}o verbose", minus_or_plus(settings.verbose_flag));
                            fprintln!(&mut line_stdout, "set {}o vi", minus_or_plus(settings.vi_flag));
                            fprintln!(&mut line_stdout, "set {}o emacs", minus_or_plus(settings.emacs_flag));
                            fprintln!(&mut line_stdout, "set {}o xtrace", minus_or_plus(settings.xtrace_flag));
                            fprintln!(&mut line_stdout, "set {}o strlossy", minus_or_plus(settings.strlossy_flag));
                        },
                        None => xsfprintln!(exec, 2, "No standard output"),
                    }
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            if args.len() <= 1 {
                match exec.current_file(1) {
                    Some(stdout_file) => {
                        let mut stdout_file_r = stdout_file.borrow_mut();
                        let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                        for (name, value) in env.unexported_vars().iter() {
                            fprintln!(&mut line_stdout, "{}={}", name, singly_quote_str(value.as_str()));
                        }
                        for (name, value) in env.exported_vars() {
                            fprintln!(&mut line_stdout, "{}={}", name, singly_quote_str(value.as_str()));
                        }
                        0
                    },
                    None => {
                        xsfprintln!(exec, 2, "No standard output");
                        interp.exit(1, false)
                    },
                }
            } else {
                if i < args.len() || is_minus_minus {
                    let args: Vec<String> = args.iter().skip(i).map(|a| a.clone()).collect();
                    settings.current_args_mut().set_args(args);
                }
                0
            }
        },
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
    fn test_set_builtin_function_sets_some_flag()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.allexport_flag = false;
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
            String::from("set"),
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
        assert_eq!(true, settings.allexport_flag);
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_unsets_some_flag()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.allexport_flag = true;
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
            String::from("set"),
            String::from("+a")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(false, settings.allexport_flag);
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_sets_some_flag_for_minus_o_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.ignoreeof_flag = false;
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
            String::from("set"),
            String::from("-o"),
            String::from("ignoreeof")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(true, settings.ignoreeof_flag);
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_unsets_some_flag_for_plus_o_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.ignoreeof_flag = true;
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
            String::from("set"),
            String::from("+o"),
            String::from("ignoreeof")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(false, settings.ignoreeof_flag);
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_sets_arguments()
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
            String::from("set"),
            String::from("jkl"),
            String::from("mno")
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
            String::from("jkl"),
            String::from("mno")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_sets_empty_arguments()
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
            String::from("set"),
            String::from("--")
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
    fn test_set_builtin_function_prints_flags_for_minus_o_option()
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
            String::from("set"),
            String::from("-o")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
allexport       off
errexit         off
ignoreeof       off
monitor         on
noclobber       off
noglob          off
noexec          off
nolog           off
notify          on
nounset         off
verbose         off
vi              off
emacs           off
xtrace          off
strlossy        off
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_prints_flags_for_plus_o_option()
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
            String::from("set"),
            String::from("+o")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        println!("{}", read_file("stdout.txt"));
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
set +o allexport
set +o errexit
set +o ignoreeof
set -o monitor
set +o noclobber
set +o noglob
set +o noexec
set +o nolog
set -o notify
set +o nounset
set +o verbose
set +o vi
set +o emacs
set +o xtrace
set +o strlossy
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_set_builtin_function_prints_variables()
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
            String::from("set")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let stdout_content = read_file("stdout.txt");
        assert!(stdout_content.contains(format!("PWD='{}'\n", current_dir().as_path().to_string_lossy()).as_str()));
        assert!(stdout_content.contains(format!("PPID='{}'\n", getppid()).as_str()));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let expected_current_args = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi")
        ];
        assert_eq!(expected_current_args.as_slice(), settings.current_args().args());
    }    
}
