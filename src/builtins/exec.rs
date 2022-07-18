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
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::process::exit;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;

pub fn main(vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, _settings: &mut Settings) -> i32
{
    match args.get(1) {
        Some(prog) => {
            for (name, value) in vars.iter() {
                env.unset_unexported_var(name.as_str());
                env.set_exported_var(name.as_str(), value.as_str());
            }
            match exec.close_and_move_files_for_execute() {
                Ok(()) => {
                    let mut cmd = Command::new(prog);
                    cmd.args(&args[2..]);
                    let err = cmd.exec();
                    eprintln!("{}: {}", prog, err);
                    let status = if err.kind() == ErrorKind::NotFound { 127 } else { 126 };
                    exit(status);
                },
                Err(err) => {
                    eprintln!("{}: {}", prog, err);
                    exit(126);
                },
            }
        },
        None => {
            interp.set_exec_redirect_flag();
            0
        },
    }
}
