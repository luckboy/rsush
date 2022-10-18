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
use crate::xcfprintln;

fn wait_for_process(pid: i32, interp: &mut Interpreter, exec: &mut Executor, settings: &Settings) -> i32
{
    interp.wait_for_process(exec, Some(pid), false, false, settings, || String::new(), |exec, wait_status| {
            let mut job_id_and_pid_idx: Option<(u32, Option<usize>)> = None;
            for (job_id, job) in exec.jobs().iter() {
                let mut is_stop = false;
                for (i, (tmp_pid, status)) in job.pids.iter().zip(job.statuses.iter()).enumerate() {
                    match status {
                        WaitStatus::None | WaitStatus::Stopped(_) => {
                            if pid == *tmp_pid {
                                job_id_and_pid_idx = Some((*job_id, Some(i)));
                                is_stop = true;
                                break;
                            }
                        },
                        _ => (),
                    }
                }
                if is_stop { break; }
                match job.last_status {
                    WaitStatus::None | WaitStatus::Stopped(_) => {
                        if pid == job.last_pid {
                            job_id_and_pid_idx = Some((*job_id, None));
                            is_stop = true;
                        }
                    },
                    _ => (),
                }
                if is_stop { break; }
            }
            match job_id_and_pid_idx {
                Some((job_id, Some(i))) => exec.set_job_status(job_id, i, wait_status),
                Some((job_id, None)) => exec.set_job_last_status(job_id, wait_status),
                None => (),
            }
            match job_id_and_pid_idx {
                Some((job_id, _)) => {
                    let is_done = match exec.jobs().get(&job_id) {
                        Some(job) => job.is_done(),
                        None => false,
                    };
                    if is_done {
                        exec.remove_job(job_id);
                    }
                },
                None => (),
            }
    }).unwrap_or(1)
}

fn wait_for_job(job_id: u32, interp: &mut Interpreter, exec: &mut Executor, settings: &Settings) -> i32
{
    let job = match exec.jobs().get(&job_id) {
        Some(tmp_job) => tmp_job.clone(),
        None => {
            xcfprintln!(exec, 2, "{}: No job", job_id);
            return 1;
        },
    };
    let mut pids: Vec<Option<i32>> = Vec::new();
    for (pid, status) in job.pids.iter().zip(job.statuses.iter()) {
        match status {
            WaitStatus::None | WaitStatus::Stopped(_) => pids.push(Some(*pid)),
            _ => (),
        }
    }
    match job.last_status {
        WaitStatus::None | WaitStatus::Stopped(_) => pids.push(Some(job.last_pid)),
        _ => (),
    }
    exec.remove_job(job_id);
    interp.wait_for_processes(exec, pids.as_slice(), Some(job.pgid), pids.len(), false, false, settings, |_: usize| (Vec::new(), String::new(), String::new())).0.unwrap_or(1)
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut status = 0;
    if args.len() > 1 {
        for arg in &args[1..] {
            match exec.parse_job_id(arg.as_str()) {
                Ok(job_id) => {
                    let tmp_status = wait_for_job(job_id, interp, exec, settings);
                    if tmp_status != 0 { 
                        status = tmp_status;
                    }
                },
                Err(JobIdError::NoPercent) => {
                    match arg.parse::<i32>() {
                        Ok(pid) if pid < 0 => {
                            xcfprintln!(exec, 2, "PID is negative");
                            status = 1;
                        },
                        Ok(0) => {
                            xcfprintln!(exec, 2, "PID is zero");
                            status = 1;
                        },
                        Ok(pid) => {
                            let tmp_status = wait_for_process(pid, interp, exec, settings);
                            if tmp_status != 0 { 
                                status = tmp_status;
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
        let job_ids: Vec<u32> = exec.jobs().keys().map(|id| *id).collect();
        for job_id in &job_ids {
            let tmp_status = wait_for_job(*job_id, interp, exec, settings);
            if tmp_status != 0 { 
                status = tmp_status;
            }
        }
    }
    status
}
