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
use std::fs::*;
use crate::exec::*;
use crate::xsfprintln;

pub fn with_std_files<T, F>(exec: &Executor, f: F) -> Option<T>
    where F: FnOnce(&mut File, &mut File, &mut File) -> T
{
    match exec.current_file(0) {
        Some(stdin_file) => {
            match exec.current_file(1) {
                Some(stdout_file) => {
                    match exec.current_file(2) {
                        Some(stderr_file) => {
                            let mut stdin_file_r = stdin_file.borrow_mut(); 
                            let mut stdout_file_r = stdout_file.borrow_mut(); 
                            let mut stderr_file_r = stderr_file.borrow_mut(); 
                            Some(f(&mut *stdin_file_r, &mut *stdout_file_r, &mut *stderr_file_r))
                        },
                        None => {
                            xsfprintln!(exec, 2, "No standard error");
                            None
                        },
                    }
                },
                None => {
                    xsfprintln!(exec, 2, "No standard output");
                    None
                },
            }
        },
        None => {
            xsfprintln!(exec, 2, "No standard input");
            None
        },
    }
}
