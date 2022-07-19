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
use crate::settings::*;
use crate::fprintln;

struct Options
{
    all_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, _settings: &mut Settings) -> i32
{
    with_std_files(exec, |_, _, stderr| {
            let mut opt_parser = getopt::Parser::new(args, "a");
            let mut opts = Options {
                all_flag: false,
            };
            loop {
                match opt_parser.next() {
                    Some(Ok(Opt('a', _))) => opts.all_flag = true,
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
            if !opts.all_flag {
                let mut status = 0;
                let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
                for name in &names {
                    if env.alias(name.as_str()).is_some() {
                        env.unset_alias(name.as_str());
                    } else {
                        fprintln!(stderr, "{}: Not found", name);
                        status = 1;
                    }
                }
                status
            } else {
                env.unset_all_aliases();
                0
            }
    }).unwrap_or(1)
}
