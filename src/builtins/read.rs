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
use crate::exec_utils::*;
use crate::interp::*;
use crate::io::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;

struct Options
{
    ignored_escape_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    with_std_files(exec, |stdin, _, stderr| {
            let mut line_stdin = LineReader::new(stdin);
            let mut opt_parser = getopt::Parser::new(args, "r");
            let mut opts = Options {
                ignored_escape_flag: false,
            };
            loop {
                match opt_parser.next() {
                    Some(Ok(Opt('r', _))) => opts.ignored_escape_flag = true,
                    Some(Ok(Opt(c, _))) => {
                        fprintln!(stderr, "unknown option -- {:?}", c);
                        return 1;
                    },
                    Some(Err(err)) => {
                        fprintln!(stderr, "{}", err);
                        return 1;
                    },
                    None => break,
                }
            }
            let mut s = String::new(); 
            let mut is_eof = false;
            if !opts.ignored_escape_flag {
                let mut is_first = true;
                loop {
                    let mut is_stop = true;
                    let mut line = String::new();
                    match line_stdin.read_line(&mut line) {
                        Ok(0) => {
                            if is_first { is_eof = true; }
                            break;
                        },
                        Ok(_) => {
                            let mut iter = line.chars();
                            loop {
                                match iter.next() {
                                    Some('\\') => {
                                        match iter.next() {
                                            Some('\n') => {
                                                is_stop = false;
                                                break;
                                            },
                                            Some(c) => s.push(c),
                                            None => break,
                                        }
                                    },
                                    Some('\n') => break,
                                    Some(c) => s.push(c),
                                    None => break,
                                }
                            }
                        },
                        Err(err) => {
                            fprintln!(stderr, "{}", err);
                            return 1;
                        },
                    }
                    if is_stop { break; }
                    is_first = false;
                }
            } else {
                let mut line = String::new();
                match line_stdin.read_line(&mut line) {
                    Ok(n) => {
                        if n == 0 { is_eof = true; }
                        let line_without_newline = str_without_newline(line.as_str());
                        s.push_str(line_without_newline);
                    },
                    Err(err) => {
                        fprintln!(stderr, "{}", err);
                        return 1;
                    },
                }
            }
            let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
            let fields = if !ifs.is_empty() {
                split_str_for_ifs(s.as_str(), ifs.as_str())
            } else {
                vec![s.as_str()]
            };
            let mut status = 0;
            let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
            for (i, name) in names.iter().enumerate() {
                if !is_name_str(name) {
                    fprintln!(stderr, "{}: Invalid variable name", name);
                    status = 1;
                    continue;
                }
                if env.read_only_var_attr(name.as_str()) {
                    fprintln!(stderr, "{}: Is read only", name);
                    status = 1;
                    continue;
                }
                match fields.get(i) {
                    Some(value) => env.set_var(name.as_str(), value, settings),
                    None => env.set_var(name.as_str(), "", settings),
                }
            }
            if is_eof { status = 1; }
            status
    }).unwrap_or(1)
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
    fn test_read_builtin_function_reads_fields()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("Some")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("line")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_last_field_that_is_not_read()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        env.unset_var("var3");
        write_file("stdin.txt", "abc def\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2"),
            String::from("var3")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("def")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
        assert_eq!(Some(String::new()), env.unexported_var("var3"));
        assert!(env.exported_var("var3").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_escapes()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var");
        write_file("stdin.txt", "a\\b\\c\\d\\ef\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
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
        assert_eq!(Some(String::from("abcdef")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_backslashes_with_newlines()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var");
        write_file("stdin.txt", "abc\\\ndef\\\nghi\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
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
        assert_eq!(Some(String::from("abcdefghi")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_r_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("-r"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("Some")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("line")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_escapes_and_r_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var");
        write_file("stdin.txt", "a\\b\\c\\d\\ef\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("-r"),
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
        assert_eq!(Some(String::from("a\\b\\c\\d\\ef")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_backslashes_with_newlines_and_r_option()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var");
        write_file("stdin.txt", "abc\\\ndef\\\nghi\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("-r"),
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
        assert_eq!(Some(String::from("abc\\")), env.unexported_var("var"));
        assert!(env.exported_var("var").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_other_ifs()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.set_unexported_var("IFS", ":");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "abc:def\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("def")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_ifs_that_is_null()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.set_unexported_var("IFS", "");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "abc def\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc def")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_twice_reads_fields()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        env.unset_var("var3");
        env.unset_var("var4");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let args2 = vec![
            String::from("read"),
            String::from("var3"),
            String::from("var4")
        ];
        let status2 = main(&[], args2.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        assert_eq!(0, status2);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        exec.clear_files();
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("Some")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("line")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
        assert_eq!(Some(String::from("Second")), env.unexported_var("var3"));
        assert!(env.exported_var("var3").is_none());
        assert_eq!(Some(String::from("line")), env.unexported_var("var4"));
        assert!(env.exported_var("var4").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_line_without_newline()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "abc def");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::from("abc")), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::from("def")), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_complains_on_invalid_variable_name()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
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
    fn test_read_builtin_function_complains_on_variable_is_read_only()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var");
        env.set_read_only_var_attr("var");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
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
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_bug_of_eof_detection()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::new()), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::new()), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_read_builtin_function_reads_fields_for_r_option_and_bug_of_eof_detection()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("IFS");
        env.unset_var("var1");
        env.unset_var("var2");
        write_file("stdin.txt", "");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("read"),
            String::from("-r"),
            String::from("var1"),
            String::from("var2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        assert_eq!(Some(String::new()), env.unexported_var("var1"));
        assert!(env.exported_var("var1").is_none());
        assert_eq!(Some(String::new()), env.unexported_var("var2"));
        assert!(env.exported_var("var2").is_none());
    }
}
