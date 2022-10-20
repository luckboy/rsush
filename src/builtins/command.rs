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
use std::fs;
use std::io::*;
use std::path;
use std::path::*;
use getopt;
use getopt::Opt;
use libc;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;
use crate::xcfprintln;
use crate::xsfprintln;

#[derive(Copy, Clone)] 
enum VerboseFlag
{
    None,
    Name,
    Description,
}

struct Options
{
    verbose_flag: VerboseFlag,
}

fn check_prog<P: AsRef<Path>>(path: P) -> Result<()>
{
    match fs::metadata(path.as_ref()) {
        Ok(metadata) => {
            if metadata.file_type().is_dir() {
                Err(Error::from_raw_os_error(libc::EACCES))
            } else {
                match access(path.as_ref(), libc::X_OK) {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(Error::from_raw_os_error(libc::EACCES)),
                    Err(err) => Err(err),
                }
            }
        },
        Err(err) => Err(err),
    }
}

pub fn main(vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "pVv");
    let mut opts = Options {
        verbose_flag: VerboseFlag::None,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('p', _))) => (),
            Some(Ok(Opt('V', _))) => opts.verbose_flag = VerboseFlag::Description,
            Some(Ok(Opt('v', _))) => opts.verbose_flag = VerboseFlag::Name,
            Some(Ok(Opt(c, _))) => {
                xcfprintln!(exec, 2, "unknown option -- {:?}", c);
                return 1;
            },
            Some(Err(err)) => {
                xcfprintln!(exec, 2, "{}", err);
                return 1;
            },
            None => break,
        }
    }
    match opts.verbose_flag {
        VerboseFlag::None => {
            let mut arg_iter = args.iter().skip(opt_parser.index());
            match arg_iter.next() {
                Some(prog) => {
                    let prog_args: Vec<String> = arg_iter.map(|a| a.clone()).collect(); 
                    interp.execute(exec, vars, prog.as_str(), prog_args.as_slice(), false, env, settings, || {
                            let mut s = String::new();
                            s.push_str(singly_quote_str(prog.as_str()).as_str());
                            for prog_arg in &prog_args {
                                s.push(' ');
                                s.push_str(singly_quote_str(prog_arg.as_str()).as_str());
                            }
                            s
                    }).unwrap_or(1)
                },
                None => 0,
            }
        },
        _ => {
            match exec.current_file(1) {
                Some(stdout_file) => {
                    let mut stdout_file_r = stdout_file.borrow_mut();
                    let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                    let mut status = 0;
                    let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
                    for name in &names {
                        match env.builtin_fun(name.as_str()) {
                            Some(_) => {
                                match opts.verbose_flag {
                                    VerboseFlag::Name => fprintln!(&mut line_stdout, "{}", name),
                                    _ => fprintln!(&mut line_stdout, "{} is built-in command", name),
                                }
                            },
                            None => {
                                match env.fun(name.as_str()) {
                                    Some(_) => {
                                        match opts.verbose_flag {
                                            VerboseFlag::Name => fprintln!(&mut line_stdout, "{}", name),
                                            _ => fprintln!(&mut line_stdout, "{} is function", name),
                                        }
                                    },
                                    None => {
                                        match env.alias(name.as_str()) {
                                            Some(value) => {
                                                match opts.verbose_flag {
                                                    VerboseFlag::Name => fprintln!(&mut line_stdout, "alias {}={}", name, singly_quote_str(value.as_str())),
                                                    _ => fprintln!(&mut line_stdout, "{} is alias to {}", name, value),
                                                }
                                            },
                                            None => {
                                                let mut res: Result<PathBuf> = Err(Error::from_raw_os_error(libc::ENOENT));
                                                if name.contains(path::MAIN_SEPARATOR) {
                                                    match check_prog(name) {
                                                        Ok(()) => res = Ok(PathBuf::from(name)),
                                                        Err(err) => res = Err(err),
                                                    }
                                                } else {
                                                    let path = env.var("PATH").unwrap_or(String::from("/bin:/usr/bin"));
                                                    for dir_path in path.split(':') {
                                                        let mut prog_path_buf = PathBuf::from(dir_path);
                                                        prog_path_buf.push(name.as_str());
                                                        match check_prog(prog_path_buf.as_path()) {
                                                            Ok(_) => {
                                                                res = Ok(prog_path_buf);
                                                                break;
                                                            },
                                                            Err(err) => res = Err(err),
                                                        }
                                                    }
                                                }
                                                match res {
                                                    Ok(prog_path_buf) => {
                                                        match opts.verbose_flag {
                                                            VerboseFlag::Name => fprintln!(&mut line_stdout, "{}", prog_path_buf.as_path().to_string_lossy()),
                                                            _ => fprintln!(&mut line_stdout, "{} is {}", name, prog_path_buf.as_path().to_string_lossy()),
                                                        }
                                                    },
                                                    Err(err) => {
                                                        xcfprintln!(exec, 2, "{}: {}", name, err);
                                                        status = 1;
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                        }
                    }
                    status
                },
                None => {
                    xsfprintln!(exec, 2, "No standard output");
                    1
                },
            }
        },
    }
}

#[cfg(test)]
mod tests
{
    use std::cell::*;
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
    fn test_command_builtin_function_executes_command()
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
            String::from("command"),
            String::from("./rsush_test"),
            String::from("args"),
            String::from("abc"),
            String::from("def")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
abc
def
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_executes_command_for_p_option()
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
            String::from("command"),
            String::from("-p"),
            String::from("./rsush_test"),
            String::from("args"),
            String::from("abc"),
            String::from("def")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
abc
def
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_builtin_functions()
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
            String::from("command"),
            String::from("-v"),
            String::from("alias"),
            String::from("break")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
alias
break
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_function()
    {
        let s = "
f() {
    echo abc
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
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("command"),
                    String::from("-v"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                let expected_stdout_content = "
f
";
                assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
            },
            _ => assert!(false),
        }
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_alias()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_alias("alias1", "echo abc");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("command"),
            String::from("-v"),
            String::from("alias1")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
alias alias1='echo abc'
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_program_path()
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
            String::from("command"),
            String::from("-v"),
            String::from("cp")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert!(read_file("stdout.txt").contains("/cp\n"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_program_path_for_program_path()
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
            String::from("command"),
            String::from("-v"),
            String::from("./rsush_test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
./rsush_test
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_builtin_function_descriptions()
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
            String::from("command"),
            String::from("-V"),
            String::from("alias"),
            String::from("break")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
alias is built-in command
break is built-in command
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_function_description()
    {
        let s = "
f() {
    echo abc
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
                write_file("stdin.txt", "Some line\nSecond line\n");
                exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
                exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
                exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
                exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
                let interp_status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
                assert_eq!(0, interp_status);
                let args = vec![
                    String::from("command"),
                    String::from("-V"),
                    String::from("f")
                ];
                let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
                exec.clear_files();
                assert_eq!(0, status);
                assert!(interp.has_none());
                assert_eq!(false, interp.exec_redirect_flag());
                let expected_stdout_content = "
f is function
";
                assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
                assert_eq!(String::new(), read_file("stderr.txt"));
                assert_eq!(String::new(), read_file("stderr2.txt"));
            },
            _ => assert!(false),
        }
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_alias_description()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.set_alias("alias1", "echo abc");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("command"),
            String::from("-V"),
            String::from("alias1")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
alias1 is alias to echo abc
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_program_description()
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
            String::from("command"),
            String::from("-V"),
            String::from("cp")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let stdout_content = read_file("stdout.txt");
        assert!(stdout_content.starts_with("cp is "));
        assert!(stdout_content.contains("/cp\n"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_prints_program_description_for_program_path()
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
            String::from("command"),
            String::from("-V"),
            String::from("./rsush_test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        let expected_stdout_content = "
./rsush_test is ./rsush_test
";
        assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_complains_on_not_found_for_name()
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
            String::from("command"),
            String::from("-v"),
            String::from("./xxx")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert!(read_file("stderr2.txt").starts_with("./xxx: "));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_command_builtin_function_complains_on_not_found_for_description()
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
            String::from("command"),
            String::from("-V"),
            String::from("./xxx")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert!(read_file("stderr2.txt").starts_with("./xxx: "));
    }
}
