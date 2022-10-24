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
use libc;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::xcfprintln;

fn run_job_in_background(job_id: u32, exec: &mut Executor, settings: &Settings) -> bool
{
    let job = match exec.jobs().get(&job_id) {
        Some(tmp_job) => tmp_job.clone(),
        None => {
            xcfprintln!(exec, 2, "{}: No job", job_id);
            return false;
        },
    };
    exec.set_foreground_for_shell(settings);
    let mut is_success = true;
    for (i, (pid, status)) in job.pids.iter().zip(job.statuses.iter()).enumerate() {
        match status {
            WaitStatus::None | WaitStatus::Stopped(_) => {
                match kill(*pid, libc::SIGCONT) {
                    Ok(()) => exec.set_job_status(job_id, i, WaitStatus::None),
                    Err(err) => {
                        xcfprintln!(exec, 2, "{}: {}", job_id, err);
                        is_success = false;
                    },
                }
            },
            _ => (),
        }
    }
    match job.last_status {
        WaitStatus::None | WaitStatus::Stopped(_) => {
            match kill(job.last_pid, libc::SIGCONT) {
                Ok(()) => exec.set_job_last_status(job_id, WaitStatus::None),
                Err(err) => {
                    xcfprintln!(exec, 2, "{}: {}", job_id, err);
                    is_success = false;
                },
            }
        },
        _ => (),
    }
    if is_success {
        xcfprintln!(exec, 1, "[{}] {}", job_id, job.name);
        true
    } else {
        false
    }
}

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut status = 0;
    if args.len() > 1 {
        for arg in &args[1..] {
            match exec.parse_job_id(arg.as_str()) {
                Ok(job_id) => {
                    if !run_job_in_background(job_id, exec, settings) {
                        status = 1;
                    }
                },
                Err(JobIdError::NoPercent) => {
                    match arg.parse::<u32>() {
                        Ok(job_id) => {
                            if !run_job_in_background(job_id, exec, settings) {
                                status = 1;
                            }
                        },
                        Err(_) => {
                            xcfprintln!(exec, 2, "Invalid number");
                            status = 1;
                        },
                    }
                },
                Err(err) => {
                    xcfprintln!(exec, 2, "{}", err);
                    status = 1;
                },
            }
        }
    } else {
        match exec.current_job_id() {
            Some(job_id) => {
                if !run_job_in_background(job_id, exec, settings) {
                    status = 1;
                }
            },
            None => {
                xcfprintln!(exec, 2, "No job");
                status = 1;
            },
        }
    }
    status
}
