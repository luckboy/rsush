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
use std::io::*;
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::exec_utils::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;

struct Options
{
    symbolic_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    with_std_files(exec, |_, stdout, stderr| {
            let mut line_stdout = LineWriter::new(stdout);
            let mut opt_parser = getopt::Parser::new(args, "S");
            let mut opts = Options {
                symbolic_flag: false,
            };
            loop {
                match opt_parser.next() {
                    Some(Ok(Opt('S', _))) => opts.symbolic_flag = true,
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
            match args.get(opt_parser.index()) {
                Some(arg) => {
                    match Mode::parse(arg.as_str()) {
                        Some(mode) => {
                            match &mode {
                                Mode::Number(new_mode) => {
                                    umask(*new_mode);
                                },
                                Mode::Symbol(_) => {
                                    let mask = umask(0);
                                    umask(mask);
                                    let new_mode = mode.change_mode(!mask & 0o777, false);
                                    umask(!new_mode & 0o777);
                                },
                            }
                            0
                        },
                        None => {
                            fprintln!(stderr, "Invalid mode");
                            1
                        },
                    }
                },
                None => {
                    let mask = umask(0);
                    umask(mask);
                    if opts.symbolic_flag {
                        fprintln!(&mut line_stdout, "{}", mode_to_string(!mask & 0o777));
                    } else {
                        fprintln!(&mut line_stdout, "{:04o}", mask);
                    }
                    0
                },
            }
    }).unwrap_or(1)
}
