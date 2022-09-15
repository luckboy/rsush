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
unset VAR1
VAR1=abc
unset VAR2
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset VAR
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
unset X
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
unset X
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
unset X
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
unset X
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

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_string_in_double_quotation()
{
    let s = "
./rsush_test args \"abc\"
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_variables_in_double_quotations()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
unset VAR3
./rsush_test args \"$VAR1\" \"$VAR2\" \"$VAR3\"
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
fn test_interpreter_interpret_logical_commands_performs_parameter_expansion_in_double_quotation()
{
    let s = "
unset VAR
VAR=abc
./rsush_test args \"${VAR:-def}\"
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_parameter_length_expansion_in_double_quotation()
{
    let s = "
unset VAR
VAR=abcdef
./rsush_test args \"${#VAR}\"
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
fn test_interpreter_interpret_logical_commands_substitutes_command_in_double_quotation()
{
    let s = "
./rsush_test args \"`echo abc def ghi`\"
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
abc def ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_arithmetic_expansion_in_double_quotation()
{
    let s = "
./rsush_test args \"$((1 + 2))\"
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
fn test_interpreter_interpret_logical_commands_takes_string_in_single_quotation()
{
    let s = "
./rsush_test args 'abc'
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_takes_empty_string_in_double_quotation()
{
    let s = "
./rsush_test args \"\"
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
fn test_interpreter_interpret_logical_commands_takes_empty_string_in_single_quotation()
{
    let s = "
./rsush_test args ''
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
fn test_interpreter_interpret_logical_commands_performs_expansions()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
./rsush_test args abc$VAR1${VAR2}ghi
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
abcabcdefghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_for_string_with_space()
{
    let s = "
./rsush_test args abc\\ def
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
abc def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_for_variable_with_spaces()
{
    let s = "
unset VAR
VAR='  abc  '
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_for_variable_with_only_spaces()
{
    let s = "
unset VAR
VAR='  '
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
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_without_spaces_and_first_string()
{
    let s = "
unset VAR
VAR=abcdef
./rsush_test args ghi$VAR
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
ghiabcdef
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_without_spaces_and_last_string()
{
    let s = "
unset VAR
VAR=abcdef
./rsush_test args ${VAR}ghi
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
abcdefghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_with_spaces_and_first_string()
{
    let s = "
unset VAR
VAR='  abc def  '
./rsush_test args ghi$VAR
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
ghi
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
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_with_spaces_and_last_string()
{
    let s = "
unset VAR
VAR='  abc def  '
./rsush_test args ${VAR}ghi
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
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_with_spaces_and_first_string_and_lasst_string_and_ifs_that_is_null()
{
    let s = "
unset VAR
VAR='  abc def  '
./rsush_test args abc${VAR}ghi
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
            env.set_unexported_var("IFS", "");
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
abc  abc def  ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_variable_with_spaces_and_first_string_and_lasst_string_and_other_ifs()
{
    let s = "
unset VAR
VAR='abc:def'
./rsush_test args abc${VAR}ghi
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
            env.set_unexported_var("IFS", ":");
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
abcabc
defghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_arguments_with_spaces_and_first_string_and_last_string()
{
    let s = "
./rsush_test args ghi$*jkl
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
            let args = vec![String::from("  abc  "), String::from("def def"), String::from("  ghi  ")];
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
ghi
abc
def
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
fn test_interpreter_interpret_logical_commands_performs_expansions_for_arguments_with_spaces_and_first_string_and_last_string_and_ifs_that_is_null()
{
    let s = "
./rsush_test args ghi$*jkl
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
            let args = vec![String::from("  abc  "), String::from("def def"), String::from("  ghi  ")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("IFS");
            env.set_unexported_var("IFS", "");
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
ghi  abc  
def def
  ghi  jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_arguments_with_spaces_and_first_string_and_last_string_and_other_ifs()
{
    let s = "
./rsush_test args ghi$*jkl
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
            let args = vec![String::from("abc"), String::from("def:def"), String::from("ghi")];
            settings.current_args_mut().set_args(args);
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("IFS");
            env.set_unexported_var("IFS", ":");
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
ghiabc
def
def
ghijkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_expansion_words_with_spaces_and_first_string_and_last_string()
{
    let s = "
unset VAR
./rsush_test args ghi${VAR:-\" abc \" \" def \"}jkl
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
            let args = vec![String::from("abc"), String::from("def:def"), String::from("ghi")];
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
ghi abc 
 def jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_for_expansions_in_double_quotation_and_first_string_and_last_string()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
./rsush_test args ghi\"$VAR1$VAR2\"jkl
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
ghiabcdefjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_in_double_quotation()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
./rsush_test args \"ghi$VAR1${VAR2}jkl\"
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
ghiabcdefjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_at_arguments()
{
    let s = "
./rsush_test args \"$@\"
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
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_empty_at_arguments()
{
    let s = "
./rsush_test args \"$@\"
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
            settings.current_args_mut().set_args(Vec::new());
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
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_at_arguments_with_spaces()
{
    let s = "
./rsush_test args \"$@\"
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
            let args = vec![String::from("abc abc"), String::from("def"), String::from("ghi ghi")];
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
abc abc
def
ghi ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_in_double_quotation_for_at_arguments_and_first_string_and_last_string()
{
    let s = "
./rsush_test args \"ghi$@jkl\"
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
ghiabc
defjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_star_arguments()
{
    let s = "
./rsush_test args \"$*\"
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
abc def ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_empty_star_arguments()
{
    let s = "
./rsush_test args \"$*\"
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
            settings.current_args_mut().set_args(Vec::new());
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
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_star_arguments_with_spaces()
{
    let s = "
./rsush_test args \"$*\"
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
            let args = vec![String::from("abc abc"), String::from("def"), String::from("ghi ghi")];
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
abc abc def ghi ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_in_double_quotation_for_star_arguments_and_first_string_and_last_string()
{
    let s = "
./rsush_test args \"ghi$*jkl\"
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
ghiabc defjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_in_double_quotation_for_star_arguments_and_first_string_and_last_string_and_ifs_that_is_null()
{
    let s = "
./rsush_test args \"ghi$*jkl\"
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
            env.set_unexported_var("IFS", "");
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
ghiabcdefjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansions_in_double_quotation_for_star_arguments_and_first_string_and_last_string_and_other_ifs()
{
    let s = "
./rsush_test args \"ghi$*jkl\"
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
            env.set_unexported_var("IFS", ":");
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
ghiabc:defjkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_expansion_words()
{
    let s = "
unset VAR
./rsush_test args \"${VAR:-abc def}\"
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
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_expansion_words_with_spaces()
{
    let s = "
unset VAR
./rsush_test args \"${VAR:-\"abc def\" \"ghi jkl\"}\"
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
abc def
ghi jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_expansion_in_double_quotation_for_expansion_words_with_minuses()
{
    let s = "
unset VAR
./rsush_test args \"${VAR:-\"abc-def\" \"ghi-jkl\"}\"
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
abc-def
ghi-jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_tylde_expansion()
{
    let s = "
./rsush_test args ~
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
            env.unset_var("HOME");
            env.set_exported_var("HOME", "/home/luck");
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
/home/luck
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_tylde_expansion_for_tylda_with_path()
{
    let s = "
./rsush_test args ~/test
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
            env.unset_var("HOME");
            env.set_exported_var("HOME", "/home/luck");
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
/home/luck/test
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_glob_expansion()
{
    let s = "
./rsush_test args test/*/*
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            write_file("test/xxx/yyy", "yyy\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/zzz", "zzz\n");
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
test/xxx/xxx
test/xxx/yyy
test/yyy/zzz
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_two_glob_expansions()
{
    let s = "
./rsush_test args test/*/* test2/*
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/xxx/xxx
test/yyy/yyy
test/yyy/zzz
test2/aaa
test2/bbb
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_glob_expansion_after_variable_expansion()
{
    let s = "
unset VAR
VAR='*/*'
./rsush_test args test/$VAR
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
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
test/xxx/xxx
test/yyy/yyy
test/yyy/zzz
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_glob_expansion_for_variable()
{
    let s = "
unset VAR
VAR='test/*/*'
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            write_file("test/xxx/yyy", "yyy\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/zzz", "zzz\n");
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
test/xxx/xxx
test/xxx/yyy
test/yyy/zzz
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_two_glob_expansions_for_variable()
{
    let s = "
unset VAR
VAR='test/*/* test2/*'
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/xxx/xxx
test/yyy/yyy
test/yyy/zzz
test2/aaa
test2/bbb
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_glob_expansion_for_string_in_double_quotation()
{
    let s = "
./rsush_test args \"test/*/*\"
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            write_file("test/xxx/yyy", "yyy\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/zzz", "zzz\n");
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
test/*/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_glob_expansion_for_string_in_double_quotation_and_variable()
{
    let s = "
unset VAR
VAR='test/*/*'
./rsush_test args \"$VAR\"
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
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
test/*/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_glob_expansion_for_string_in_single_quotation()
{
    let s = "
./rsush_test args 'test/*/*'
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            write_file("test/xxx/yyy", "yyy\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/zzz", "zzz\n");
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
test/*/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_two_glob_expansions_for_expansion_words()
{
    let s = "
unset VAR
./rsush_test args ${VAR:-test/*/* test2/*}
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/xxx/xxx
test/yyy/yyy
test/yyy/zzz
test2/aaa
test2/bbb
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_two_glob_expansions_for_doubly_quoted_expansion_words()
{
    let s = "
unset VAR
./rsush_test args ${VAR:-\"test/*/*\" \"test2/*\"}
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/*/*
test2/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_two_glob_expansions_for_expansion_words_in_double_quotation()
{
    let s = "
unset VAR
./rsush_test args \"${VAR:-test/*/* test2/*}\"
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/*/*
test2/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_two_glob_expansions_for_doubly_quoted_expansion_words_in_double_quotation()
{
    let s = "
unset VAR
./rsush_test args \"${VAR:-\"test/*/*\" \"test2/*\"}\"
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
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/yyy", "yyy\n");
            write_file("test/yyy/zzz", "zzz\n");
            make_dir_all("test2");
            write_file("test2/aaa", "aaa\n");
            write_file("test2/bbb", "bbb\n");
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
test/*/*
test2/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_performs_tylde_expansion_for_noglob_that_is_set()
{
    let s = "
./rsush_test args ~
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
            settings.noglob_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("IFS");
            env.unset_var("HOME");
            env.set_exported_var("HOME", "/home/luck");
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
/home/luck
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_perform_glob_expansion_for_noglob_that_is_set()
{
    let s = "
./rsush_test args test/*/*
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
            settings.noglob_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("IFS");
            make_dir_all("test/xxx");
            write_file("test/xxx/xxx", "xxx\n");
            write_file("test/xxx/yyy", "yyy\n");
            make_dir_all("test/yyy");
            write_file("test/yyy/zzz", "zzz\n");
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
test/*/*
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_compains_on_unset_variable_for_nounset_that_is_set()
{
    let s = "
unset VAR
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
            settings.nounset_flag = true;
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
fn test_interpreter_interpret_logical_commands_compains_on_unset_variable_in_arithmetic_expansion_for_nounset_that_is_set()
{
    let s = "
unset VAR
./rsush_test args $((VAR))
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
            settings.nounset_flag = true;
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
fn test_interpreter_interpret_logical_commands_sets_variables_for_command()
{
    let s = "
VAR1=abc VAR2=def VAR3=ghi ./rsush_test env
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
            settings.nounset_flag = true;
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
            let stdout_content = read_file("stdout.txt");
            assert!(stdout_content.contains("VAR1=abc\n"));
            assert!(stdout_content.contains("VAR2=def\n"));
            assert!(stdout_content.contains("VAR3=ghi\n"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_sets_variables()
{
    let s = "
unset VAR1 VAR2 VAR3
VAR1=abc VAR2=def VAR3=ghi
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
            settings.nounset_flag = true;
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_return_zero_last_status()
{
    let s = "
./rsush_test exit 0
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
            settings.nounset_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_return_other_last_status()
{
    let s = "
./rsush_test exit 12
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
            settings.nounset_flag = true;
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
            assert_eq!(12, status);
            assert_eq!(12, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_return_other_last_status_for_two_commands()
{
    let s = "
./rsush_test exit 11
./rsush_test exit 12
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
            settings.nounset_flag = true;
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
            assert_eq!(12, status);
            assert_eq!(12, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_prints_commands_for_xtrace_that_is_set()
{
    let s = "
unset VAR1 VAR2
VAR1=abc VAR2=def
VAR3=ghi ./rsush_test args abc
./rsush_test args $VAR1
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
            settings.xtrace_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("PS4");
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
            let expected_stderr_content = "
+ unset VAR1 VAR2
+ VAR1=abc VAR2=def
+ VAR3=ghi ./rsush_test args abc
+ ./rsush_test args abc
";
            assert_eq!(String::from(&expected_stderr_content[1..]), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_prints_commands_for_extxtrace_that_is_set()
{
    let s = "
unset VAR1 VAR2
VAR1=abc VAR2=def
VAR3=ghi ./rsush_test args abc
./rsush_test args $VAR1
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
            settings.extxtrace_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            env.unset_var("PS4");
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
            let expected_stderr_content = "
+ test.sh: 1.1: unset VAR1 VAR2
+ test.sh: 2.1: VAR1=abc VAR2=def
+ test.sh: 3.1: VAR3=ghi ./rsush_test args abc
+ test.sh: 4.1: ./rsush_test args abc
";
            assert_eq!(String::from(&expected_stderr_content[1..]), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_redirections()
{
    let s = "
./rsush_test write_2fds 1 2 2 abc def ghi jkl > test1.txt 2> test2.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test1_txt_content = "
abc
def
";
            assert_eq!(String::from(&expected_test1_txt_content[1..]), read_file("test1.txt"));
            let expected_test2_txt_content = "
ghi
jkl
";
            assert_eq!(String::from(&expected_test2_txt_content[1..]), read_file("test2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_input_redirection()
{
    let s = "
./rsush_test read_fd 0 7 < test.txt
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
            write_file("test.txt", "abcdef\n");
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
abcdef

";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection()
{
    let s = "
./rsush_test args abc def ghi > test.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_for_overwriting()
{
    let s = "
./rsush_test args abc def ghi > test.txt
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
            write_file("test.txt", "xxx\nyyy\n");
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
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_with_bar()
{
    let s = "
./rsush_test args abc def ghi >| test.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_with_bar_for_overwriting()
{
    let s = "
./rsush_test args abc def ghi >| test.txt
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
            write_file("test.txt", "xxx\nyyy\n");
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
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_input_and_output_redirection_for_reading()
{
    let s = "
./rsush_test read_fd 0 7 <> test.txt
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
            write_file("test.txt", "abcdef\n");
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
abcdef

";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_input_and_output_redirection_for_writing()
{
    let s = "
./rsush_test args abc def 1<> test.txt
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
            write_file("test.txt", "ghi\njkl\nmno\n");
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
            let expected_test_txt_content = "
abc
def
mno
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_appending_redirection()
{
    let s = "
./rsush_test args abc def >> test.txt
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
            write_file("test.txt", "xxx\nyyy\n");
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
            let expected_test_txt_content = "
xxx
yyy
abc
def
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_input_duplicating_redirection()
{
    let s = "
./rsush_test read_fd 3 22 3<& 0
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
Some line
Second line

";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_duplicating_redirection()
{
    let s = "
./rsush_test write_2fds 1 2 2 abc def ghi jkl 2>& 1
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
fn test_interpreter_interpret_logical_commands_interprets_here_document_redirection()
{
    let s = "
./rsush_test read_fd 0 14 << EOT
abcdef
ghijkl
EOT
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
abcdef
ghijkl

";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_two_here_document_redirections()
{
    let s = "
./rsush_test read_2fds 0 7 3 7 << EOT 3<< EOT
abcdef
EOT
ghijkl
EOT
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
abcdef

ghijkl

";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_here_document_redirection_for_variable_and_at_arguments_and_star_arguments()
{
    let s = "
unset VAR
VAR=ghi
cat << EOT
abc${VAR}def
$@ $*
EOT
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
abcghidef
abc def abc def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_changes_redirection_files()
{
    let s = "
exec > test1.txt 2> test2.txt
./rsush_test args abc def
./rsush_test write_fd 2 ghi jkl
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test1_txt_content = "
abc
def
";
            assert_eq!(String::from(&expected_test1_txt_content[1..]), read_file("test1.txt"));
            let expected_test2_txt_content = "
ghi
jkl
";
            assert_eq!(String::from(&expected_test2_txt_content[1..]), read_file("test2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_for_noclobber_that_is_set()
{
    let s = "
./rsush_test args abc def ghi > test.txt
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
            settings.noclobber_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_overwrite_for_noclobber_that_is_set_and_output_redirection()
{
    let s = "
./rsush_test args abc def ghi > test.txt
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
            settings.noclobber_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            write_file("test.txt", "xxx\nyyy\n");
            write_file("stdin.txt", "Some line\nSecond line\n");
            exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
            exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
            exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
            exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            let stderr2_content = read_file("stderr2.txt");
            assert!(stderr2_content.starts_with("test.txt: "));
            let expected_test_txt_content = "
xxx
yyy
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_with_bar_for_noclobber_that_is_set()
{
    let s = "
./rsush_test args abc def ghi >| test.txt
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
            settings.noclobber_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_output_redirection_with_bar_for_noclobber_that_is_set_and_overwriting()
{
    let s = "
./rsush_test args abc def ghi >| test.txt
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
            settings.noclobber_flag = true;
            settings.arg0 = String::from("rsush");
            initialize_builtin_funs(&mut env);
            initialize_test_builtin_funs(&mut env);
            initialize_vars(&mut env);
            write_file("test.txt", "xxx\nyyy\n");
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
            let expected_test_txt_content = "
abc
def
ghi
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_negative_command_for_status_zero()
{
    let s = "
! ./rsush_test exit 0
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_interprets_negative_command_for_other_status()
{
    let s = "
! ./rsush_test exit 11
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_pipes()
{
    let s = "
./rsush_test args abc def | cat | tee test.txt
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_negative_pipes()
{
    let s = "
! ./rsush_test args abc def | cat | tee test.txt
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc
def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc
def
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_and_for_first_true()
{
    let s = "
true && ./rsush_test args abc def
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_and_for_first_false()
{
    let s = "
false && ./rsush_test args abc def
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_interprets_logical_or_for_first_true()
{
    let s = "
true || ./rsush_test args abc def
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_or_for_first_false()
{
    let s = "
false || ./rsush_test args abc def
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_and_and_logical_or_for_first_example()
{
    let s = "
true && ./rsush_test args abc def || ./rsush_test args ghi jkl
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_and_and_logical_or_for_second_example()
{
    let s = "
true && false || ./rsush_test args ghi jkl
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
fn test_interpreter_interpret_logical_commands_interprets_logical_and_and_logical_or_for_third_example()
{
    let s = "
false && ./rsush_test args abc def || ./rsush_test args ghi jkl
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
fn test_interpreter_interpret_logical_commands_interprets_command_for_errexit_that_is_set_and_status_zero()
{
    let s = "
./rsush_test exit 0
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
            settings.errexit_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_command_for_errexit_that_is_set_and_status_one()
{
    let s = "
./rsush_test exit 1
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_negative_command_for_errexit_that_is_set()
{
    let s = "
! ./rsush_test exit 1
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
            settings.errexit_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_logical_operators_for_errexit_that_is_set()
{
    let s = "
true && false || false
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_does_not_interprets_command_for_noexec_that_is_set()
{
    let s = "
./rsush_test args abc def
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
            settings.noexec_flag = true;
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_alias()
{
    let s = "
alias alias2='./rsush_test args'
alias2 abc def
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_alias_with_variable()
{
    let s = "
alias alias2='./rsush_test args $VAR'
unset VAR
VAR=ghi
alias2 abc def
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
ghi
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
fn test_interpreter_interpret_logical_commands_interprets_alias_for_settings_of_variables()
{
    let s = "
alias alias2='VAR1=abc ./rsush_test env'
VAR2=def alias2
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
            let stdout_content = read_file("stdout.txt");
            assert!(stdout_content.contains("VAR1=abc\n"));
            assert!(stdout_content.contains("VAR2=def\n"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_alias_with_redirections()
{
    let s = "
alias alias2='./rsush_test write_2fds 1 2 2 abc def ghi jkl > test1.txt'
alias2 2> test2.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test1_txt_content = "
abc
def
";
            assert_eq!(String::from(&expected_test1_txt_content[1..]), read_file("test1.txt"));
            let expected_test2_txt_content = "
ghi
jkl
";
            assert_eq!(String::from(&expected_test2_txt_content[1..]), read_file("test2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_brace_group()
{
    let s = "
{
    echo abc def
    echo ghi jkl
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
            let status = interp.interpret_logical_commands(&mut exec, logical_commands.as_slice(), &mut env, &mut settings);
            exec.clear_files();
            assert_eq!(0, status);
            assert_eq!(0, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc def
ghi jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_brace_group_for_variable_changing()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
{
    echo $VAR1 $VAR2
    VAR2=ghi
    echo $VAR1 $VAR2
}
echo $VAR1 $VAR2
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
abc def
abc ghi
abc ghi
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_brace_group_for_redirections()
{
    let s = "
{
    echo abc
    echo def > test2.txt
    echo ghi
} > test1.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test1_txt_content = "
abc
ghi
";
            assert_eq!(String::from(&expected_test1_txt_content[1..]), read_file("test1.txt"));
            let expected_test2_txt_content = "
def
";
            assert_eq!(String::from(&expected_test2_txt_content[1..]), read_file("test2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_brace_group_for_errexit_that_is_set()
{
    let s = "
{
    echo abc def
    false
    echo ghi jkl
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
abc def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_subshell()
{
    let s = "
(
    echo abc def
    echo ghi jkl
)
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
abc def
ghi jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_subshell_for_variable_changing()
{
    let s = "
unset VAR1
VAR1=abc
unset VAR2
VAR2=def
(
    echo $VAR1 $VAR2
    VAR2=ghi
    echo $VAR1 $VAR2
)
echo $VAR1 $VAR2
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
abc def
abc ghi
abc def
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_subshell_for_redirections()
{
    let s = "
(
    echo abc
    echo def > test2.txt
    echo ghi
) > test1.txt
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test1_txt_content = "
abc
ghi
";
            assert_eq!(String::from(&expected_test1_txt_content[1..]), read_file("test1.txt"));
            let expected_test2_txt_content = "
def
";
            assert_eq!(String::from(&expected_test2_txt_content[1..]), read_file("test2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_for_loop()
{
    let s = "
for i in 1 2 3 4 5; do
    echo $i
done
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
1
2
3
4
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_for_loop_with_break()
{
    let s = "
for i in 1 2 3 4 5; do
    if [ \"$i\" -eq 4 ]; then
        break
    fi
    echo $i
done
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
1
2
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
fn test_interpreter_interpret_logical_commands_interprets_for_loop_with_continue()
{
    let s = "
for i in 1 2 3 4 5; do
    if [ \"$i\" -eq 4 ]; then
        continue
    fi
    echo $i
done
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
1
2
3
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_for_loop_with_double_break()
{
    let s = "
for i in 1 2 3 4 5; do
    for j in 1 2 3 4 5; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            break 2
        fi
        echo \"$i =/= $j\"
    done
done
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
1 = 1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_for_loop_with_double_continue()
{
    let s = "
for i in 1 2 3 4 5; do
    for j in 1 2 3 4 5; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            continue 2
        fi
        echo \"$i =/= $j\"
    done
done
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
1 = 1
2 =/= 1
2 = 2
3 =/= 1
3 =/= 2
3 = 3
4 =/= 1
4 =/= 2
4 =/= 3
4 = 4
5 =/= 1
5 =/= 2
5 =/= 3
5 =/= 4
5 = 5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_for_loop_for_errexit_that_is_set()
{
    let s = "
for i in 1 2 3 4 5; do
    echo $i
    false
done
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_first_match()
{
    let s = "
case abcdef in
    abc*) echo abc;;
    def*) echo def;;
esac
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_second_match()
{
    let s = "
case defabc in
    abc*) echo abc;;
    def*) echo def;;
esac
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
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_no_match()
{
    let s = "
case xxxdef in
    abc*) echo abc;;
    def*) echo def;;
esac
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_first_match_and_patterns()
{
    let s = "
case 123def in
    abc*|123*) echo abc;;
    def*|456*) echo def;;
esac
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_second_match_and_patterns()
{
    let s = "
case 456abc in
    abc*|123*) echo abc;;
    def*|456*) echo def;;
esac
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
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_no_match_and_patterns()
{
    let s = "
case xxxdef in
    abc*|123*) echo abc;;
    def*|456*) echo def;;
esac
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_errexit_that_is_set()
{
    let s = "
case abcdef in
    abc*)
        echo abc
        false
        echo def
        ;;
esac
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_for_true_condition()
{
    let s = "
if true; then
    echo abc
fi
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_if_clause_for_false_condition()
{
    let s = "
if false; then
    echo abc
fi
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_else_for_true_condition()
{
    let s = "
if true; then
    echo abc
else
    echo def
fi
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_else_for_false_condition()
{
    let s = "
if false; then
    echo abc
else
    echo def
fi
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_for_true_first_condition()
{
    let s = "
if true; then
    echo abc
elif true; then
    echo def
elif true; then
    echo ghi
fi
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_for_true_second_condition()
{
    let s = "
if false; then
    echo abc
elif true; then
    echo def
elif true; then
    echo ghi
fi
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_for_true_third_condition()
{
    let s = "
if false; then
    echo abc
elif false; then
    echo def
elif true; then
    echo ghi
fi
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_for_all_false_conditions()
{
    let s = "
if false; then
    echo abc
elif false; then
    echo def
elif false; then
    echo ghi
fi
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_and_else_for_all_false_conditions()
{
    let s = "
if false; then
    echo abc
elif false; then
    echo def
elif false; then
    echo ghi
else
    echo jkl
fi
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_for_errexit_that_is_set()
{
    let s = "
if true; then
    echo abc
    false
    echo def
fi
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_else_for_errexit_that_is_set()
{
    let s = "
if false; then
    echo abc
else
    echo def
    false
    echo ghi
fi
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_with_three_conditions_for_errexit_that_is_set()
{
    let s = "
if false; then
    echo abc
elif false; then
    echo def
elif true; then
    echo ghi
    false
    echo jkl
fi
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
            settings.errexit_flag = true;
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
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_interprets_while_loop()
{
    let s = "
i=1
while [ \"$i\" -le 5 ]; do
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
3
4
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_while_loop_with_break()
{
    let s = "
i=1
while [ \"$i\" -le 5 ]; do
    if [ \"$i\" -eq 4 ]; then
        break
    fi
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
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
fn test_interpreter_interpret_logical_commands_interprets_while_loop_with_continue()
{
    let s = "
i=1
while [ \"$i\" -le 5 ]; do
    if [ \"$i\" -eq 4 ]; then
        i=\"`expr \"$i\" + 1`\"
        continue
    fi
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
3
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_while_loop_with_double_break()
{
    let s = "
i=1
while [ \"$i\" -le 5 ]; do
    j=1
    while [ \"$j\" -le 5 ]; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            break 2
        fi
        echo \"$i =/= $j\"
        j=\"`expr \"$j\" + 1`\"
    done
    i=\"`expr \"$i\" + 1`\"
done
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
1 = 1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_while_loop_with_double_continue()
{
    let s = "
i=1
while [ \"$i\" -le 5 ]; do
    j=1
    while [ \"$j\" -le 5 ]; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            i=\"`expr \"$i\" + 1`\"
            continue 2
        fi
        echo \"$i =/= $j\"
        j=\"`expr \"$j\" + 1`\"
    done
    i=\"`expr \"$i\" + 1`\"
done
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
1 = 1
2 =/= 1
2 = 2
3 =/= 1
3 =/= 2
3 = 3
4 =/= 1
4 =/= 2
4 =/= 3
4 = 4
5 =/= 1
5 =/= 2
5 =/= 3
5 =/= 4
5 = 5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_while_loop_for_errexit_that_is_set()
{
    let s = "
i=1
while false
    [ \"$i\" -le 5 ]; do
    echo $i
    false
    i=\"`expr \"$i\" + 1`\"
done
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_until_loop()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
3
4
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_until_loop_with_break()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    if [ \"$i\" -eq 4 ]; then
        break
    fi
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
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
fn test_interpreter_interpret_logical_commands_interprets_until_loop_with_continue()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    if [ \"$i\" -eq 4 ]; then
        i=\"`expr \"$i\" + 1`\"
        continue
    fi
    echo $i
    i=\"`expr \"$i\" + 1`\"
done
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
1
2
3
5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_until_loop_with_double_break()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    j=1
    until [ \"$j\" -gt 5 ]; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            break 2
        fi
        echo \"$i =/= $j\"
        j=\"`expr \"$j\" + 1`\"
    done
    i=\"`expr \"$i\" + 1`\"
done
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
1 = 1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_nested_until_loop_with_double_continue()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    j=1
    until [ \"$j\" -gt 5 ]; do
        if [ \"$i\" -eq \"$j\" ]; then
            echo \"$i = $j\"
            i=\"`expr \"$i\" + 1`\"
            continue 2
        fi
        echo \"$i =/= $j\"
        j=\"`expr \"$j\" + 1`\"
    done
    i=\"`expr \"$i\" + 1`\"
done
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
1 = 1
2 =/= 1
2 = 2
3 =/= 1
3 =/= 2
3 = 3
4 =/= 1
4 =/= 2
4 =/= 3
4 = 4
5 =/= 1
5 =/= 2
5 =/= 3
5 =/= 4
5 = 5
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_until_loop_for_errexit_that_is_set()
{
    let s = "
i=1
until [ \"$i\" -gt 5 ]; do
    echo $i
    false
    i=\"`expr \"$i\" + 1`\"
done
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
1
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_function_definition_and_function_invocation()
{
    let s = "
f() {
    echo abc $*
    echo jkl
}
f def ghi
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
abc def ghi
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
fn test_interpreter_interpret_logical_commands_interprets_function_definition_and_function_invocation_for_settings_of_variables()
{
    let s = "
f() {
    echo abc $VAR1 $VAR2
    echo jkl
}
VAR1=def VAR2=ghi f
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
abc def ghi
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
fn test_interpreter_interpret_logical_commands_interprets_function_definition_and_function_invocation_for_redirection()
{
    let s = "
f() {
    echo abc def
} > test.txt
f
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
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
            let expected_test_txt_content = "
abc def
";
            assert_eq!(String::from(&expected_test_txt_content[1..]), read_file("test.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_function_with_last_status_zero_as_return_value()
{
    let s = "
f() {
    echo abc
    true
}
f
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_function_with_last_status_one_as_return_value()
{
    let s = "
f() {
    echo abc
    false
}
f
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_interprets_function_with_return_and_return_value_zero()
{
    let s = "
f() {
    echo abc
    return 0
    echo def
}
f
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_function_with_return_and_return_value_one()
{
    let s = "
f() {
    echo abc
    return 1
    echo def
}
f
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_interprets_nested_function_invocation()
{
    let s = "
f() {
    echo def $*
}
g() {
    echo abc $1
    f 123 456
    echo ghi $2
}
g xxx yyy
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
abc xxx
def 123 456
ghi yyy
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_function_definition_and_function_invocation_for_errexit_that_is_set()
{
    let s = "
f() {
    echo abc
    false
    echo def
}
f
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
            settings.errexit_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            let expected_stdout_content = "
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
fn test_interpreter_interpret_logical_commands_exports_variables()
{
    let s = "
export VAR1=abc VAR2=def VAR3=ghi
./rsush_test env
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
            settings.nounset_flag = true;
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
            let stdout_content = read_file("stdout.txt");
            assert!(stdout_content.contains("VAR1=abc\n"));
            assert!(stdout_content.contains("VAR2=def\n"));
            assert!(stdout_content.contains("VAR3=ghi\n"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_set_varaible_that_has_readonly_attribute()
{
    let s = "
unset VAR
VAR=abc
readonly VAR
VAR=def
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
            settings.nounset_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_set_varaible_that_has_readonly_attribute_for_colon_equal_parameter_expansion()
{
    let s = "
unset VAR
readonly VAR
./rsush_test args ${VAR:=abc}
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
            settings.nounset_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_set_varaible_that_has_readonly_attribute_for_equal_parameter_expansion()
{
    let s = "
unset VAR
readonly VAR
./rsush_test args ${VAR=abc}
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
            settings.nounset_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_does_not_set_varaible_that_has_readonly_attribute_for_arithmetic_expansion()
{
    let s = "
unset VAR
readonly VAR
./rsush_test args $((VAR=1))
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
            settings.nounset_flag = true;
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::Exit(false), interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::from("VAR: Is read only\n"), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_non_existent_file_for_input_redirection()
{
    let s = "
./rsush_test read_fd 0 7 < test.txt
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            let stderr2_content = read_file("stderr2.txt");
            assert!(stderr2_content.starts_with("test.txt: "));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_invalid_i_o_number_for_input_duplicating_redirection()
{
    let s = "
./rsush_test read_fd 3 22 3<& xxx
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("test.sh: 1.27: invalid I/O number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_too_large_i_o_number_for_input_duplicating_redirection()
{
    let s = "
./rsush_test read_fd 3 22 3<& 2147483648
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("test.sh: 1.27: too large I/O number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_bad_fd_number_for_input_duplicating_redirection()
{
    let s = "
./rsush_test read_fd 3 22 3<& 4
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("4: Bad fd number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_invalid_i_o_number_for_output_duplicating_redirection()
{
    let s = "
./rsush_test write_2fds 1 2 2 abc def ghi jkl 2>& xxx
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("test.sh: 1.47: invalid I/O number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_too_large_i_o_number_for_output_duplicating_redirection()
{
    let s = "
./rsush_test write_2fds 1 2 2 abc def ghi jkl 2>& 2147483648
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("test.sh: 1.47: too large I/O number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_complains_on_bad_fd_number_for_output_duplicating_redirection()
{
    let s = "
./rsush_test write_2fds 1 2 2 abc def ghi jkl 2>& 3
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
            assert_eq!(1, status);
            assert_eq!(1, interp.last_status);
            assert_eq!(ReturnState::None, interp.return_state);
            assert_eq!(false, interp.exec_redirect_flag);
            assert_eq!(String::new(), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::from("3: Bad fd number\n"), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_commands_for_bug_of_variable_and_pattern_expansion_in_first_case()
{
    let s = "
VAR='\\\\\\'
echo $VAR
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
\\\\\\
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_commands_for_bug_of_variable_and_pattern_expansion_in_second_case()
{
    let s = "
VAR1='   '
echo \"abc${VAR1}def\"
VAR2=$VAR1
echo \"ghi${VAR2}jkl\"
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
abc   def
ghi   jkl
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_case_clause_for_bug_of_case_pattern_expasion()
{
    let s = "
case abc in
    *) echo abc;;
esac
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
";
            assert_eq!(String::from(&expected_stdout_content[1..]), read_file("stdout.txt"));
            assert_eq!(String::new(), read_file("stderr.txt"));
            assert_eq!(String::new(), read_file("stderr2.txt"));
        },
        _ => assert!(false),
    }
}

#[sealed_test(before=setup(), after=teardown())]
fn test_interpreter_interpret_logical_commands_interprets_for_loop_for_bug_of_for_default_words()
{
    let s = "
for i; do
    echo $i
done
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_for_first_condition_and_bug_of_returns_of_clause_conditions()
{
    let s = "
if exit 11; then
    echo abc
fi
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
            assert_eq!(11, status);
            assert_eq!(11, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
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
fn test_interpreter_interpret_logical_commands_interprets_if_clause_for_second_condition_and_bug_of_returns_of_clause_conditions()
{
    let s = "
if false; then
    echo abc
elif exit 11; then
    echo def
fi
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
            assert_eq!(11, status);
            assert_eq!(11, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
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
fn test_interpreter_interpret_logical_commands_interprets_while_loop_for_bug_of_returns_of_clause_conditions()
{
    let s = "
while exit 11; do
    echo abc
done
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
            assert_eq!(11, status);
            assert_eq!(11, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
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
fn test_interpreter_interpret_logical_commands_interprets_until_loop_for_bug_of_returns_of_clause_conditions()
{
    let s = "
until exit 11; do
    echo abc
done
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
            assert_eq!(11, status);
            assert_eq!(11, interp.last_status);
            assert_eq!(ReturnState::Exit(true), interp.return_state);
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
