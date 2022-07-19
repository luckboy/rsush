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
            if !opts.ignored_escape_flag {
                loop {
                    let mut is_stop = true;
                    let mut line = String::new();
                    match line_stdin.read_line(&mut line) {
                        Ok(0) => break,
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
                }
            } else {
                let mut line = String::new();
                match line_stdin.read_line(&mut line) {
                    Ok(_) => {
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
            let fields = split_str_for_ifs(s.as_str(), ifs.as_str());
            let mut status = 0;
            let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
            for (i, name) in names.iter().enumerate() {
                if is_name_str(name) {
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
            status
    }).unwrap_or(1)
}
