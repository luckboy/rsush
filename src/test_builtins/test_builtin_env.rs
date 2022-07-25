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
use crate::env::*;
use crate::exec::*;
use crate::exec_utils::*;
use crate::interp::*;
use crate::settings::*;
use crate::fprintln;

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    with_std_files(exec, |_, stdout, stderr| {
            let mut line_stdout = LineWriter::new(stdout);
            if args.len() < 1 {
                fprintln!(stderr, "No built-in function name");
                return 1;
            }
            for (name, value) in std::env::vars() {
                fprintln!(&mut line_stdout, "{}={}", name, value);
            }
            0
    }).unwrap_or(1)
}
