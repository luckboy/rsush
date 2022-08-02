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
use std::process;
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
fn test_interpreter_interpret_logical_commands_interprets_command()
{
    let s = "./rsush_test args abc def";
    let mut cursor = Cursor::new(s.as_bytes());
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_two_commands()
{
    let s = "
./rsush_test args abc def
./rsush_test args ghi jkl
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
ghi
jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_variables()
{
    let s = "
VAR1=abc
VAR2=def
unset VAR3
./rsush_test args $VAR1 $VAR2 $VAR3
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_arguments()
{
    let s = "
./rsush_test args $0 $1 $2 $12
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
            let args = vec![String::from("abc"), String::from("def")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
rsush
abc
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_at_special_parameter()
{
    let s = "
./rsush_test args $@
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
            let args = vec![String::from("abc"), String::from("def"), String::from("ghi")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_star_special_parameter()
{
    let s = "
./rsush_test args $*
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
            let args = vec![String::from("abc"), String::from("def"), String::from("ghi")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_hash_special_parameter()
{
    let s = "
./rsush_test args $#
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
            let args = vec![String::from("abc"), String::from("def"), String::from("ghi")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
3
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_ques_special_parameter()
{
    let s = "
./rsush_test args $?
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
0
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_minus_special_parameter()
{
    let s = "
./rsush_test args $-
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
mb
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_dolar_special_parameter()
{
    let s = "
./rsush_test args $$
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = format!("{}\n", process::id());
            assert_eq!(expected_stdout_content, read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_excl_special_parameter()
{
    let s = "
./rsush_test args $!
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_minus_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR:-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_minus_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR:-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_minus_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR:-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_minus_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_minus_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_minus_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR-def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_equal_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR:=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_equal_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR:=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_equal_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR:=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_equal_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_equal_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_equal_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR=def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_ques_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR:?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_ques_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR:?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: def\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_ques_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR:?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: def\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_ques_parameter_expansion_for_unset_and_empty_word_vector()
{
    let s = "
unset VAR
./rsush_test args ${VAR:?}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: Parameter null or not set\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_ques_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_ques_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_ques_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR?def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: def\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_ques_parameter_expansion_for_unset_and_empty_word_vector()
{
    let s = "
unset VAR
./rsush_test args ${VAR?}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: Parameter not set\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_plus_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR:+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_plus_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR:+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_colon_plus_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR:+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_plus_parameter_expansion_for_set_and_not_null()
{
    let s = "
VAR=abc
./rsush_test args ${VAR+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
abc
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_plus_parameter_expansion_for_set_but_null()
{
    let s = "
VAR=
./rsush_test args ${VAR+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_plus_parameter_expansion_for_unset()
{
    let s = "
unset VAR
./rsush_test args ${VAR+def}
./rsush_test args $VAR
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_perc_parameter_expansion()
{
    let s = "
VAR=file.c
./rsush_test args ${VAR%.c}
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
file
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_perc_perc_parameter_expansion()
{
    let s = "
VAR=posix/src/std
./rsush_test args ${VAR%%/*}
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
posix
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_hash_parameter_expansion()
{
    let s = "
VAR=/home/luck/src/cmd
./rsush_test args ${VAR#/home/luck}
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
/src/cmd
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_hash_hash_parameter_expansion()
{
    let s = "
VAR=/one/two/three
./rsush_test args ${VAR##*/}
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
three
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_parameter_length_expansion()
{
    let s = "
VAR=abcdef
./rsush_test args ${#VAR}
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
6
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_substitutes_command()
{
    let s = "
./rsush_test args `echo abc def`
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_substitutes_nested_command()
{
    let s = "
./rsush_test args `echo abc \\`echo def\\` ghi`
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
            env.unset_var("IFS");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_number()
{
    let s = "
./rsush_test args $((1234))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
1234
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_parameter()
{
    let s = "
X=1234
./rsush_test args $((X))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
1234
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_parameter_that_is_null()
{
    let s = "
X=
./rsush_test args $((X))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
0
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_parameter_that_is_not_set()
{
    let s = "
unset X
./rsush_test args $((X))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
0
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_unary_operators()
{
    let s = "
./rsush_test args $((-!0))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
-1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_binary_operators()
{
    let s = "
./rsush_test args $((1 * 2 + 4 / 3))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
3
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_assignment_operator()
{
    let s = "
X=1234
./rsush_test args $((X = 2345))
./rsush_test args $X
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
2345
2345
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_binary_operator_with_assignment()
{
    let s = "
X=2
./rsush_test args $((X *= 3))
./rsush_test args $X
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
6
6
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_conditional_operator_with_true()
{
    let s = "
./rsush_test args $((1 ? 2 : 3))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
2
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_for_conditional_operator_with_false()
{
    let s = "
./rsush_test args $((0 ? 2 : 3))
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
3
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}
