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
fn test_executor_execute_executes_test_builtin_args()
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
    let args = vec![String::from("abc"), String::from("def"), String::from("ghi")];
    let res = exec.execute(&mut interp, &[], "test_builtin_args", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let expected_stdout_content = "
abc
def
ghi
";
    assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_test_builtin_args()
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
    let args = vec![String::from("abc"), String::from("def")];
    let res = exec.execute(&mut interp, &[], "test_builtin_args", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("ghi"), String::from("jkl")];
    let res2 = exec.execute(&mut interp, &[], "test_builtin_args", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let expected_stdout_content = "
abc
def
ghi
jkl
";
    assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_test_builtin_env()
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
    let res = exec.execute(&mut interp, &[], "test_builtin_env", &[], false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let stdout_content = read_file("stdout.txt");
    assert!(stdout_content.contains(format!("PWD={}\n", current_dir().as_path().to_string_lossy()).as_str()));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_test_builtin_vars()
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
    let vars = vec![
        (String::from("var1"), String::from("abc")),
        (String::from("var2"), String::from("def")), 
        (String::from("var3"), String::from("ghi"))
    ];
    let res = exec.execute(&mut interp, vars.as_slice(), "test_builtin_vars", &[], false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let expected_stdout_content = "
var1=abc
var2=def
var3=ghi
";
    assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_test_builtin_exit_for_status_zero()
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
    let args = vec![String::from("0")];
    let res = exec.execute(&mut interp, &[], "test_builtin_exit", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_test_builtin_exit_for_other_status()
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
    let args = vec![String::from("11")];
    let res = exec.execute(&mut interp, &[], "test_builtin_exit", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(11), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_args()
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
    let args = vec![String::from("args"), String::from("abc"), String::from("def"), String::from("ghi")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let expected_stdout_content = "
abc
def
ghi
";
    assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_rsush_test_args()
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
    let args = vec![String::from("args"), String::from("abc"), String::from("def")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("args"), String::from("ghi"), String::from("jkl")];
    let res2 = exec.execute(&mut interp, &[], "./rsush_test", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let expected_stdout_content = "
abc
def
ghi
jkl
";
    assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_env()
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
    let args = vec![String::from("env")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let stdout_content = read_file("stdout.txt");
    assert!(stdout_content.contains(format!("PWD={}\n", current_dir().as_path().to_string_lossy()).as_str()));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_exit_for_status_zero()
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
    let args = vec![String::from("exit"), String::from("0")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_exit_for_other_status()
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
    let args = vec![String::from("exit"), String::from("12")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(12), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_read_fd_for_stdin()
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
    let args = vec![String::from("read_fd"), String::from("0"), String::from("10")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("Some line\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_rsush_test_read_fd_for_stdin()
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
    let args = vec![String::from("read_fd"), String::from("0"), String::from("10")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("read_fd"), String::from("0"), String::from("12")];
    let res2 = exec.execute(&mut interp, &[], "./rsush_test", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("Some line\n\nSecond line\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_read_fd_for_other_fd()
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
    write_file("4.txt", "abcdef\nghijkl\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    let args = vec![String::from("read_fd"), String::from("4"), String::from("7")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("abcdef\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_rsush_test_read_fd_for_other_fd()
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
    write_file("4.txt", "abcdef\nghijkl\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    let args = vec![String::from("read_fd"), String::from("4"), String::from("7")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("read_fd"), String::from("4"), String::from("7")];
    let res2 = exec.execute(&mut interp, &[], "./rsush_test", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("abcdef\n\nghijkl\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_write_fd_for_stderr()
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
    let args = vec![String::from("write_fd"), String::from("2"), String::from("abc"), String::from("def"), String::from("ghi")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    let expected_stderr_content = "
abc
def
ghi
";
    assert_eq!(String::from(&expected_stderr_content[1..]), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_rsush_test_write_fd_for_stderr()
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
    let args = vec![String::from("write_fd"), String::from("2"), String::from("abc"), String::from("def")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("write_fd"), String::from("2"), String::from("ghi"), String::from("jkl")];
    let res2 = exec.execute(&mut interp, &[], "./rsush_test", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    let expected_stderr_content = "
abc
def
ghi
jkl
";
    assert_eq!(String::from(&expected_stderr_content[1..]), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_write_fd_for_other_fd()
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
    exec.push_file(4, Rc::new(RefCell::new(create_file("4.txt"))));
    let args = vec![String::from("write_fd"), String::from("4"), String::from("abc"), String::from("def"), String::from("ghi")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
    let expected_file_content = "
abc
def
ghi
";
    assert_eq!(String::from(&expected_file_content[1..]), read_file("4.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_rsush_test_args_for_other_fd()
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
    exec.push_file(4, Rc::new(RefCell::new(create_file("4.txt"))));
    let args = vec![String::from("write_fd"), String::from("4"), String::from("abc"), String::from("def")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    let args2 = vec![String::from("write_fd"), String::from("4"), String::from("ghi"), String::from("jkl")];
    let res2 = exec.execute(&mut interp, &[], "./rsush_test", args2.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    match res2 {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
    let expected_file_content = "
abc
def
ghi
jkl
";
    assert_eq!(String::from(&expected_file_content[1..]), read_file("4.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_read_2fds_for_other_fds()
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
    write_file("4.txt", "abcdef\n");
    write_file("5.txt", "ghijkl\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(5, Rc::new(RefCell::new(open_file("5.txt"))));
    let args = vec![String::from("read_2fds"), String::from("4"), String::from("7"), String::from("5"), String::from("7")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("abcdef\n\nghijkl\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_write_2fds_for_other_fd()
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
    exec.push_file(4, Rc::new(RefCell::new(create_file("4.txt"))));
    exec.push_file(5, Rc::new(RefCell::new(create_file("5.txt"))));
    let args = vec![String::from("write_2fds"), String::from("4"), String::from("5"), String::from("2"), String::from("abc"), String::from("def"), String::from("ghi"), String::from("jkl")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
    let expected_file_content = "
abc
def
";
    assert_eq!(String::from(&expected_file_content[1..]), read_file("4.txt"));
    let expected_file_content2 = "
ghi
jkl
";
    assert_eq!(String::from(&expected_file_content2[1..]), read_file("5.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_sets_variables()
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
    let vars = vec![
        (String::from("VAR1"), String::from("abc")),
        (String::from("VAR2"), String::from("def")),
        (String::from("VAR3"), String::from("ghi"))
    ];
    let args = vec![String::from("env")];
    let res = exec.execute(&mut interp, vars.as_slice(), "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    let stdout_content = read_file("stdout.txt");
    assert!(stdout_content.contains(format!("PWD={}\n", current_dir().as_path().to_string_lossy()).as_str()));
    assert!(stdout_content.contains("VAR1=abc\n"));
    assert!(stdout_content.contains("VAR2=def\n"));
    assert!(stdout_content.contains("VAR3=ghi\n"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_duplicates_file_for_reading()
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
    write_file("4.txt", "abcdef\nghijkl\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let file = Rc::new(RefCell::new(open_file("4.txt")));
    exec.push_file(4, file.clone());
    exec.push_file(5, file);
    let args = vec![String::from("read_2fds"), String::from("4"), String::from("7"), String::from("5"), String::from("7")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::from("abcdef\n\nghijkl\n\n"), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_duplicates_file_for_writing()
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
    let file = Rc::new(RefCell::new(create_file("4.txt")));
    exec.push_file(4, file.clone());
    exec.push_file(5, file);
    let args = vec![String::from("write_2fds"), String::from("4"), String::from("5"), String::from("2"), String::from("abc"), String::from("def"), String::from("ghi"), String::from("jkl")];
    let res = exec.execute(&mut interp, &[], "./rsush_test", args.as_slice(), false, &mut env, &mut settings, |_| true);
    exec.clear_files();
    match res {
        Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
        _ => assert!(false),
    }
    assert_eq!(String::new(), read_file("stdout.txt"));
    assert_eq!(String::new(), read_file("stderr.txt"));
    let expected_file_content = "
abc
def
ghi
jkl
";
    assert_eq!(String::from(&expected_file_content[1..]), read_file("4.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_push_file_pushes_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_push_file_twice_pushes_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_push_file_thrice_pushes_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    write_file("4_3.txt", "mnopqr\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_3.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("mnopqr\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_pop_file_pops_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_pop_file_twice_pops_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    write_file("4_3.txt", "mnopqr\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_3.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("mnopqr\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_pop_file_thrice_pops_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    write_file("4_3.txt", "mnopqr\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_3.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("mnopqr\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    assert_eq!(true, exec.current_file(4).is_none());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_pop_penultimate_file_pops_penultimate_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    write_file("4_3.txt", "mnopqr\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_3.txt"))));
    exec.pop_penultimate_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("mnopqr\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.pop_file(4);
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_push_file_and_set_saved_file_pushes_file_and_sets_saved_file()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("4_2.txt", "ghijkl\n");
    exec.push_file_and_set_saved_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(open_file("4_2.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    match exec.saved_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_push_file_clear_files()
{
    let mut exec = Executor::new();
    write_file("4.txt", "abcdef\n");
    write_file("5.txt", "ghijkl\n");
    write_file("6.txt", "mnopqr\n");
    exec.push_file(4, Rc::new(RefCell::new(open_file("4.txt"))));
    exec.push_file(5, Rc::new(RefCell::new(open_file("5.txt"))));
    exec.push_file(6, Rc::new(RefCell::new(open_file("6.txt"))));
    match exec.current_file(4) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("abcdef\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    match exec.current_file(5) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("ghijkl\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    match exec.current_file(6) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            assert_eq!(String::from("mnopqr\n"), read_stream(&mut *file_r));
        },
        _ => assert!(false),
    }
    exec.clear_files();
    assert_eq!(true, exec.current_file(4).is_none());
    assert_eq!(true, exec.current_file(5).is_none());
    assert_eq!(true, exec.current_file(6).is_none());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_set_pipes_sets_pipes()
{
    let mut exec = Executor::new();
    write_file("4_r.txt", "abcdef\n");
    write_file("5_r.txt", "ghijkl\n");
    let pipes = vec![
        Pipe::new(Rc::new(RefCell::new(open_file("4_r.txt"))), Rc::new(RefCell::new(create_file("4_w.txt")))),
        Pipe::new(Rc::new(RefCell::new(open_file("5_r.txt"))), Rc::new(RefCell::new(create_file("5_w.txt"))))
    ];
    exec.set_pipes(pipes);
    let mut reading_file_r1 = exec.pipes()[0].reading_file.borrow_mut();
    assert_eq!(String::from("abcdef\n"), read_stream(&mut *reading_file_r1));
    let mut writing_file_r1 = exec.pipes()[0].writing_file.borrow_mut();
    write_stream(&mut *writing_file_r1, "abcdef2\n");
    assert_eq!(String::from("abcdef2\n"), read_file("4_w.txt"));
    let mut reading_file_r2 = exec.pipes()[1].reading_file.borrow_mut();
    assert_eq!(String::from("ghijkl\n"), read_stream(&mut *reading_file_r2));
    let mut writing_file_r2 = exec.pipes()[1].writing_file.borrow_mut();
    write_stream(&mut *writing_file_r2, "ghijkl2\n");
    assert_eq!(String::from("ghijkl2\n"), read_file("5_w.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_clear_pipes_clears_pipes()
{
    let mut exec = Executor::new();
    write_file("4_r.txt", "abcdef\n");
    write_file("5_r.txt", "ghijkl\n");
    let pipes = vec![
        Pipe::new(Rc::new(RefCell::new(open_file("4_r.txt"))), Rc::new(RefCell::new(create_file("4_w.txt")))),
        Pipe::new(Rc::new(RefCell::new(open_file("5_r.txt"))), Rc::new(RefCell::new(create_file("5_w.txt"))))
    ];
    exec.set_pipes(pipes);
    {
        let mut reading_file_r1 = exec.pipes()[0].reading_file.borrow_mut();
        assert_eq!(String::from("abcdef\n"), read_stream(&mut *reading_file_r1));
        let mut writing_file_r1 = exec.pipes()[0].writing_file.borrow_mut();
        write_stream(&mut *writing_file_r1, "abcdef2\n");
        assert_eq!(String::from("abcdef2\n"), read_file("4_w.txt"));
        let mut reading_file_r2 = exec.pipes()[1].reading_file.borrow_mut();
        assert_eq!(String::from("ghijkl\n"), read_stream(&mut *reading_file_r2));
        let mut writing_file_r2 = exec.pipes()[1].writing_file.borrow_mut();
        write_stream(&mut *writing_file_r2, "ghijkl2\n");
        assert_eq!(String::from("ghijkl2\n"), read_file("5_w.txt"));
    }
    exec.clear_pipes();
    assert_eq!(true, exec.pipes().is_empty());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_shell_pid_returns_pid()
{
    let exec = Executor::new();
    assert_eq!(process::id() as i32, exec.shell_pid());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_adds_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_twice_adds_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(2), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&2) {
        Some(job) => {
            assert_eq!(2345, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test2"), job.name);
            assert_eq!(Some(1), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_thirce_adds_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(2), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&2) {
        Some(job) => {
            assert_eq!(2345, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test2"), job.name);
            assert_eq!(Some(1), job.prev_job_id);
            assert_eq!(Some(3), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&3) {
        Some(job) => {
            assert_eq!(3456, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.name);
            assert_eq!(Some(2), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_remove_job_removes_first_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    exec.remove_job(1);
    match exec.jobs().get(&2) {
        Some(job) => {
            assert_eq!(2345, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test2"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(3), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&3) {
        Some(job) => {
            assert_eq!(3456, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.name);
            assert_eq!(Some(2), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_remove_job_removes_center_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    exec.remove_job(2);
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(3), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&3) {
        Some(job) => {
            assert_eq!(3456, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.name);
            assert_eq!(Some(1), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_remove_job_removes_last_job()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    exec.remove_job(3);
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(2), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&2) {
        Some(job) => {
            assert_eq!(2345, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test2"), job.name);
            assert_eq!(Some(1), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_adds_job_after_job_removing()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    exec.remove_job(2);
    match exec.add_job(&Job::new(4567, "test4")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(Some(3), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&3) {
        Some(job) => {
            assert_eq!(3456, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.name);
            assert_eq!(Some(1), job.prev_job_id);
            assert_eq!(Some(2), job.next_job_id);
        },
        None => assert!(false),
    }
    match exec.jobs().get(&2) {
        Some(job) => {
            assert_eq!(4567, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test4"), job.name);
            assert_eq!(Some(3), job.prev_job_id);
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_current_job_id_returns_current_job_id()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    assert_eq!(Some(3), exec.current_job_id());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_current_job_id_does_not_return_current_job_id_after_job_removing()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    exec.remove_job(1);
    assert_eq!(true, exec.current_job_id().is_none());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_current_job_id_does_not_return_current_job_id_for_empty_jobs()
{
    let exec = Executor::new();
    assert_eq!(true, exec.current_job_id().is_none());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_prev_current_job_id_returns_previous_current_job_id()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(2345, "test2")) {
        Some(job_id) => assert_eq!(job_id, 2),
        _ => assert!(false),
    }
    match exec.add_job(&Job::new(3456, "test3")) {
        Some(job_id) => assert_eq!(job_id, 3),
        _ => assert!(false),
    }
    assert_eq!(Some(2), exec.prev_current_job_id());
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_set_job_last_status_sets_job_last_status()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    exec.set_job_last_status(1, WaitStatus::Signaled(9, false));
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::Signaled(9, false), job.last_status);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_adds_job_without_pids_and_process_names()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new(1234, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(Vec::<i32>::new(), job.pids);
            assert_eq!(Vec::<WaitStatus>::new(), job.statuses);
            assert_eq!(Vec::<String>::new(), job.process_names);
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(1234, job.pgid);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(false, job.show_flag);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_add_job_adds_job_with_pids_and_process_names()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new_with_pids_and_process_names(vec![2345, 3456], vec![String::from("test1"), String::from("test2")], 1234, "test3", 4567, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(vec![2345, 3456], job.pids);
            let expected_statuses = vec![
                WaitStatus::None,
                WaitStatus::None
            ];
            assert_eq!(expected_statuses, job.statuses);
            let expected_process_names = vec![
                String::from("test1"),
                String::from("test2")
            ];
            assert_eq!(expected_process_names, job.process_names);
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.last_process_name);
            assert_eq!(4567, job.pgid);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(false, job.show_flag);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_set_job_status_sets_job_status()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new_with_pids_and_process_names(vec![2345, 3456], vec![String::from("test1"), String::from("test2")], 1234, "test3", 4567, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    exec.set_job_status(1, 0, WaitStatus::Signaled(9, false));
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(vec![2345, 3456], job.pids);
            let expected_statuses = vec![
                WaitStatus::Signaled(9, false),
                WaitStatus::None
            ];
            assert_eq!(expected_statuses, job.statuses);
            let expected_process_names = vec![
                String::from("test1"),
                String::from("test2")
            ];
            assert_eq!(expected_process_names, job.process_names);
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.last_process_name);
            assert_eq!(4567, job.pgid);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(false, job.show_flag);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_set_job_statuses_sets_job_statuses()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new_with_pids_and_process_names(vec![2345, 3456], vec![String::from("test1"), String::from("test2")], 1234, "test3", 4567, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    exec.set_job_statuses(1, vec![WaitStatus::Signaled(9, false), WaitStatus::Signaled(2, false)]);
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(vec![2345, 3456], job.pids);
            let expected_statuses = vec![
                WaitStatus::Signaled(9, false),
                WaitStatus::Signaled(2, false)
            ];
            assert_eq!(expected_statuses, job.statuses);
            let expected_process_names = vec![
                String::from("test1"),
                String::from("test2")
            ];
            assert_eq!(expected_process_names, job.process_names);
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.last_process_name);
            assert_eq!(4567, job.pgid);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(false, job.show_flag);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_set_job_show_flag_sets_job_show_flag()
{
    let mut exec = Executor::new();
    match exec.add_job(&Job::new_with_pids_and_process_names(vec![2345, 3456], vec![String::from("test1"), String::from("test2")], 1234, "test3", 4567, "test")) {
        Some(job_id) => assert_eq!(job_id, 1),
        _ => assert!(false),
    }
    exec.set_job_show_flag(1, true);
    match exec.jobs().get(&1) {
        Some(job) => {
            assert_eq!(vec![2345, 3456], job.pids);
            let expected_statuses = vec![
                WaitStatus::None,
                WaitStatus::None
            ];
            assert_eq!(expected_statuses, job.statuses);
            let expected_process_names = vec![
                String::from("test1"),
                String::from("test2")
            ];
            assert_eq!(expected_process_names, job.process_names);
            assert_eq!(1234, job.last_pid);
            assert_eq!(WaitStatus::None, job.last_status);
            assert_eq!(String::from("test3"), job.last_process_name);
            assert_eq!(4567, job.pgid);
            assert_eq!(String::from("test"), job.name);
            assert_eq!(true, job.show_flag);
            assert_eq!(true, job.prev_job_id.is_none());
            assert_eq!(true, job.next_job_id.is_none());
        },
        None => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_interpret_sets_in_interpreter_state()
{
    let mut exec = Executor::new();
    let mut is_in_interp = false;
    let mut is_in_interp2 = false;
    let mut state_stack_len = 0;
    exec.interpret(|exec| {
            is_in_interp = exec.current_state == State::InInterpreter;
            is_in_interp2 = exec.state_stack[0] == State::InInterpreter;
            state_stack_len = exec.state_stack.len();
    });
    assert_eq!(true, is_in_interp);
    assert_eq!(true, is_in_interp2);
    assert_eq!(1, state_stack_len);
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_interpret_twice_sets_in_interpreter_state()
{
    let mut exec = Executor::new();
    let mut is_in_interp = false;
    let mut is_in_interp2 = false;
    let mut is_in_interp3 = false;
    let mut state_stack_len = 0;
    exec.interpret(|exec| {
            exec.interpret(|exec| {
                    is_in_interp = exec.current_state == State::InInterpreter;
                    is_in_interp2 = exec.state_stack[0] == State::InInterpreter;
                    is_in_interp3 = exec.state_stack[1] == State::InInterpreter;
                    state_stack_len = exec.state_stack.len();
            });
    });
    assert_eq!(true, is_in_interp);
    assert_eq!(true, is_in_interp2);
    assert_eq!(true, is_in_interp3);
    assert_eq!(2, state_stack_len);
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_interpret_or_sets_in_interpreter_state()
{
    let mut exec = Executor::new();
    let mut is_in_interp = false;
    let mut is_in_interp2 = false;
    let mut state_stack_len = 0;
    exec.interpret_or(true, |exec| {
            is_in_interp = exec.current_state == State::InInterpreter;
            is_in_interp2 = exec.state_stack[0] == State::InInterpreter;
            state_stack_len = exec.state_stack.len();
    });
    assert_eq!(true, is_in_interp);
    assert_eq!(true, is_in_interp2);
    assert_eq!(1, state_stack_len);
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_interpret_or_does_not_set_in_interpreter_state()
{
    let mut exec = Executor::new();
    let mut is_in_interp = false;
    let mut state_stack_len = 0;
    exec.interpret_or(false, |exec| {
            is_in_interp = exec.current_state == State::InInterpreter;
            state_stack_len = exec.state_stack.len();
    });
    assert_eq!(true, is_in_interp);
    assert_eq!(0, state_stack_len);
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_creates_process()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let res = exec.create_process(false, &mut settings, |exec, _| {
            match exec.current_file(1) {
                Some(file) => {
                    let mut file_r = file.borrow_mut();
                    write_stream(&mut *file_r, format!("{:?}\n", process::id() as i32).as_str());
                    write_stream(&mut *file_r, format!("{:?}\n", exec.current_state).as_str());
                    write_stream(&mut *file_r, format!("{:?}\n", exec.state_stack.len()).as_str());
                },
                None => (),
            }
            0
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(0)) => {
                    let expected_stdout_content = format!("{:?}\n{:?}\n{:?}\n", pid, State::InNewProcess, 1);
                    assert_eq!(expected_stdout_content, read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_does_not_create_process_for_in_new_process()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let res = exec.create_process(false, &mut settings, |exec, settings| {
            let res = exec.create_process(false, settings, |exec, _| {
                    match exec.current_file(1) {
                        Some(file) => {
                            let mut file_r = file.borrow_mut();
                            write_stream(&mut *file_r, format!("{:?}\n", process::id() as i32).as_str());
                            write_stream(&mut *file_r, format!("{:?}\n", exec.current_state).as_str());
                            write_stream(&mut *file_r, format!("{:?}\n", exec.state_stack.len()).as_str());
                        },
                        None => (),
                    }
                    0
            });
            match res {
                Ok(pid) => {
                    let res2 = exec.wait_for_process(pid, true, false, true, settings);
                    match res2 {
                        Ok(WaitStatus::Exited(status)) => status,
                        _ => 1,
                    }
                },
                Err(_) => 1,
            }
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(0)) => {
                    let expected_stdout_content = format!("{:?}\n{:?}\n{:?}\n", pid, State::InNewProcess, 2);
                    assert_eq!(expected_stdout_content, read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_creates_process_for_background()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let res = exec.create_process(true, &mut settings, |exec, _| {
            match exec.current_file(1) {
                Some(file) => {
                    let mut file_r = file.borrow_mut();
                    write_stream(&mut *file_r, format!("{:?}\n", process::id() as i32).as_str());
                    write_stream(&mut *file_r, format!("{:?}\n", exec.current_state).as_str());
                    write_stream(&mut *file_r, format!("{:?}\n", exec.state_stack.len()).as_str());
                },
                None => (),
            }
            0
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(0)) => {
                    let expected_stdout_content = format!("{:?}\n{:?}\n{:?}\n", pid, State::InNewProcess, 2);
                    assert_eq!(expected_stdout_content, read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_creates_process_for_in_new_process_and_background()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    exec.push_file(4, Rc::new(RefCell::new(create_file("4.txt"))));
    let res = exec.create_process(false, &mut settings, |exec, settings| {
            let res = exec.create_process(true, settings, |exec, _| {
                    match exec.current_file(1) {
                        Some(file) => {
                            let mut file_r = file.borrow_mut();
                            write_stream(&mut *file_r, format!("{:?}\n", process::id() as i32).as_str());
                            write_stream(&mut *file_r, format!("{:?}\n", exec.current_state).as_str());
                            write_stream(&mut *file_r, format!("{:?}\n", exec.state_stack.len()).as_str());
                        },
                        None => (),
                    }
                    0
            });
            match res {
                Ok(Some(pid)) => {
                    match exec.current_file(4) {
                        Some(file) => {
                            let mut file_r = file.borrow_mut();
                            write_stream(&mut *file_r, format!("{}\n", pid).as_str());
                        },
                        None => (),
                    }
                    let res2 = exec.wait_for_process(Some(pid), true, false, true, settings);
                    match res2 {
                        Ok(WaitStatus::Exited(status)) => status,
                        _ => 1,
                    }
                },
                Ok(None) => {
                    let res2 = exec.wait_for_process(None, true, false, true, settings);
                    match res2 {
                        Ok(WaitStatus::Exited(status)) => status,
                        _ => 1,
                    }
                },
                Err(_) => 1,
            }
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(0)) => {
                    let expected_stdout_content = format!("{}\n{:?}\n{:?}\n", read_file("4.txt").trim(), State::InNewProcess, 3);
                    assert_eq!(expected_stdout_content, read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_creates_process_for_status_zero()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let res = exec.create_process(false, &mut settings, |_, _| {
            0
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(status)) => {
                    assert_eq!(0, status);
                    assert_eq!(String::new(), read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_create_process_creates_process_for_other_status()
{
    let mut exec = Executor::new();
    let mut settings = Settings::new();
    settings.arg0 = String::from("rsush");
    write_file("stdin.txt", "Some line\nSecond line\n");
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
    let res = exec.create_process(false, &mut settings, |_, _| {
            12
    });
    match res {
        Ok(Some(pid)) => {
            let res2 = exec.wait_for_process(Some(pid), true, false, true, &mut settings);
            match res2 {
                Ok(WaitStatus::Exited(status)) => {
                    assert_eq!(12, status);
                    assert_eq!(String::new(), read_file("stdout.txt"));
                    assert_eq!(String::new(), read_file("stderr.txt"));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    exec.clear_files();
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_function()
{
    let s = "
f() {
    echo abc $*
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let args = vec![String::from("def"), String::from("ghi")];
            let res = exec.execute(&mut interp, &[], "f", args.as_slice(), false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            let expected_stdout_content = "
abc def ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_function()
{
    let s = "
f() {
    echo abc $*
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let args = vec![String::from("def"), String::from("ghi")];
            let res = exec.execute(&mut interp, &[], "f", args.as_slice(), false, &mut env, &mut settings, |_| true);
            let args2 = vec![String::from("jkl"), String::from("mno")];
            let res2 = exec.execute(&mut interp, &[], "f", args2.as_slice(), false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            match res2 {
                Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            let expected_stdout_content = "
abc def ghi
abc jkl mno
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_function_for_settings_of_variables()
{
    let s = "
f() {
    echo abc $var1 $var2
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let vars = vec![
                (String::from("var1"), String::from("def")),
                (String::from("var2"), String::from("ghi"))
            ];
            let res = exec.execute(&mut interp, vars.as_slice(), "f", &[], false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            let expected_stdout_content = "
abc def ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_twice_executes_function_for_settings_of_variables()
{
    let s = "
f() {
    echo abc $var1 $var2
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let vars = vec![
                (String::from("var1"), String::from("def")),
                (String::from("var2"), String::from("ghi"))
            ];
            let res = exec.execute(&mut interp, vars.as_slice(), "f", &[], false, &mut env, &mut settings, |_| true);
            let vars2 = vec![
                (String::from("var1"), String::from("jkl")),
                (String::from("var2"), String::from("mno"))
            ];
            let res2 = exec.execute(&mut interp, vars2.as_slice(), "f", &[], false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            match res2 {
                Ok((wait_status, Some(_))) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            let expected_stdout_content = "
abc def ghi
abc jkl mno
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_function_with_return_value_true()
{
    let s = "
f() {
    true
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let res = exec.execute(&mut interp, &[], "f", &[], false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(0), wait_status),
                _ => assert!(false),
            }
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_function_with_return_value_false()
{
    let s = "
f() {
    false
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            assert_eq!(0, status);
            let res = exec.execute(&mut interp, &[], "f", &[], false, &mut env, &mut settings, |_| true);
            exec.clear_files();
            match res {
                Ok((wait_status, None)) => assert_eq!(WaitStatus::Exited(1), wait_status),
                _ => assert!(false),
            }
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
        },
        _ => assert!(false),
    }
}
