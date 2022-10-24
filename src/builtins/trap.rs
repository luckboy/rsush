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
use std::collections::HashMap;
use libc;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::iter::*;
use crate::settings::*;
use crate::signals::*;
use crate::utils::*;
use crate::xcfprintln;
use crate::xsfprintln;

pub fn initialize_signals(sigs: &mut HashMap<String, i32>)
{
    sigs.insert(String::from("EXIT"), 0);
    sigs.insert(String::from("ABRT"), libc::SIGABRT);
    sigs.insert(String::from("ALRM"), libc::SIGALRM);
    sigs.insert(String::from("BUS"), libc::SIGBUS);
    sigs.insert(String::from("CHLD"), libc::SIGCHLD);
    sigs.insert(String::from("CONT"), libc::SIGCONT);
    sigs.insert(String::from("FPE"), libc::SIGFPE);
    sigs.insert(String::from("HUP"), libc::SIGHUP);
    sigs.insert(String::from("ILL"), libc::SIGILL);
    sigs.insert(String::from("INT"), libc::SIGINT);
    sigs.insert(String::from("KILL"), libc::SIGKILL);
    sigs.insert(String::from("PIPE"), libc::SIGPIPE);
    sigs.insert(String::from("QUIT"), libc::SIGQUIT);
    sigs.insert(String::from("SEGV"), libc::SIGSEGV);
    sigs.insert(String::from("STOP"), libc::SIGSTOP);
    sigs.insert(String::from("TERM"), libc::SIGTERM);
    sigs.insert(String::from("TSTP"), libc::SIGTSTP);
    sigs.insert(String::from("TTIN"), libc::SIGTTIN);
    sigs.insert(String::from("TTOU"), libc::SIGTTOU);
    sigs.insert(String::from("USR1"), libc::SIGUSR1);
    sigs.insert(String::from("USR2"), libc::SIGUSR2);
    //sigs.insert(String::from("POLL"), libc::SIGPOLL); // SIGPOLL doesn't appear in FreeBSD.
    sigs.insert(String::from("PROF"), libc::SIGPROF);
    sigs.insert(String::from("SYS"), libc::SIGSYS);
    sigs.insert(String::from("TRAP"), libc::SIGTRAP);
    sigs.insert(String::from("URG"), libc::SIGURG);
    sigs.insert(String::from("VTALRM"), libc::SIGVTALRM);
    sigs.insert(String::from("XCPU"), libc::SIGXCPU);
    sigs.insert(String::from("XFSZ"), libc::SIGXFSZ);
}

fn initialize_signal_names(sigs: &HashMap<String, i32>, sig_names: &mut HashMap<i32, String>)
{
    for (sig_name, sig) in sigs.iter() {
        sig_names.insert(*sig, sig_name.clone());
    }
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, settings: &mut Settings) -> i32
{
    let mut sigs: HashMap<String, i32> = HashMap::new();
    initialize_signals(&mut sigs);
    let mut arg_iter = PushbackIter::new(args.iter().skip(1));
    match arg_iter.next() {
        Some(arg) => {
            if arg != &String::from("--") {
                arg_iter.undo(arg);
            }
        },
        None => (),
    }
    match arg_iter.next() {
        Some(action) => {
            for arg in arg_iter {
                let sig = match arg.parse::<i32>() {
                    Ok(tmp_sig) => tmp_sig,
                    Err(_) => {
                        match sigs.get(arg) {
                            Some(tmp_sig) => *tmp_sig,
                            None => {
                                xsfprintln!(exec, 2, "Invalid signal");
                                return interp.exit(1, false);
                            },
                        }
                    },
                };
                if action != &String::from("-") {
                    if sig != 0 {
                        match set_signal(sig, true, settings.interactive_flag) {
                            Ok(()) => interp.set_action(sig, action.clone()),
                            Err(err) => {
                                xsfprintln!(exec, 2, "{}", err);
                                return interp.exit(1, false);
                            },
                        }
                    } else {
                        interp.set_action(sig, action.clone());
                    }
                } else {
                    if sig != 0 {
                        match set_signal(sig, false, settings.interactive_flag) {
                            Ok(()) => interp.unset_action(sig),
                            Err(err) => {
                                xsfprintln!(exec, 2, "{}", err);
                                return interp.exit(1, false);
                            },
                        }
                    } else {
                        interp.unset_action(sig)
                    }
                }
            }
            0
        },
        None => {
            let mut sig_names: HashMap<i32, String> = HashMap::new();
            initialize_signal_names(&sigs, &mut sig_names);
            for (sig, action) in interp.actions().iter() {
                let mut sig_name = format!("{}", sig);
                match sig_names.get(sig) {
                    Some(tmp_sig_name) => sig_name = tmp_sig_name.clone(),
                    None => (),
                }
                xcfprintln!(exec, 1, "trap -- {} {}", singly_quote_str(action.as_str()), sig_name);
            }
            0
        },
    }
}
