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
fn test_interpreter_interpret_logical_commands_does_not_perform_two_glob_expansions_for_expansion_words()
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
