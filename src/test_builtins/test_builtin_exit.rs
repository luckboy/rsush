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
use crate::env::*;
use crate::exec::*;
use crate::exec_utils::*;
use crate::interp::*;
use crate::settings::*;
use crate::fprintln;

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    with_std_files(exec, |_, _, stderr| {
            if args.len() < 1 {
                fprintln!(stderr, "No built-in function name");
                return 1;
            }
            match args.get(1) {
                Some(s) => {
                    match s.parse::<i32>() {
                        Ok(status) => status,
                        Err(err) => {
                            fprintln!(stderr, "{}", err);
                            1
                        },
                    }
                },
                None => {
                    fprintln!(stderr, "Too few arguments");
                    1
                },
            }
    }).unwrap_or(1)
}
