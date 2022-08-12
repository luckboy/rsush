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
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;
use crate::xsfprintln;

fn on_or_off(b: bool) -> &'static str
{
    if b {
        "on"
    } else {
        "off"
    }
}

fn minus_or_plus(b: bool) -> char
{
    if b {
        '+'
    } else {
        '-'
    }
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    let res = settings.parse_options(args, |opt_type, c, settings| {
            match (opt_type, c) {
                (OptionType::Minus, 'o') => {
                    match exec.current_file(1) {
                        Some(stdout_file) => {
                            let mut stdout_file_r = stdout_file.borrow_mut();
                            let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                            fprintln!(&mut line_stdout, "allexport       {}", on_or_off(settings.allexport_flag));
                            fprintln!(&mut line_stdout, "errexit         {}", on_or_off(settings.errexit_flag));
                            fprintln!(&mut line_stdout, "ignoreeof       {}", on_or_off(settings.ignoreeof_flag));
                            fprintln!(&mut line_stdout, "monitor         {}", on_or_off(settings.monitor_flag));
                            fprintln!(&mut line_stdout, "noclobber       {}", on_or_off(settings.noclobber_flag));
                            fprintln!(&mut line_stdout, "noglob          {}", on_or_off(settings.noglob_flag));
                            fprintln!(&mut line_stdout, "noexec          {}", on_or_off(settings.noexec_flag));
                            fprintln!(&mut line_stdout, "nolog           {}", on_or_off(settings.nolog_flag));
                            fprintln!(&mut line_stdout, "nounset         {}", on_or_off(settings.nounset_flag));
                            fprintln!(&mut line_stdout, "verbose         {}", on_or_off(settings.verbose_flag));
                            fprintln!(&mut line_stdout, "vi              {}", on_or_off(settings.vi_flag));
                            fprintln!(&mut line_stdout, "xtrace          {}", on_or_off(settings.xtrace_flag));
                            fprintln!(&mut line_stdout, "strlossy        {}", on_or_off(settings.strlossy_flag));
                        },
                        None => xsfprintln!(exec, 2, "No standard output"),
                    }
                    true
                },
                (OptionType::Plus, 'o') => {
                    match exec.current_file(1) {
                        Some(stdout_file) => {
                            let mut stdout_file_r = stdout_file.borrow_mut();
                            let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                            fprintln!(&mut line_stdout, "set {}o allexport", minus_or_plus(settings.allexport_flag));
                            fprintln!(&mut line_stdout, "set {}o errexit", minus_or_plus(settings.errexit_flag));
                            fprintln!(&mut line_stdout, "set {}o ignoreeof", minus_or_plus(settings.ignoreeof_flag));
                            fprintln!(&mut line_stdout, "set {}o monitor", minus_or_plus(settings.monitor_flag));
                            fprintln!(&mut line_stdout, "set {}o noclobber", minus_or_plus(settings.noclobber_flag));
                            fprintln!(&mut line_stdout, "set {}o noglob", minus_or_plus(settings.noglob_flag));
                            fprintln!(&mut line_stdout, "set {}o noexec", minus_or_plus(settings.noexec_flag));
                            fprintln!(&mut line_stdout, "set {}o nolog", minus_or_plus(settings.nolog_flag));
                            fprintln!(&mut line_stdout, "set {}o nounset", minus_or_plus(settings.nounset_flag));
                            fprintln!(&mut line_stdout, "set {}o verbose", minus_or_plus(settings.verbose_flag));
                            fprintln!(&mut line_stdout, "set {}o vi", minus_or_plus(settings.vi_flag));
                            fprintln!(&mut line_stdout, "set {}o xtrace", minus_or_plus(settings.xtrace_flag));
                            fprintln!(&mut line_stdout, "set {}o strlossy", minus_or_plus(settings.strlossy_flag));
                        },
                        None => xsfprintln!(exec, 2, "No standard output"),
                    }
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            if args.len() <= 1 {
                match exec.current_file(1) {
                    Some(stdout_file) => {
                        let mut stdout_file_r = stdout_file.borrow_mut();
                        let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                        for (name, value) in env.unexported_vars().iter() {
                            fprintln!(&mut line_stdout, "{}={}", name, singly_quote_str(value.as_str()));
                        }
                        for (name, value) in env.exported_vars() {
                            fprintln!(&mut line_stdout, "{}={}", name, singly_quote_str(value.as_str()));
                        }
                        0
                    },
                    None => {
                        xsfprintln!(exec, 2, "No standard output");
                        interp.exit(1, false)
                    },
                }
            } else {
                if i < args.len() || is_minus_minus {
                    let args: Vec<String> = args.iter().skip(i).map(|a| a.clone()).collect();
                    settings.current_args_mut().set_args(args);
                }
                0
            }
        },
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            interp.exit(1, false)
        },
    }
}
