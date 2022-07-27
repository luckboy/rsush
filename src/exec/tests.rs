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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
fn test_executor_execute_executes_test_builtin_exit_for_zero_status()
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(11), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    let stdout_content = read_file("stdout.txt");
    assert!(stdout_content.contains(format!("PWD={}\n", current_dir().as_path().to_string_lossy()).as_str()));
    assert_eq!(String::new(), read_file("stderr.txt"));
}

#[sealed_test(before=setup(), after=teardown())]
fn test_executor_execute_executes_rsush_test_exit_for_zero_status()
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(12), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
    }
    match res2 {
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
        Ok(wait_status) => assert_eq!(WaitStatus::Exited(0), wait_status),
        Err(_) => assert!(false),
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
