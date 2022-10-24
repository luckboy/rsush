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
use std::env;
use std::fs;
use std::path;
use std::path::*;
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::xcfprintln;

#[derive(Eq, PartialEq)]
enum PathFlag
{
    None,
    Logical,
    Physical,
}

struct Options
{
    path_flag: PathFlag,
}

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "LP");
    let mut opts = Options {
        path_flag: PathFlag::None,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('L', _))) => opts.path_flag = PathFlag::Logical,
            Some(Ok(Opt('P', _))) => opts.path_flag = PathFlag::Physical,
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
    let paths: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    let (mut path_buf, is_pwd) = match paths.get(0) {
        Some(path) => {
            if paths.len() > 1 {
                xcfprintln!(exec, 2, "Too many arguments");
                return 1;
            }
            if *path == &String::from("-") {
                match env.var("OLDPWD") {
                    Some(oldpwd) => (PathBuf::from(oldpwd), true),
                    None => {
                        xcfprintln!(exec, 2, "OLDPWD not set");
                        return 1;
                    },
                }
            } else {
                (PathBuf::from(path), false)
            }
        },
        None => {
            let mut sep = String::new();
            sep.push(path::MAIN_SEPARATOR);
            let home = env.var("HOME").unwrap_or(sep);
            (PathBuf::from(home), false)
        },
    };
    if opts.path_flag != PathFlag::Physical {
        match fs::canonicalize(path_buf.as_path()) {
            Ok(tmp_path_buf) => path_buf = tmp_path_buf,
            Err(err) => {
                xcfprintln!(exec, 2, "{}: {}", path_buf.as_path().to_string_lossy(), err);
                return 1;
            },
        }
    }
    match env::set_current_dir(path_buf.as_path()) {
        Ok(())   => (),
        Err(err) => {
            xcfprintln!(exec, 2, "{}: {}", path_buf.as_path().to_string_lossy(), err);
            return 1;
        },
    }
    match env::current_dir() {
        Ok(tmp_path_buf) => path_buf = tmp_path_buf,
        Err(err) => {
            xcfprintln!(exec, 2, "{}", err);
            return 1;
        },
    }
    env.set_var("PWD", path_buf.as_path().to_string_lossy().into_owned().as_str(), settings);
    if is_pwd {
        xcfprintln!(exec, 1, "{}", path_buf.as_path().to_string_lossy());
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
    fn test_cd_builtin_function_changes_current_directory()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let mut expected_new_dir = saved_dir.clone();
        expected_new_dir.push("test");
        assert_eq!(expected_new_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(expected_new_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_logically_changes_current_directory()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-L"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let mut expected_new_dir = saved_dir.clone();
        expected_new_dir.push("test");
        assert_eq!(expected_new_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(expected_new_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_physically_changes_current_directory()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-P"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let mut expected_new_dir = saved_dir.clone();
        expected_new_dir.push("test");
        assert_eq!(expected_new_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(expected_new_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_changes_current_directory_for_oldpwd()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        env.set_unexported_var("OLDPWD", format!("{}/test", saved_dir.as_path().to_string_lossy()).as_str());
        env.unset_exported_var("OLDPWD");
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(format!("{}/test\n", saved_dir.as_path().to_string_lossy()), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let mut expected_new_dir = saved_dir.clone();
        expected_new_dir.push("test");
        assert_eq!(expected_new_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(expected_new_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_changes_current_directory_for_home()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        env.unset_unexported_var("HOME");
        env.set_exported_var("HOME", format!("{}/home/luck", saved_dir.as_path().to_string_lossy()).as_str());
        make_dir_all("home/luck");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(0, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::new(), read_file("stderr2.txt"));
        let mut expected_new_dir = saved_dir.clone();
        expected_new_dir.push("home");
        expected_new_dir.push("luck");
        assert_eq!(expected_new_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(expected_new_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_complains_on_too_many_arguments()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("test"),
            String::from("test2")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("Too many arguments\n"), read_file("stderr2.txt"));
        assert_eq!(saved_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(saved_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }    

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_complains_on_oldpwd_not_set()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        env.unset_var("OLDPWD");
        let saved_dir = current_dir();
        make_dir_all("test");
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        assert_eq!(String::from("OLDPWD not set\n"), read_file("stderr2.txt"));
        assert_eq!(saved_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(saved_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }    
    
    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_complains_on_non_existent_directory()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        let stderr2_content = read_file("stderr2.txt");
        assert!(stderr2_content.starts_with("test: "));
        assert_eq!(saved_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(saved_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_complains_on_non_existent_directory_for_logical()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-L"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        let stderr2_content = read_file("stderr2.txt");
        assert!(stderr2_content.starts_with("test: "));
        assert_eq!(saved_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(saved_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }

    #[sealed_test(before=setup(), after=teardown())]
    fn test_cd_builtin_function_complains_on_non_existent_directory_for_physical()
    {
        let mut exec = Executor::new();
        let mut interp = Interpreter::new();
        let mut env = Environment::new();
        let mut settings = Settings::new();
        settings.arg0 = String::from("rsush");
        initialize_builtin_funs(&mut env);
        initialize_test_builtin_funs(&mut env);
        initialize_vars(&mut env);
        let saved_dir = current_dir();
        write_file("stdin.txt", "Some line\nSecond line\n");
        exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(open_file("stdin.txt"))));
        exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(create_file("stdout.txt"))));
        exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(create_file("stderr.txt"))));
        exec.push_file(2, Rc::new(RefCell::new(create_file("stderr2.txt"))));
        let args = vec![
            String::from("cd"),
            String::from("-P"),
            String::from("test")
        ];
        let status = main(&[], args.as_slice(), &mut interp, &mut exec, &mut env, &mut settings);
        exec.clear_files();
        let new_dir = current_dir();
        set_current_dir(saved_dir.as_path());
        assert_eq!(1, status);
        assert!(interp.has_none());
        assert_eq!(false, interp.exec_redirect_flag());
        assert_eq!(String::new(), read_file("stdout.txt"));
        assert_eq!(String::new(), read_file("stderr.txt"));
        let stderr2_content = read_file("stderr2.txt");
        assert!(stderr2_content.starts_with("test: "));
        assert_eq!(saved_dir, new_dir);
        assert!(env.unexported_var("PWD").is_none());
        assert_eq!(Some(saved_dir.as_path().to_string_lossy().into_owned()), env.exported_var("PWD"));
    }
}
