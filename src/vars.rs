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
use crate::utils::*;

pub fn initialize_vars(env: &mut Environment)
{
    match std::env::current_dir() {
        Ok(path_buf) => {
            env.unset_unexported_var("PWD");
            env.set_exported_var("PWD", path_buf.as_path().to_string_lossy().into_owned().as_str());
        },
        Err(_) => (),
    }
    env.unset_exported_var("PPID");
    env.set_unexported_var("PPID", format!("{}", getppid()).as_str());
}
