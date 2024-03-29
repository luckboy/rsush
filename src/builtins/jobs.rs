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
use crate::interp::*;
use crate::settings::*;
use crate::xcfprintln;

struct Options
{
    job_format_flag: JobFormatFlag,
}

fn print_job(job_id: u32, job: &Job, opts: &Options, interp: &Interpreter, exec: &Executor)
{ xcfprintln!(exec, 1, "{}", interp.job_to_string(exec, job_id, job, None, opts.job_format_flag)); }

fn print_job_for_job_id(job_id: u32, opts: &Options, interp: &Interpreter, exec: &Executor) -> bool
{
    match exec.jobs().get(&job_id) {
        Some(job) => {
            xcfprintln!(exec, 1, "{}", interp.job_to_string(exec, job_id, job, None, opts.job_format_flag));
            true
        },
        None => {
            xcfprintln!(exec, 2, "{}: No job", job_id);
            false
        }
    }
}
    
pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "lp");
    let mut opts = Options {
        job_format_flag: JobFormatFlag::None,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('l', _))) => opts.job_format_flag = JobFormatFlag::Long,
            Some(Ok(Opt('p', _))) => opts.job_format_flag = JobFormatFlag::Process,
            Some(Ok(Opt(c, _))) => {
                xcfprintln!(exec, 2, "unknown option -- {:?}", c);
                return 1;
            },
            Some(Err(err)) => {
                xcfprintln!(exec, 2, "{}", err);
                return 1;
            },
            None => break,
        }
    }
    let mut status = 0;
    let args: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    if !args.is_empty() {
        for arg in &args {
            match exec.parse_job_id(arg.as_str()) {
                Ok(job_id) => {
                    if !print_job_for_job_id(job_id, &opts, interp, exec) {
                        status = 1;
                    }
                },
                Err(JobIdError::NoPercent) => {
                    match arg.parse::<u32>() {
                        Ok(job_id) => {
                            if !print_job_for_job_id(job_id, &opts, interp, exec) {
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
        for (job_id, job) in exec.jobs() {
            print_job(*job_id, job, &opts, interp, exec);
        }
    }
    status
}
