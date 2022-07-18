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
use crate::interp::*;
use crate::settings::*;
use crate::xsfprintln;

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    if !interp.is_in_fun() {
        xsfprintln!(exec, 2, "Isn't in function");
        return interp.exit(1, false);
    }
    match args.get(1) {
        Some(arg) => {
            if args.len() > 2 {
                xsfprintln!(exec, 2, "Too many arguments");
                return interp.exit(1, false);
            }
            match arg.parse::<i32>() {
                Ok(status) => {
                    interp.ret(status)
                },
                Err(_) => {
                    xsfprintln!(exec, 2, "Invalid number");
                    return interp.exit(1, false);
                },
            }
        },
        None => interp.ret(0),
    }
}
