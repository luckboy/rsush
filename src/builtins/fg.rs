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
use libc;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;
use crate::xcfprintln;
use crate::xsfprintln;

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, settings: &mut Settings) -> i32
{
    let job_id = match args.get(1) {
        Some(arg) => {
            match arg.parse::<u32>() {
                Ok(tmp_job_id) => {
                    if args.len() > 2 {
                        xcfprintln!(exec, 2, "Too many arguments");
                        return 1;
                    }
                    tmp_job_id
                },
                Err(_) => {
                    xcfprintln!(exec, 2, "Invalid number");
                    return 1;
                },
            }
        },
        None => {
            match exec.current_job_id() {
                Some(tmp_job_id) => tmp_job_id,
                None => {
                    xcfprintln!(exec, 2, "No job");
                    return 1;
                },
            }
        },
    };
    let (pid, name) = match exec.jobs().get(&job_id) {
        Some(job) => (job.pid, job.name.clone()),
        None => {
            xcfprintln!(exec, 2, "{}: No job", job_id);
            return 1;
        },
    };
    exec.set_foreground_for_process(pid, settings);
    match kill(pid, libc::SIGCONT) {
        Ok(()) => {
            match exec.current_file(1) {
                Some(stdout_file) => {
                    let mut stdout_file_r = stdout_file.borrow_mut();
                    let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                    fprintln!(&mut line_stdout, "{}", name);
                },
                None => {
                    xsfprintln!(exec, 2, "No standard output");
                    return 1;
                },
            }
            exec.remove_job(job_id);
            interp.wait_for_process(exec, Some(pid), false, settings).unwrap_or(1)
        },
        Err(err) => {
            xcfprintln!(exec, 2, "{}: {}", job_id, err);
            1
        },
    }
}
