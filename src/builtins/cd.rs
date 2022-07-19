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
use std::io::*;
use std::path;
use std::path::*;
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::exec_utils::*;
use crate::interp::*;
use crate::settings::*;
use crate::fprintln;

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
    with_std_files(exec, |_, stdout, stderr| {
            let mut line_stdout = LineWriter::new(stdout);
            let mut opt_parser = getopt::Parser::new(args, "LP");
            let mut opts = Options {
                path_flag: PathFlag::None,
            };
            loop {
                match opt_parser.next() {
                    Some(Ok(Opt('L', _))) => opts.path_flag = PathFlag::Logical,
                    Some(Ok(Opt('P', _))) => opts.path_flag = PathFlag::Physical,
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
            let paths: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
            let (mut path_buf, is_pwd) = match paths.get(1) {
                Some(path) => {
                    if paths.len() > 1 {
                        fprintln!(stderr, "Too many arguments");
                        return 1;
                    }
                    if *path == &String::from("-") {
                        match env.var("OLDPWD") {
                            Some(oldpwd) => (PathBuf::from(oldpwd), true),
                            None => {
                                fprintln!(stderr, "OLDPWD not set");
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
                        fprintln!(stderr, "{}: {}", path_buf.as_path().to_string_lossy(), err);
                        return 1;
                    },
                }
            }
            match env::set_current_dir(path_buf.as_path()) {
                Ok(())   => (),
                Err(err) => {
                    fprintln!(stderr, "{}: {}", path_buf.as_path().to_string_lossy(), err);
                    return 1;
                },
            }
            match env::current_dir() {
                Ok(tmp_path_buf) => path_buf = tmp_path_buf,
                Err(err) => {
                    fprintln!(stderr, "{}", err);
                    return 1;
                },
            }
            env.set_var("PWD", path_buf.as_path().to_string_lossy().into_owned().as_str(), settings);
            if is_pwd {
                fprintln!(&mut line_stdout, "{}", path_buf.as_path().to_string_lossy());
            }
            0
    }).unwrap_or(1)
}
