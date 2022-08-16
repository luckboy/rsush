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
use std::cell::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::*;
use std::io::*;
use std::path;
use std::rc::*;
use std::slice;
use libc;
use crate::env::*;
use crate::exec::*;
use crate::io::*;
use crate::lexer::*;
use crate::parser::*;
use crate::settings::*;
use crate::utils::*;
use crate::xcfprintln;
use crate::xsfprint;
use crate::xsfprintln;

pub const DEFAULT_IFS: &'static str = " \t\n";

#[derive(Clone, Debug)]
pub enum Value
{
    String(String),
    AtArray(Vec<String>),
    StarArray(Vec<String>),
    ExpansionArray(Vec<String>),
}

impl Value
{
    pub fn is_null(&self) -> bool
    {
        match self {
            Value::String(s) => s.is_empty(),
            Value::AtArray(ss) => ss.is_empty(),
            Value::StarArray(ss) => ss.is_empty(),
            Value::ExpansionArray(ss) => ss.is_empty(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ReturnState
{
    None,
    Break(usize),
    Continue(usize),
    Return,
    Exit(bool),
}

#[derive(Clone)]
enum InterpreterRedirection
{
    Input(i32, String),
    Output(i32, String, bool),
    InputAndOutput(i32, String),
    Appending(i32, String),
    Duplicating(i32, i32),
    HereDocument(i32, String),
}

pub struct Interpreter
{
    last_status: i32,
    non_simple_comamnd_count: usize,
    return_state: ReturnState,
    exec_redirect_flag: bool,
    loop_count_stack: Vec<usize>,
    current_loop_count: usize,
    fun_count: usize,
    last_job_pid: Option<i32>,
    signal_names: HashMap<i32, String>,
    special_builtin_fun_names: HashSet<String>,
}

fn set_vars(exec: &Executor, vars: &[(String, String)], env: &mut Environment, settings: &Settings) -> i32
{
    for (name, value) in vars.iter() {
        if env.read_only_var_attr(name) {
            xcfprintln!(exec, 2, "{}: Is read only", name);
            return 1;
        }
        env.set_var(name.as_str(), value.as_str(), settings);
    }
    0
}

fn print_command_for_xtrace(exec: &Executor, vars: &[(String, String)], args: &[String])
{
    xsfprint!(exec, 2, "+");
    for (name, value) in vars.iter() {
        xsfprint!(exec, 2, " {}={}", name, value);
    }
    for arg in args.iter() {
        xsfprint!(exec, 2, " {}", arg);
    }
    xsfprintln!(exec, 2, "");
}

impl Interpreter
{
    pub fn new() -> Interpreter
    {
        let mut sig_names: HashMap<i32, String> = HashMap::new();
        sig_names.insert(libc::SIGABRT, String::from("Aborted"));
        sig_names.insert(libc::SIGALRM, String::from("Alarm clock"));
        sig_names.insert(libc::SIGBUS, String::from("Bus error"));
        sig_names.insert(libc::SIGCHLD, String::from("Child exited"));
        sig_names.insert(libc::SIGCONT, String::from("Continued"));
        sig_names.insert(libc::SIGFPE, String::from("Floating point exception"));
        sig_names.insert(libc::SIGHUP, String::from("Hangup"));
        sig_names.insert(libc::SIGILL, String::from("Illegal instruction"));
        sig_names.insert(libc::SIGINT, String::from("Interrupt"));
        sig_names.insert(libc::SIGKILL, String::from("Killed"));
        sig_names.insert(libc::SIGPIPE, String::from("Broken pipe"));
        sig_names.insert(libc::SIGQUIT, String::from("Quit"));
        sig_names.insert(libc::SIGSEGV, String::from("Segmentation fault"));
        sig_names.insert(libc::SIGSTOP, String::from("Stopped (signal)"));
        sig_names.insert(libc::SIGTERM, String::from("Terminated"));
        sig_names.insert(libc::SIGTSTP, String::from("Stopped"));
        sig_names.insert(libc::SIGTTIN, String::from("Stopped (tty input)"));
        sig_names.insert(libc::SIGTTOU, String::from("Stopped (tty output)"));
        sig_names.insert(libc::SIGUSR1, String::from("User defined signal 1"));
        sig_names.insert(libc::SIGUSR2, String::from("User defined signal 2"));
        sig_names.insert(libc::SIGPROF, String::from("Profiling timer expired"));
        sig_names.insert(libc::SIGSYS, String::from("Bad system call"));
        sig_names.insert(libc::SIGTRAP, String::from("Trace/breakpoint trap"));
        sig_names.insert(libc::SIGURG, String::from("Urgent I/O condition"));
        sig_names.insert(libc::SIGVTALRM, String::from("Virtual timer expired"));
        sig_names.insert(libc::SIGXCPU, String::from("CPU time limit exceeded"));
        sig_names.insert(libc::SIGXFSZ, String::from("File size limit exceeded"));
        let mut special_builtin_fun_names: HashSet<String> = HashSet::new();
        special_builtin_fun_names.insert(String::from("."));
        special_builtin_fun_names.insert(String::from(":"));
        special_builtin_fun_names.insert(String::from("break"));
        special_builtin_fun_names.insert(String::from("continue"));
        special_builtin_fun_names.insert(String::from("eval"));
        special_builtin_fun_names.insert(String::from("exec"));
        special_builtin_fun_names.insert(String::from("exit"));
        special_builtin_fun_names.insert(String::from("export"));
        special_builtin_fun_names.insert(String::from("readonly"));
        special_builtin_fun_names.insert(String::from("return"));
        special_builtin_fun_names.insert(String::from("set"));
        special_builtin_fun_names.insert(String::from("times"));
        special_builtin_fun_names.insert(String::from("trap"));
        special_builtin_fun_names.insert(String::from("unset"));
        Interpreter {
            last_status: 0,
            non_simple_comamnd_count: 0,
            return_state: ReturnState::None,
            exec_redirect_flag: false,
            loop_count_stack: Vec::new(),
            current_loop_count: 0,
            fun_count: 0,
            last_job_pid: None,
            signal_names: sig_names,
            special_builtin_fun_names,
        }
    }

    pub fn last_status(&self) -> i32
    { self.last_status }

    pub fn has_none(&self) -> bool
    { self.return_state == ReturnState::None }

    pub fn has_break_with(&self, n: usize) -> bool
    { self.return_state == ReturnState::Break(n) }

    pub fn has_continue_with(&self, n: usize) -> bool
    { self.return_state == ReturnState::Continue(n) }
    
    pub fn has_return(&self) -> bool
    { self.return_state == ReturnState::Return }
    
    pub fn has_exit_with(&self, is_interactive: bool) -> bool
    { self.return_state == ReturnState::Exit(is_interactive) }

    pub fn has_continue_with_one(&self) -> bool
    {
        match self.return_state {
            ReturnState::Continue(1) => true,
            _ => false,
        }
    }
    
    pub fn has_break_or_continue(&self) -> bool
    {
        match self.return_state {
            ReturnState::Break(_) | ReturnState::Continue(_) => true,
            _ => false,
        }
    }
    
    pub fn has_break_or_continue_or_return(&self) -> bool
    {
        match self.return_state {
            ReturnState::Break(_) | ReturnState::Continue(_) | ReturnState::Return => true,
            _ => false,
        }
    }
    
    pub fn has_break_or_continue_or_return_or_exit(&self) -> bool
    {
        match self.return_state {
            ReturnState::Break(_) |  ReturnState::Continue(_) | ReturnState::Return | ReturnState::Exit(_) => true,
            _ => false,
        }
    }
    
    pub fn has_exit(&self) -> bool
    {
        match self.return_state {
            ReturnState::Exit(_) => true,
            _ => false,
        }
    }

    pub fn has_exit_with_interactive(&self) -> bool
    {
        match self.return_state {
            ReturnState::Exit(true) => true,
            _ => false,
        }
    }

    pub fn exit(&mut self, status: i32, is_interactive: bool) -> i32
    {
        self.return_state = ReturnState::Exit(is_interactive);
        status
    }

    pub fn ret(&mut self, status: i32) -> i32
    {
        self.return_state = ReturnState::Return;
        status
    }
    
    pub fn brk(&mut self, n: usize) -> i32
    {
        self.return_state = ReturnState::Break(n);
        0
    }
    
    pub fn cont(&mut self, n: usize) -> i32
    {
        self.return_state = ReturnState::Continue(n);
        0
    }

    pub fn set_break(&mut self, n: usize)
    { self.return_state = ReturnState::Break(n); }

    pub fn set_continue(&mut self, n: usize)
    { self.return_state = ReturnState::Continue(n); }
    
    pub fn set_return(&mut self)
    { self.return_state = ReturnState::Return; }
    
    pub fn set_exit(&mut self, is_interactive: bool)
    { self.return_state = ReturnState::Exit(is_interactive); }
    
    pub fn clear_return_state(&mut self)
    { self.return_state = ReturnState::None; }

    pub fn clear_return_state_for_break_or_continue(&mut self)
    {
        match self.return_state {
            ReturnState::Break(n) if n > 1 => self.return_state = ReturnState::Break(n - 1),
            ReturnState::Continue(n) if n > 1 => self.return_state = ReturnState::Continue(n - 1),
            _ => self.return_state = ReturnState::None,
        }
    }
    
    pub fn exec_redirect_flag(&self) -> bool
    { self.exec_redirect_flag }
    
    pub fn set_exec_redirect_flag(&mut self)
    { self.exec_redirect_flag = true; }
    
    pub fn is_in_loop(&self) -> bool
    { self.current_loop_count > 0 }
    
    pub fn increase_current_loop_count(&mut self) 
    { self.current_loop_count += 1; }

    pub fn decrease_current_loop_count(&mut self) 
    { self.current_loop_count -= 1; }
    
    pub fn is_in_fun(&self) -> bool
    { self.fun_count > 0 }
    
    pub fn increase_fun_count(&mut self) 
    { self.fun_count += 1; }

    pub fn decrease_fun_count(&mut self) 
    { self.fun_count -= 1; }

    fn push_loop_count(&mut self, count: usize)
    {
        self.loop_count_stack.push(self.current_loop_count);
        self.current_loop_count = count;
    }
    
    fn pop_loop_count(&mut self)
    {
        match self.loop_count_stack.pop() {
            Some(count) => self.current_loop_count = count,
            None => (),
        }
    }
    
    fn signal_name(&self, sig: i32) -> Option<&str>
    { self.signal_names.get(&sig).map(|s| s.as_str()) }

    fn signal_string(&self, sig: i32, is_coredump: bool) -> String
    {
        let coredump_s = if is_coredump {
            " (coredump)"
        } else {
            ""
        };
        format!("{}{}", self.signal_name(sig).unwrap_or("Unknown signal"), coredump_s)
    }

    fn has_special_builtin_fun(&self, name: &str, env: &Environment) -> bool
    { self.special_builtin_fun_names.contains(&String::from(name)) && env.builtin_fun(name).is_some() }
    
    fn execute(&mut self, exec: &mut Executor, vars: &[(String, String)], arg0: &str, args: &[String], is_exit_for_err: bool, env: &mut Environment, settings: &mut Settings) -> Option<i32>
    {
        match exec.execute(self, vars, arg0, args, false, env, settings, |_| true) {
            Ok(WaitStatus::None) => panic!("wait status is none"),
            Ok(WaitStatus::Exited(status)) => Some(status),
            Ok(WaitStatus::Signaled(sig, is_coredump)) => {
                if is_exit_for_err {
                    xsfprintln!(exec, 2, "{}", self.signal_string(sig, is_coredump));
                } else {
                    xcfprintln!(exec, 2, "{}", self.signal_string(sig, is_coredump));
                }
                Some(sig + 128)
            },
            Ok(WaitStatus::Stopped(_)) => panic!("wait status is stopped"),
            Err(err) => {
                xcfprintln!(exec, 2, "{}", err);
                None
            }
        }
    }
    
    pub fn wait_for_process(&mut self, exec: &mut Executor, pid: Option<i32>, is_exit_for_err: bool) -> Option<i32>
    {
        match exec.wait_for_process(pid, true, false) {
            Ok(WaitStatus::None) => panic!("wait status is none"),
            Ok(WaitStatus::Exited(status)) => Some(status),
            Ok(WaitStatus::Signaled(sig, is_coredump)) => {
                if is_exit_for_err {
                    xsfprintln!(exec, 2, "{}", self.signal_string(sig, is_coredump));
                } else {
                    xcfprintln!(exec, 2, "{}", self.signal_string(sig, is_coredump));
                }
                Some(sig + 128)
            },
            Ok(WaitStatus::Stopped(_)) => panic!("wait status is stopped"),
            Err(err) => {
                xcfprintln!(exec, 2, "{}", err);
                None
            },
        }
    }
    
    pub fn param(&self, exec: &Executor, param_name: &ParameterName, env: &Environment, settings: &Settings) -> Option<Value>
    {
        match param_name {
            ParameterName::Variable(name) => env.var(name.as_str()).map(|s| Value::String(s)),
            ParameterName::Argument(n) => {
                if *n > 0 {
                    settings.current_args().args().get(n - 1).map(|s| Value::String(s.clone()))
                } else {
                    Some(Value::String(settings.arg0.clone()))
                }
            },
            ParameterName::Special(SpecialParameterName::At) => Some(Value::AtArray(settings.current_args().args().iter().map(|s| s.clone()).collect())),
            ParameterName::Special(SpecialParameterName::Star) => Some(Value::StarArray(settings.current_args().args().iter().map(|s| s.clone()).collect())),
            ParameterName::Special(SpecialParameterName::Hash) => Some(Value::String(format!("{}", settings.current_args().args().len()))),
            ParameterName::Special(SpecialParameterName::Ques) => Some(Value::String(format!("{}", self.last_status))),
            ParameterName::Special(SpecialParameterName::Minus) => Some(Value::String(settings.option_string())),
            ParameterName::Special(SpecialParameterName::Dolar) => Some(Value::String(format!("{}", exec.shell_pid()))),
            ParameterName::Special(SpecialParameterName::Excl) => self.last_job_pid.map(|pid| Value::String(format!("{}", pid))),
        }
    }
    
    fn param_to_string(&mut self, exec: &Executor, param_name: &ParameterName, env: &Environment, settings: &Settings) -> Option<Option<String>>
    {
        match self.param(exec, param_name, env, settings) {
            Some(Value::String(s)) => Some(Some(s)),
            Some(Value::AtArray(ss)) => {
                let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
                let sep = match ifs.chars().next() {
                    Some(c) => {
                        let mut tmp_sep = String::new();
                        tmp_sep.push(c);
                        tmp_sep
                    },
                    None => String::new(),
                };
                Some(Some(ss.join(sep.as_str())))
            },
            Some(Value::StarArray(ss)) => {
                let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
                let sep = match ifs.chars().next() {
                    Some(c) => {
                        let mut tmp_sep = String::new();
                        tmp_sep.push(c);
                        tmp_sep
                    },
                    None => String::new(),
                };
                Some(Some(ss.join(sep.as_str())))
            },
            Some(Value::ExpansionArray(ss)) => {
                let ts = self.unescape_strings(exec, ss.as_slice(), settings)?;
                let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
                let sep = match ifs.chars().next() {
                    Some(c) => {
                        let mut tmp_sep = String::new();
                        tmp_sep.push(c);
                        tmp_sep
                    },
                    None => String::new(),
                };
                Some(Some(ts.join(sep.as_str())))
            },
            None => Some(None),
        }
    }
    
    fn perform_param_expansion(&mut self, exec: &mut Executor, param_name: &ParameterName, modifier_and_words: &Option<(ParameterModifier, Vec<Rc<Word>>)>, env: &mut Environment, settings: &mut Settings) -> Option<Option<Value>>
    {
        match modifier_and_words {
            None => {
                match self.param(exec, param_name, env, settings) {
                    Some(value) => Some(Some(value)),
                    None => {
                        if !settings.nounset_flag {
                            Some(None)
                        } else {
                            xsfprintln!(exec, 2, "{}: Parameter not set", param_name);
                            self.set_exit(false);
                            None
                        }
                    },
                }
            },
            Some((ParameterModifier::ColonMinus, words)) => {
                match self.perform_pattern_word_expansions(exec, words.as_slice(), env, settings) {
                    Some(new_words) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    Some(Some(Value::ExpansionArray(new_words)))
                                }
                            },
                            None => Some(Some(Value::ExpansionArray(new_words))),
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::Minus, words)) => {
                match self.perform_pattern_word_expansions(exec, words.as_slice(), env, settings) {
                    Some(new_words) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    Some(Some(Value::String(String::new())))
                                }
                            },
                            None => Some(Some(Value::ExpansionArray(new_words))),
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::ColonEqual, words)) => {
                match self.perform_var_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(word) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    if set_param(param_name, word.as_str(), env, settings) {
                                        Some(Some(Value::String(word)))
                                    } else {
                                        if is_read_only_param(param_name, env) {
                                            xsfprintln!(exec, 2, "{}: Is read only", param_name);
                                        } else {
                                            xsfprintln!(exec, 2, "{}: Can't set parameter", param_name);
                                        }
                                        self.set_exit(false);
                                        None
                                    }
                                }
                            },
                            None => {
                                if set_param(param_name, word.as_str(), env, settings) {
                                    Some(Some(Value::String(word)))
                                } else {
                                    if is_read_only_param(param_name, env) {
                                        xsfprintln!(exec, 2, "{}: Is read only", param_name);
                                    } else {
                                        xsfprintln!(exec, 2, "{}: Can't set parameter", param_name);
                                    }
                                    self.set_exit(false);
                                    None
                                }
                            },
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::Equal, words)) => {
                match self.perform_var_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(word) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    Some(Some(Value::String(String::new())))
                                }
                            },
                            None => {
                                if set_param(param_name, word.as_str(), env, settings) {
                                    Some(Some(Value::String(word)))
                                } else {
                                    if is_read_only_param(param_name, env) {
                                        xsfprintln!(exec, 2, "{}: Is read only", param_name);
                                    } else {
                                        xsfprintln!(exec, 2, "{}: Can't set parameter", param_name);
                                    }
                                    self.set_exit(false);
                                    None
                                }
                            },
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::ColonQues, words)) => {
                match self.perform_var_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(word) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    let err = if !word.is_empty() {
                                        word
                                    } else {
                                        String::from("Parameter null or not set")
                                    };
                                    xsfprintln!(exec, 2, "{}: {}", param_name, err);
                                    self.set_exit(false);
                                    None
                                }
                            },
                            None => {
                                let err = if !word.is_empty() {
                                   word
                                } else {
                                    String::from("Parameter null or not set")
                                };
                                xsfprintln!(exec, 2, "{}: {}", param_name, err);
                                self.set_exit(false);
                                None
                            },
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::Ques, words)) => {
                match self.perform_var_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(word) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(value))
                                } else {
                                    Some(Some(Value::String(String::new())))
                                }
                            },
                            None => {
                                let err = if !word.is_empty() {
                                   word
                                } else {
                                    String::from("Parameter not set")
                                };
                                xsfprintln!(exec, 2, "{}: {}", param_name, err);
                                self.set_exit(false);
                                None
                            },
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::ColonPlus, words)) => {
                match self.perform_pattern_word_expansions(exec, words.as_slice(), env, settings) {
                    Some(new_words) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(value) => {
                                if !value.is_null() {
                                    Some(Some(Value::ExpansionArray(new_words)))
                                } else {
                                    Some(Some(Value::String(String::new())))
                                }
                            },
                            None => Some(Some(Value::String(String::new()))),
                        }
                    },
                    None => None,
                }
            },
            Some((ParameterModifier::Plus, words)) => {
                match self.perform_pattern_word_expansions(exec, words.as_slice(), env, settings) {
                    Some(new_words) => {
                        match self.param(exec, param_name, env, settings) {
                            Some(_) => Some(Some(Value::ExpansionArray(new_words))),
                            None => Some(Some(Value::String(String::new()))),
                        }
                    },
                    None => None,
                }
            },
            Some((modifier @ (ParameterModifier::Perc | ParameterModifier::PercPerc), words)) => {
                match self.perform_pattern_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(pattern) => {
                        let s = (self.param_to_string(exec, param_name, env, settings)?).unwrap_or(String::new());
                        if !s.is_empty() {
                            let mut is: Vec<usize> = s.char_indices().map(|p| p.0).collect();
                            is.push(s.len());
                            let mut t = s.as_str();
                            if modifier == &ParameterModifier::Perc {
                                is.reverse();
                            }
                            for i in &is {
                                if fnmatch(&pattern, &s[(*i)..], 0) {
                                    t = &s[..(*i)];
                                    break;
                                }
                            }
                            Some(Some(Value::String(String::from(t))))
                        } else {
                            Some(Some(Value::String(s)))
                        }
                    },
                    None => None,
                }
            },
            Some((modifier @ (ParameterModifier::Hash | ParameterModifier::HashHash), words)) => {
                match self.perform_pattern_word_expansions_as_string(exec, words.as_slice(), env, settings) {
                    Some(pattern) => {
                        let s = (self.param_to_string(exec, param_name, env, settings)?).unwrap_or(String::new());
                        if !s.is_empty() {
                            let mut is: Vec<usize> = s.char_indices().map(|p| p.0).collect();
                            is.push(s.len());
                            let mut t = s.as_str();
                            if modifier != &ParameterModifier::Hash {
                                is.reverse();
                            }
                            for i in &is {
                                if fnmatch(&pattern, &s[..(*i)], 0) {
                                    t = &s[(*i)..];
                                    break;
                                }
                            }
                            Some(Some(Value::String(String::from(t))))
                        } else {
                            Some(Some(Value::String(s)))
                        }
                    },
                    None => None,
                }
            },
        }
    }

    fn perform_param_len_expansion(&mut self, exec: &Executor, param_name: &ParameterName, env: &Environment, settings: &Settings) -> Option<String>
    {
        match self.param_to_string(exec, param_name, env, settings) {
            Some(Some(s)) => Some(format!("{}", s.len())),
            Some(None) => {
                if !settings.nounset_flag {
                    Some(String::from("0"))
                } else {
                    xsfprintln!(exec, 2, "{}: Parameter not set", param_name);
                    self.set_exit(false);
                    None
                }
            },
            None => None,
        }
    }
    
    fn substitute_command(&mut self, exec: &mut Executor, commands: &[Rc<LogicalCommand>], env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        exec.interpret(|exec| {
                let mut pipes: Vec<Pipe> = Vec::new();
                let mut is_success = true;
                let mut pid: Option<i32> = None;
                let mut s = String::new();
                match pipe_with_cloexec() {
                    Ok(pipe_fds) => pipes.push(unsafe { Pipe::from_pipe_fds(&pipe_fds) }),
                    Err(err) => {
                        xsfprintln!(exec, 2, "{}", err);
                        is_success = false;
                    },
                }
                exec.set_pipes(pipes);
                if is_success {
                    let res = exec.create_process(false, settings, |exec, settings| {
                            exec.push_file(1, exec.pipes()[0].writing_file.clone());
                            exec.clear_pipes();
                            self.push_loop_count(0);
                            let status = exec.interpret_or(commands.len() > 1, |exec| {
                                    self.interpret_logical_commands(exec, commands, env, settings)
                            });
                            self.pop_loop_count();
                            status
                    });
                    match res {
                        Ok(tmp_pid) => pid = tmp_pid,
                        Err(err) => {
                            xsfprintln!(exec, 2, "{}", err);
                            is_success = false;
                        },
                    }
                }
                if is_success {
                    let file = exec.pipes()[0].reading_file.clone();
                    exec.clear_pipes();
                    let mut file_r = file.borrow_mut();
                    match file_r.read_to_string(&mut s) {
                        Ok(_) => (),
                        Err(err) => {
                            xsfprintln!(exec, 2, "{}", err);
                            is_success = false;
                        },
                    }
                    s = String::from(s.trim_end_matches('\n'));
                    match self.wait_for_process(exec, pid, true) {
                        Some(status) => self.last_status = status,
                        None => is_success = false,
                    }
                } else {
                    exec.clear_pipes();
                }
                if is_success {
                    Some(s)
                } else {
                    self.set_exit(false);
                    None
                }
        })
    }

    fn assign_to_arith_expr(&mut self, exec: &Executor, expr: &ArithmeticExpression, x: i64, env: &mut Environment, settings: &Settings) -> Option<i64>
    {
        match expr {
            ArithmeticExpression::Parameter(_, _, param_name) => {
                if set_param(param_name, format!("{}", x).as_str(), env, settings) {
                    Some(x)
                } else {
                    if is_read_only_param(param_name, env) {
                        xsfprintln!(exec, 2, "{}: Is read only", param_name);
                    } else {
                        xsfprintln!(exec, 2, "{}: Can't set parameter", param_name);
                    }
                    self.set_exit(false);
                    None
                }
            },
            _ => {
                xsfprintln!(exec, 2, "Can't assign to not parameter");
                self.set_exit(false);
                None
            },
        }
    }

    fn evaluate_arith_expr(&mut self, exec: &Executor, expr: &ArithmeticExpression, env: &mut Environment, settings: &Settings) -> Option<i64>
    {
        match expr {
            ArithmeticExpression::Number(_, _, x) => Some(*x),
            ArithmeticExpression::Parameter(_, _, param_name) => {
                match self.param_to_string(exec, param_name, env, settings)? {
                    Some(s) => {
                        if !s.is_empty() {
                            if is_number_str(s.as_str()) {
                                match str_to_number(s.as_str()) {
                                    Ok(x) => Some(x),
                                    Err(_) => {
                                        if !s.starts_with('-') {
                                            xsfprintln!(exec, 2, "{}: Too large number", param_name);
                                        } else {
                                            xsfprintln!(exec, 2, "{}: Too small number", param_name);
                                        }
                                        self.set_exit(false);
                                        None
                                    },
                                }
                            } else {
                                xsfprintln!(exec, 2, "{}: Invalid number", param_name);
                                None
                            }
                        } else {
                            Some(0)
                        }
                    },
                    None => {
                        if !settings.nounset_flag {
                            Some(0)
                        } else {
                            xsfprintln!(exec, 2, "{}: Parameter not set", param_name);
                            self.set_exit(false);
                            None
                        }
                    },
                }
            },
            ArithmeticExpression::Unary(_, _, UnaryOperator::Negate, expr1) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                match x.checked_neg() {
                    Some(y) => Some(y),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Unary(_, _, UnaryOperator::Not, expr1) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                Some(!x)
            },
            ArithmeticExpression::Unary(_, _, UnaryOperator::LogicalNot, expr1) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                if x != 0 {
                    Some(0)
                } else {
                    Some(1)
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Multiply, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_mul(y) {
                    Some(z) => Some(z),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Divide, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y != 0 {
                    match x.checked_div(y) {
                        Some(z) => Some(z),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Division by zero");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Module, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y != 0 {
                    match x.checked_rem(y) {
                        Some(z) => Some(z),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Division by zero");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Add, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_add(y) {
                    Some(z) => Some(z),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Substract, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_sub(y) {
                    Some(z) => Some(z),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ShiftLeft, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y <= u32::MAX as i64 && y >= 0 {
                    match x.checked_shl(y as u32) {
                        Some(z) => Some(z),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Overflow");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ShiftRight, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y <= u32::MAX as i64 && y >= 0 {
                    match x.checked_shl(y as u32) {
                        Some(z) => Some(z),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Overflow");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::LessThan, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x < y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::GreaterEqual, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x >= y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::GreaterThan, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x > y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::LessEqual, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x <= y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Equal, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x == y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::NotEqual, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(if x != y { 1 } else { 0 })
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::And, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(x & y)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ExclusiveOr, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(x ^ y)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Or, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                Some(x | y)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::LogicalAnd, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                if x != 0 {
                    let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                    Some(y)
                } else {
                    Some(x)
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::LogicalOr, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                if x == 0 {
                    let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                    Some(y)
                } else {
                    Some(x)
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::Assign, expr2) => {
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                self.assign_to_arith_expr(exec, &(*expr1), y, env, settings)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::MultiplyAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_mul(y) {
                    Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::DivideAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y != 0 {
                    match x.checked_div(y) {
                        Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Division by zero");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ModuleAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y != 0 {
                    match x.checked_rem(y) {
                        Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Division by zero");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::AddAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_add(y) {
                    Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::SubstractAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                match x.checked_sub(y) {
                    Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                    None => {
                        xsfprintln!(exec, 2, "Overflow");
                        self.set_exit(false);
                        None
                    },
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ShiftLeftAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y <= u32::MAX as i64 && y >= 0 {
                    match x.checked_shl(y as u32) {
                        Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Overflow");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ShiftRightAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                if y <= u32::MAX as i64 && y >= 0 {
                    match x.checked_shl(y as u32) {
                        Some(z) => self.assign_to_arith_expr(exec, &(*expr1), z, env, settings),
                        None => {
                            xsfprintln!(exec, 2, "Overflow");
                            self.set_exit(false);
                            None
                        },
                    }
                } else {
                    xsfprintln!(exec, 2, "Overflow");
                    self.set_exit(false);
                    None
                }
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::AndAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                self.assign_to_arith_expr(exec, &(*expr1), x & y, env, settings)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::ExclusiveOrAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                self.assign_to_arith_expr(exec, &(*expr1), x ^ y, env, settings)
            },
            ArithmeticExpression::Binary(_, _, expr1, BinaryOperator::OrAssign, expr2) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                let y = self.evaluate_arith_expr(exec, &(*expr2), env, settings)?;
                self.assign_to_arith_expr(exec, &(*expr1), x | y, env, settings)
            },
            ArithmeticExpression::Conditional(_, _, expr1, expr2, expr3) => {
                let x = self.evaluate_arith_expr(exec, &(*expr1), env, settings)?;
                if x != 0 {
                    self.evaluate_arith_expr(exec, &(*expr2), env, settings)
                } else {
                    self.evaluate_arith_expr(exec, &(*expr3), env, settings)
                }
            },
        }
    }
    
    fn perform_arith_expansion(&mut self, exec: &Executor, expr: &ArithmeticExpression, env: &mut Environment, settings: &Settings) -> Option<String>
    {
        match self.evaluate_arith_expr(exec, expr, env, settings) {
            Some(x) => Some(format!("{}", x)),
            None => None,
        }
    }
    
    fn add_simple_word_elem_expansions(&mut self, exec: &mut Executor, elems: &[SimpleWordElement], ss: &mut Vec<String>, is_here_doc: bool, env: &mut Environment, settings: &mut Settings) -> bool
    {
        let mut is_empty = true;
        for elem in elems.iter() {
            let mut ts: Vec<String> = Vec::new();
            let mut is_join = false;
            let mut is_unescaping = false;
            match elem {
                SimpleWordElement::String(s) => ts.push(s.clone()),
                SimpleWordElement::Parameter(param_name, modifier_and_words) => {
                    match self.perform_param_expansion(exec, param_name, modifier_and_words, env, settings) {
                        Some(Some(Value::String(s))) => ts.push(s.clone()),
                        Some(Some(Value::AtArray(ss))) => {
                            ts.extend(ss);
                            is_join = is_here_doc;
                        },
                        Some(Some(Value::StarArray(ss))) => {
                            ts.extend(ss);
                            is_join = true;
                        },
                        Some(Some(Value::ExpansionArray(ss))) => {
                            ts.extend(ss);
                            is_join = is_here_doc;
                            is_unescaping = true;
                        },
                        Some(None) => ts.push(String::new()),
                        None => return false,
                    }
                },
                SimpleWordElement::ParameterLength(param_name) => {
                    match self.perform_param_len_expansion(exec, param_name, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                },
                SimpleWordElement::Command(commands) => {
                    match self.substitute_command(exec, commands, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                },
                SimpleWordElement::ArithmeticExpression(expr) => {
                    match self.perform_arith_expansion(exec, expr, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                },
            }
            if is_unescaping {
                match self.unescape_strings(exec, ts.as_slice(), settings) {
                    Some(us) => ts = us,
                    None => return false,
                }
            }
            if is_join {
                let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
                let sep = match ifs.chars().next() {
                    Some(c) => {
                        let mut tmp_sep = String::new();
                        tmp_sep.push(c);
                        tmp_sep
                    },
                    None => String::new(),
                };
                ts = vec![ts.join(sep.as_str())];
            }
            if !is_here_doc {
                ts = ts.iter().map(|s| escape_str(s.as_str())).collect();
            }
            if !is_empty {
                if !ts.is_empty() {
                    let ss_len = ss.len();
                    ss[ss_len - 1].push_str(ts[0].as_str());
                    ss.extend_from_slice(&ts[1..]);
                }
            } else {
                is_empty = ts.is_empty();
                ss.extend(ts);
            }
        }
        true
    }

    fn add_word_elem_expansions(&mut self, exec: &mut Executor, elems: &[WordElement], ss: &mut Vec<String>, env: &mut Environment, settings: &mut Settings) -> bool
    {
        let mut is_first = true;
        let mut is_empty = true;
        let is_one_elem = elems.len() == 1;
        let mut is_last_s_to_pop = false;
        for elem in elems.iter() {
            let mut ts: Vec<String> = Vec::new();
            let mut is_split = false;
            match elem {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    let mut tylda_with_sep = String::from("~");
                    tylda_with_sep.push(path::MAIN_SEPARATOR);
                    let mut sep = String::new();
                    sep.push(path::MAIN_SEPARATOR);
                    if is_first && s.starts_with(tylda_with_sep.as_str()) {
                        let mut t = String::new();
                        t.push_str(escape_str(env.var("HOME").unwrap_or(sep).as_str()).as_str());
                        t.push(path::MAIN_SEPARATOR);
                        t.push_str(&s[2..]);
                        ts.push(t);
                    } else if is_first && s == &String::from("~") && is_one_elem {
                        ts.push(escape_str(env.var("HOME").unwrap_or(sep).as_str()));
                    } else {
                        ts.push(s.clone());
                    }
                },
                WordElement::Simple(SimpleWordElement::Parameter(param_name, modifier_and_words)) => {
                    match self.perform_param_expansion(exec, param_name, modifier_and_words, env, settings) {
                        Some(Some(Value::String(s))) => {
                            ts.push(s.clone());
                            is_split = true;
                        },
                        Some(Some(Value::AtArray(ss))) => {
                            ts.extend(ss);
                            is_split = true;
                        },
                        Some(Some(Value::StarArray(ss))) => {
                            ts.extend(ss);
                            is_split = true;
                        },
                        Some(Some(Value::ExpansionArray(ss))) => ts.extend(ss),
                        Some(None) => is_split = true,
                        None => return false,
                    }
                },
                WordElement::Simple(SimpleWordElement::ParameterLength(param_name)) => {
                    match self.perform_param_len_expansion(exec, param_name, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                    is_split = true;
                },
                WordElement::Simple(SimpleWordElement::Command(commands)) => {
                    match self.substitute_command(exec, commands, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                    is_split = true;
                },
                WordElement::Simple(SimpleWordElement::ArithmeticExpression(expr)) => {
                    match self.perform_arith_expansion(exec, expr, env, settings) {
                        Some(s) => ts.push(s),
                        None => return false,
                    }
                    is_split = true;
                },
                WordElement::SinglyQuoted(s) => ts.push(escape_str(s.as_str())),
                WordElement::DoublyQuoted(simple_elems) => {
                    if !simple_elems.is_empty() {
                        if !self.add_simple_word_elem_expansions(exec, simple_elems, &mut ts, false, env, settings) {
                            return false;
                        }
                    } else {
                        ts.push(String::new());
                    }
                },
            }
            if is_split {
                let ifs = env.var("IFS").unwrap_or(String::from(DEFAULT_IFS));
                let mut us: Vec<String> = Vec::new();
                let is_space = ifs.chars().any(char::is_whitespace);
                let spaces = ifs.replace(|c: char| !c.is_whitespace(), "");
                if !ifs.is_empty() {
                    for t in &ts {
                        let mut vs: Vec<String> = split_str_for_ifs(t.as_str(), ifs.as_str()).iter().map(|s| String::from(*s)).collect();
                        match vs.last() {
                            Some(s) if s.is_empty() => {
                                vs.pop();
                            },
                            _ => (),
                        }
                        us.extend(vs);
                    }
                } else {
                    for t in &ts {
                        if !t.is_empty() {
                            us.push(t.clone());
                        }
                    }
                }
                let mut tmp_ts: Vec<String> = Vec::new();
                match ts.first() {
                    Some(s) if !is_empty && is_space && is_first_char(s, spaces.as_str()) && us.first().map(|t| !t.is_empty()).unwrap_or(false) => tmp_ts.push(String::new()),
                    _ => (),
                }
                tmp_ts.extend(us);
                match ts.last() {
                    Some(s) if is_space && is_last_char(s, spaces.as_str()) => {
                        tmp_ts.push(String::new());
                        is_last_s_to_pop = true;
                    },
                    _ => is_last_s_to_pop = false,
                }
                ts = tmp_ts;
            } else {
                is_last_s_to_pop = false;
            }
            if !is_empty {
                if !ts.is_empty() {
                    let ss_len = ss.len();
                    ss[ss_len - 1].push_str(ts[0].as_str());
                    ss.extend_from_slice(&ts[1..]);
                }
            } else {
                is_empty = ts.is_empty();
                ss.extend(ts);
            }
            is_first = false;
        }
        if is_last_s_to_pop {
            ss.pop();
        }
        true
    }
    
    fn add_glob_expansions(&mut self, exec: &Executor, ss: &[String], ts: &mut Vec<String>, settings: &mut Settings) -> bool
    {
        for s in ss.iter() {
            if !settings.noglob_flag {
                match glob(s, 0, None) {
                    GlobResult::Ok(path_bufs) => {
                        for path_buf in &path_bufs {
                            let t = if settings.strlossy_flag {
                                path_buf.to_string_lossy().into_owned()
                            } else {
                                match path_buf.to_str() {
                                    Some(t) => String::from(t),
                                    None => {
                                        xsfprintln!(exec, 2, "Invalid UTF-8");
                                        self.set_exit(false);
                                        return false;
                                    },
                                }
                            };
                            ts.push(t);
                        }
                    },
                    GlobResult::Aborted => {
                        xsfprintln!(exec, 2, "Glob I/O error");
                        self.set_exit(false);
                        return false;
                    },
                    GlobResult::NoMatch => {
                        let path_buf = unescape_path_pattern(s);
                        let t = if settings.strlossy_flag {
                            path_buf.to_string_lossy().into_owned()
                        } else {
                            match path_buf.to_str() {
                                Some(t) => String::from(t),
                                None => {
                                    xsfprintln!(exec, 2, "Invalid UTF-8");
                                    self.set_exit(false);
                                    return false;
                                },
                            }
                        };
                        ts.push(t);
                    },
                    GlobResult::NoSpace => {
                        xsfprintln!(exec, 2, "Can't allocate memory");
                        self.set_exit(false);
                        return false;
                    },
                }
            } else {
                let path_buf = unescape_path_pattern(s);
                let t = if settings.strlossy_flag {
                    path_buf.to_string_lossy().into_owned()
                } else {
                    match path_buf.to_str() {
                        Some(t) => String::from(t),
                        None => {
                            xsfprintln!(exec, 2, "Invalid UTF-8");
                            self.set_exit(false);
                            return false;
                        },
                    }
                };
                ts.push(t);
            }
        }
        true
    }

    fn unescape_strings(&mut self, exec: &Executor, ss: &[String], settings: &Settings) -> Option<Vec<String>>
    {
        let mut ts: Vec<String> = Vec::new();
        for s in ss.iter() {
            let path_buf = unescape_path_pattern(s);
            let t = if settings.strlossy_flag {
                path_buf.to_string_lossy().into_owned()
            } else {
                match path_buf.to_str() {
                    Some(t) => String::from(t),
                    None => {
                        xsfprintln!(exec, 2, "Invalid UTF-8");
                        self.set_exit(false);
                        return None;
                    },
                }
            };
            ts.push(t);
        }
        Some(ts)
    }    
    
    fn perform_var_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        let mut ss: Vec<String> = Vec::new();
        if !self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
            return None;
        }
        match self.unescape_strings(exec, ss.as_slice(), settings) {
            Some(ts) => Some(ts.join(" ")),
            None => None,
        }
    }

    fn perform_var_word_expansions_as_string(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        let mut ss: Vec<String> = Vec::new();
        for word in words.iter() {
            if !self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
                return None;
            }
        }
        match self.unescape_strings(exec, ss.as_slice(), settings) {
            Some(ts) => Some(ts.join(" ")),
            None => None,
        }
    }    
    
    fn perform_pattern_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        let mut ss: Vec<String> = Vec::new();
        if self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
            Some(ss.join(" "))
        } else {
            None
        }
    }

    fn perform_pattern_word_expansions(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<Vec<String>>
    {
        let mut ss: Vec<String> = Vec::new();
        for word in words.iter() {
            if !self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
                return None;
            }
        }
        Some(ss)
    }
    
    fn perform_pattern_word_expansions_as_string(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        match self.perform_pattern_word_expansions(exec, words, env, settings) {
            Some(ss) => Some(ss.join(" ")),
            None => None,
        }
    }    
    
    fn perform_word_expansion(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<Vec<String>>
    {
        let mut ss: Vec<String> = Vec::new();
        if !self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
            return None;
        }
        let mut ts: Vec<String> = Vec::new();
        if self.add_glob_expansions(exec, &ss, &mut ts, settings) {
            Some(ts)
        } else {
            None
        }
    }
        
    fn perform_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
       match self.perform_word_expansion(exec, word, env, settings) {
           Some(ss) => Some(ss.join(" ")),
           None => None,
       }
    }

    fn perform_word_expansions(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<Vec<String>>
    {
        let mut ss: Vec<String> = Vec::new();
        for word in words {
            if !self.add_word_elem_expansions(exec, word.word_elems.as_slice(), &mut ss, env, settings) {
                return None;
            }
        }
        let mut ts: Vec<String> = Vec::new();
        if self.add_glob_expansions(exec, &ss, &mut ts, settings) {
            Some(ts)
        } else {
            None
        }
    }
    
    fn perform_here_doc_expansion(&mut self, exec: &mut Executor, here_doc: &HereDocument, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        let mut ss: Vec<String> = Vec::new();
        if !self.add_simple_word_elem_expansions(exec, &here_doc.simple_word_elems, &mut ss, true, env, settings) {
            return None;
        }
        Some(ss.join(""))
    }
    
    fn interpret_redirects<F>(&mut self, exec: &mut Executor, redirects: &[Rc<Redirection>], is_special_builtin_fun: bool, env: &mut Environment, settings: &mut Settings, f: F) -> i32
        where F: FnOnce(&mut Self, &mut Executor, &mut Environment, &mut Settings) -> i32
    {
        let mut is_success = true;
        let mut interp_redirects: Vec<InterpreterRedirection> = Vec::new();
        for redirect in redirects.iter() {
            match &(**redirect) {
                Redirection::Input(_, _, n, word) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Input(n.unwrap_or(0), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::Output(_, _, n, word, is_bar) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Output(n.unwrap_or(1), path, *is_bar)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::InputAndOutput(_, _, n, word) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::InputAndOutput(n.unwrap_or(0), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::Appending(_, _, n, word) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Appending(n.unwrap_or(1), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::InputDuplicating(path, pos, n, word) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(fd_s) => {
                            if is_io_number_str(fd_s.as_str()) {
                                match fd_s.parse::<i32>() {
                                    Ok(fd) => interp_redirects.push(InterpreterRedirection::Duplicating(n.unwrap_or(0), fd)),
                                    Err(_) => {
                                        if is_special_builtin_fun {
                                            xsfprintln!(exec, 2, "{}: {}: too large I/O number", path, pos);
                                        } else {
                                            xcfprintln!(exec, 2, "{}: {}: too large I/O number", path, pos);
                                        }
                                        is_success = false;
                                    },
                                }
                            } else {
                                if is_special_builtin_fun {
                                    xsfprintln!(exec, 2, "{}: {}: invalid I/O number", path, pos);
                                } else {
                                    xcfprintln!(exec, 2, "{}: {}: invalid I/O number", path, pos);
                                }
                                is_success = false;
                            }
                        },
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::OutputDuplicating(path, pos, n, word) => {
                    match self.perform_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(fd_s) => {
                            if is_io_number_str(fd_s.as_str()) {
                                match fd_s.parse::<i32>() {
                                    Ok(fd) => interp_redirects.push(InterpreterRedirection::Duplicating(n.unwrap_or(1), fd)),
                                    Err(_) => {
                                        if is_special_builtin_fun {
                                            xsfprintln!(exec, 2, "{}: {}: too large I/O number", path, pos);
                                        } else {
                                            xcfprintln!(exec, 2, "{}: {}: too large I/O number", path, pos);
                                        }
                                        is_success = false;
                                    },
                                }
                            } else {
                                if is_special_builtin_fun {
                                    xsfprintln!(exec, 2, "{}: {}: invalid I/O number", path, pos);
                                } else {
                                    xcfprintln!(exec, 2, "{}: {}: invalid I/O number", path, pos);
                                }
                                is_success = false;
                            }
                        },
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::HereDocument(_, _, n, here_doc) => {
                    match self.perform_here_doc_expansion(exec, &here_doc.borrow(), env, settings) {
                        Some(s) => interp_redirects.push(InterpreterRedirection::HereDocument(n.unwrap_or(0), s)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
            }
        }
        if !is_success {
            if is_special_builtin_fun {
                return self.exit(1, false);
            } else {
                return 1;
            }
        }
        let mut is_success_for_interp_redirects = true;
        let mut pipes: Vec<Pipe> = Vec::new();
        let mut i = 0;
        for interp_redirect in &interp_redirects {
            match interp_redirect {
                InterpreterRedirection::Input(vfd, path) => {
                    match File::open(path) {
                        Ok(file) => exec.push_file(*vfd, Rc::new(RefCell::new(file))),
                        Err(err) => {
                            if is_special_builtin_fun {
                                xsfprintln!(exec, 2, "{}: {}", path, err);
                            } else {
                                xcfprintln!(exec, 2, "{}: {}", path, err);
                            }
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::Output(vfd, path, is_bar) => {
                    let mut open_opts = OpenOptions::new();
                    open_opts.write(true);
                    if *is_bar || !settings.noclobber_flag {
                        open_opts.create(true).truncate(true);
                    } else {
                        open_opts.create_new(true);
                    }
                    match open_opts.open(path) {
                        Ok(file) => exec.push_file(*vfd, Rc::new(RefCell::new(file))),
                        Err(err) => {
                            if is_special_builtin_fun {
                                xsfprintln!(exec, 2, "{}: {}", path, err);
                            } else {
                                xcfprintln!(exec, 2, "{}: {}", path, err);
                            }
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::InputAndOutput(vfd, path) => {
                    let mut open_opts = OpenOptions::new();
                    open_opts.read(true).write(true);
                    open_opts.create(true);
                    match open_opts.open(path) {
                        Ok(file) => exec.push_file(*vfd, Rc::new(RefCell::new(file))),
                        Err(err) => {
                            if is_special_builtin_fun {
                                xsfprintln!(exec, 2, "{}: {}", path, err);
                            } else {
                                xcfprintln!(exec, 2, "{}: {}", path, err);
                            }
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::Appending(vfd, path) => {
                    let mut open_opts = OpenOptions::new();
                    open_opts.write(true).append(true);
                    open_opts.create(true);
                    match open_opts.open(path) {
                        Ok(file) => exec.push_file(*vfd, Rc::new(RefCell::new(file))),
                        Err(err) => {
                            if is_special_builtin_fun {
                                xsfprintln!(exec, 2, "{}: {}", path, err);
                            } else {
                                xcfprintln!(exec, 2, "{}: {}", path, err);
                            }
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::Duplicating(new_vfd, old_vfd) => {
                    let old_file = exec.current_file(*old_vfd).map(|f| f.clone());
                    match old_file {
                        Some(file) => exec.push_file(*new_vfd, file),
                        None => {
                            if is_special_builtin_fun {
                                xsfprintln!(exec, 2, "{}: Bad fd number", *old_vfd);
                            } else {
                                xcfprintln!(exec, 2, "{}: Bad fd number", *old_vfd);
                            }
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::HereDocument(_, _) => {
                    match pipe_with_cloexec() {
                        Ok(pipe_fds) => pipes.push(unsafe { Pipe::from_pipe_fds(&pipe_fds) }),
                        Err(err) => {
                            xsfprintln!(exec, 2, "{}", err);
                            is_success = false;
                            is_success_for_interp_redirects = false;
                            break;
                        }
                    }
                },
            }
            i += 1;
        }
        exec.set_pipes(pipes);
        let mut status = 1;
        if is_success {
            if exec.pipes().is_empty() {
                status = f(self, exec, env, settings);
            } else {
                status = exec.interpret(|exec| {
                        let mut j = 0;
                        let mut k = 0;
                        let mut pids: Vec<Option<i32>> = Vec::new(); 
                        for interp_redirect in &interp_redirects {
                            match interp_redirect {
                                InterpreterRedirection::HereDocument(_, s) => {
                                    let res = exec.create_process(false, settings, |exec, _| {
                                            let file = exec.pipes()[j].writing_file.clone();
                                            exec.clear_pipes();
                                            let mut file_r = file.borrow_mut();
                                            match file_r.write_all(s.as_bytes()) {
                                                Ok(()) => 0,
                                                Err(err) => {
                                                    if is_special_builtin_fun {
                                                        xsfprintln!(exec, 2, "{}", err);
                                                    } else {
                                                        xcfprintln!(exec, 2, "{}", err);
                                                    }
                                                    1
                                                },
                                            }
                                    });
                                    match res {
                                        Ok(pid) => pids.push(pid),
                                        Err(err) => {
                                            if is_special_builtin_fun {
                                                xsfprintln!(exec, 2, "{}", err);
                                            } else {
                                                xcfprintln!(exec, 2, "{}", err);
                                            }
                                            is_success = false;
                                            is_success_for_interp_redirects = false;
                                            break;
                                        },
                                    }
                                    j += 1;
                                },
                                _ => (),
                            }
                            k += 1;
                        }
                        let mut pid: Option<i32> = None;
                        if is_success {
                            let res = exec.create_process(false, settings, |exec, settings| {
                                    let mut l = 0;
                                    for interp_redirect in &interp_redirects {
                                        match interp_redirect {
                                            InterpreterRedirection::HereDocument(vfd, _) => {
                                                exec.push_file(*vfd, exec.pipes()[l].reading_file.clone());
                                                l += 1;
                                            },
                                            _ => (),
                                        }
                                    }
                                    exec.clear_pipes();
                                    let status = f(self, exec, env, settings);
                                    interp_redirects.reverse();
                                    for interp_redirect in &interp_redirects {
                                        match interp_redirect {
                                            InterpreterRedirection::HereDocument(vfd, _) => {
                                                l -= 1;
                                                exec.pop_file(*vfd);
                                            },
                                            _ => (),
                                        }
                                    }
                                    status
                            });
                            match res {
                                Ok(tmp_pid) => pid = tmp_pid,
                                Err(err) => {
                                    xcfprintln!(exec, 2, "{}", err);
                                    is_success = false;
                                },
                            }
                        }
                        exec.clear_pipes();
                        let mut tmp_is_success = true;
                        j = 0;
                        for interp_redirect in &interp_redirects[0..k] {
                            match interp_redirect {
                                InterpreterRedirection::HereDocument(_, _) => {
                                    match self.wait_for_process(exec, pids[j], is_special_builtin_fun) {
                                        Some(tmp_status) if tmp_status != 0 => {
                                            tmp_is_success = false;
                                            is_success_for_interp_redirects = false;
                                        },
                                        None => {
                                            tmp_is_success = false;
                                            is_success_for_interp_redirects = false;
                                        },
                                        _ => (),
                                    }
                                    j += 1;
                                },
                                _ => (),
                            }
                        }
                        if is_success {
                            match self.wait_for_process(exec, pid, false) {
                                Some(tmp_status) => {
                                    is_success &= tmp_is_success;
                                    tmp_status
                                },
                                None => {
                                    is_success = false;
                                    1
                                },
                            }
                        } else {
                            1
                        }
                });
            }
        } else {
            exec.clear_pipes();
        }
        if is_success && self.exec_redirect_flag {
            for interp_redirect in &interp_redirects {
                match interp_redirect {
                    InterpreterRedirection::Input(vfd, _) => exec.pop_penultimate_file(*vfd),
                    InterpreterRedirection::Output(vfd, _, _) => exec.pop_penultimate_file(*vfd),
                    InterpreterRedirection::InputAndOutput(vfd, _) => exec.pop_penultimate_file(*vfd),
                    InterpreterRedirection::Appending(vfd, _) => exec.pop_penultimate_file(*vfd),
                    InterpreterRedirection::Duplicating(vfd, _) => exec.pop_penultimate_file(*vfd),
                    InterpreterRedirection::HereDocument(_, _) => (),
                }
            }
            self.exec_redirect_flag = false;
        } else {
            interp_redirects.reverse();
            for interp_redirect in &interp_redirects[(interp_redirects.len() - i)..] {
                match interp_redirect {
                    InterpreterRedirection::Input(vfd, _) => exec.pop_file(*vfd),
                    InterpreterRedirection::Output(vfd, _, _) => exec.pop_file(*vfd),
                    InterpreterRedirection::InputAndOutput(vfd, _) => exec.pop_file(*vfd),
                    InterpreterRedirection::Appending(vfd, _) => exec.pop_file(*vfd),
                    InterpreterRedirection::Duplicating(vfd, _) => exec.pop_file(*vfd),
                    InterpreterRedirection::HereDocument(_, _) => (),
                }
            }
        }
        if is_success_for_interp_redirects {
            status
        } else {
            if is_special_builtin_fun {
                self.exit(status, false)
            } else {
                status
            }
        }
    }
    
    fn add_vars(&mut self, exec: &mut Executor, word_iter: &mut slice::Iter<'_, Rc<Word>>, vars: &mut Vec<(String, String)>,  env: &mut Environment, settings: &mut Settings) -> Option<Option<Rc<Word>>>
    {
        loop {
            match word_iter.next() {
                Some(word) => {
                    let first_s = match word.word_elems.first() {
                        Some(WordElement::Simple(SimpleWordElement::String(s))) => Some(s),
                        _ => None,
                    };
                    match first_s.map(|s| s.split_once('=')).flatten() {
                        Some((name, value_part)) => {
                            if is_name_str(name) {
                                let mut word_elems: Vec<WordElement> = Vec::new();
                                if !value_part.is_empty() {
                                    word_elems.push(WordElement::Simple(SimpleWordElement::String(String::from(value_part))));
                                }
                                word_elems.extend_from_slice(&word.word_elems[1..]);
                                let new_word = Word {
                                    path: word.path.clone(),
                                    pos: Position { line: word.pos.line, column: word.pos.column + name.len() as u64, }, 
                                    word_elems,
                                };
                                match self.perform_var_word_expansion_as_string(exec, &new_word, env, settings) {
                                    Some(value) => vars.push((String::from(name), value)),
                                    None => break None,
                                }
                            } else {
                                break Some(Some((*word).clone()));
                            }
                        },
                        None => break Some(Some((*word).clone())),
                    }
                },
                None => break Some(None),
            }
        }
    }

    fn interpret_simple_command(&mut self, exec: &mut Executor, command: &SimpleCommand, env: &mut Environment, settings: &mut Settings) -> i32
    {
        if settings.noexec_flag {
            return self.last_status;
        }
        let mut vars: Vec<(String, String)> = Vec::new();
        let mut word_iter = command.words.iter();
        let status = match self.add_vars(exec, &mut word_iter, &mut vars, env, settings) {
            Some(Some(prog_word)) => {
                match self.perform_word_expansion(exec, &(*prog_word), env, settings) {
                    Some(mut args) => {
                        let mut redirects: Vec<Rc<Redirection>> = Vec::new();
                        let mut is_success = true;
                        if args.is_empty() {
                            loop {
                                match word_iter.next() {
                                    Some(prog_word) => {
                                        match self.perform_word_expansion(exec, &(*prog_word), env, settings) {
                                            Some(args2) => {
                                                args.extend(args2);
                                                if !args.is_empty() { break; }
                                            },
                                            None => is_success = false,
                                        }
                                    },
                                    None => (),
                                }
                            }
                        }
                        if is_success {
                            match args.first() {
                                Some(arg0) => {
                                    match env.alias(arg0) {
                                        Some(alias) => {
                                            let mut cursor = Cursor::new(alias.as_bytes());
                                            let mut cr = CharReader::new(&mut cursor);
                                            let mut lexer = Lexer::new("(alias)", &Position::new(0, 0), &mut cr, 0, false);
                                            let mut parser = Parser::new();
                                            parser.set_error_cont(false);
                                            match parser.parse_alias_command(&mut lexer, settings) {
                                                Ok(alias_command) => {
                                                    let mut alias_word_iter = alias_command.command.words.iter();
                                                    match self.add_vars(exec, &mut alias_word_iter, &mut vars, env, settings) {
                                                        Some(Some(alias_prog_word)) => {
                                                            let mut alias_args: Vec<String> = Vec::new();
                                                            match self.perform_word_expansion(exec, &(*alias_prog_word), env, settings) {
                                                                Some(alias_args2) => alias_args.extend(alias_args2),
                                                                None => is_success = false,
                                                            }
                                                            if is_success {
                                                                let tmp_alias_words: Vec<Rc<Word>> = alias_word_iter.map(|we| we.clone()).collect();
                                                                match self.perform_word_expansions(exec, tmp_alias_words.as_slice(), env, settings) {
                                                                    Some(alias_args2) => alias_args.extend(alias_args2),
                                                                    None => is_success = false,
                                                                }
                                                            }
                                                            if is_success {
                                                                let tmp_args = args.clone();
                                                                args = alias_args;
                                                                args.extend_from_slice(&tmp_args[1..]);
                                                                redirects.extend(alias_command.command.redirects);
                                                            }
                                                        },
                                                        Some(None) => redirects.extend(alias_command.command.redirects),
                                                        None => is_success = false,
                                                    }
                                                },
                                                Err(err) => {
                                                    xsfprintln!(exec, 2, "{}", err);
                                                    self.last_status = 1;
                                                    return self.exit(1, false);
                                                },
                                            }
                                        },
                                        None => (),
                                    }
                                },
                                None => (),
                            }
                        }
                        if is_success {
                            let tmp_words: Vec<Rc<Word>> = word_iter.map(|w| w.clone()).collect();
                            match self.perform_word_expansions(exec, tmp_words.as_slice(), env, settings) {
                                Some(args2) => args.extend(args2),
                                None => is_success = false,
                            }
                            redirects.extend(command.redirects.clone());
                        }
                        if is_success {
                            match args.first() {
                                Some(arg0) => {
                                    if settings.xtrace_flag {
                                        print_command_for_xtrace(exec, vars.as_slice(), args.as_slice());
                                    }
                                    self.interpret_redirects(exec, redirects.as_slice(), self.has_special_builtin_fun(arg0.as_str(), env), env, settings, |interp, exec, env, settings| {
                                            interp.execute(exec, vars.as_slice(), arg0.as_str(), &args[1..], false, env, settings).unwrap_or(1)
                                    })
                                },
                                None => {
                                    if settings.xtrace_flag {
                                        print_command_for_xtrace(exec, vars.as_slice(), &[]);
                                    }
                                    self.interpret_redirects(exec, redirects.as_slice(), false, env, settings, |_, exec, env, settings| {
                                            set_vars(exec, vars.as_slice(), env, settings)
                                    })
                                },
                            }
                        } else {
                            1
                        }
                    },
                    None => 1,
                }
            },
            Some(None) => {
                if settings.xtrace_flag {
                    print_command_for_xtrace(exec, vars.as_slice(), &[]);
                }
                self.interpret_redirects(exec, command.redirects.as_slice(), false, env, settings, |_, exec, env, settings| {
                        set_vars(exec, vars.as_slice(), env, settings)
                })
            },
            None => 1,
        };
        self.last_status = status;
        if status != 0 && settings.errexit_flag && self.non_simple_comamnd_count == 0 {
            self.exit(status, true)
        } else {
            status
        }
    }

    fn interpret_compound_command(&mut self, exec: &mut Executor, command: &CompoundCommand, redirects: &[Rc<Redirection>], env: &mut Environment, settings: &mut Settings) -> i32
    {
        self.interpret_redirects(exec, redirects, false, env, settings, |interp, exec, env, settings| {
                match command {
                    CompoundCommand::BraceGroup(commands) => {
                        exec.interpret_or(commands.len() > 1, |exec| {
                                interp.interpret_logical_commands(exec, commands.as_slice(), env, settings)
                        })
                    },
                    CompoundCommand::Subshell(commands) => {
                        if settings.noexec_flag {
                            return interp.last_status;
                        }
                        let res = exec.create_process(false, settings, |exec, settings| {
                                exec.interpret_or(commands.len() > 1, |exec| {
                                        interp.interpret_logical_commands(exec, commands.as_slice(), env, settings)
                                })
                        });
                        match res {
                            Ok(pid) => {
                                let status = interp.wait_for_process(exec, pid, false).unwrap_or(1);
                                interp.last_status = status;
                                status
                            },
                            Err(err) => {
                                xcfprintln!(exec, 2, "{}", err);
                                1
                            },
                        }
                    },
                    CompoundCommand::For(name_word, words, commands) => {
                        exec.interpret(|exec| {
                                if settings.noexec_flag {
                                    return interp.last_status;
                                }
                                match interp.perform_word_expansion_as_string(exec, &(*name_word), env, settings) {
                                    Some(name) => {
                                        match interp.perform_word_expansions(exec, words.as_slice(), env, settings) {
                                            Some(elems) => {
                                                interp.current_loop_count += 1;
                                                for elem in elems {
                                                    if env.read_only_var_attr(name.as_str()) {
                                                        xcfprintln!(exec, 2, "{}: Is read only", name);
                                                        interp.last_status = 1;
                                                        break;
                                                    }
                                                    env.set_var(name.as_str(), elem.as_str(), settings);
                                                    if settings.noexec_flag { break; }
                                                    interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                                                    if interp.has_continue_with_one() {
                                                        interp.clear_return_state();
                                                        continue;
                                                    }
                                                    if interp.has_break_or_continue_or_return_or_exit() {
                                                        break;
                                                    }
                                                }
                                                interp.current_loop_count -= 1;
                                                if interp.has_break_or_continue() {
                                                    interp.clear_return_state_for_break_or_continue();
                                                }
                                                interp.last_status
                                            },
                                            None => {
                                                interp.last_status = 1;
                                                1
                                            },
                                        }
                                    },
                                    None => {
                                        interp.last_status = 1;
                                        1
                                    },
                                }
                        })
                    },
                    CompoundCommand::Case(value_word, pairs) => {
                        exec.interpret(|exec| {
                                if settings.noexec_flag {
                                    return interp.last_status;
                                }
                                match interp.perform_word_expansion_as_string(exec, &(*value_word), env, settings) {
                                    Some(value) => {
                                        let mut is_success = true;
                                        for pair in pairs.iter() {
                                            let mut is_matched = true;
                                            for pattern_word in &pair.pattern_words {
                                                match interp.perform_word_expansion_as_string(exec, &(*pattern_word), env, settings) {
                                                    Some(pattern) => {
                                                        is_matched = fnmatch(&pattern, &value, 0);
                                                        if is_matched { break; }
                                                    },
                                                    None => {
                                                        is_success = false;
                                                        break;
                                                    },
                                                }
                                            }
                                            if !is_success { break; }
                                            if is_matched {
                                                interp.interpret_logical_commands(exec, pair.commands.as_slice(), env, settings);
                                                break;
                                            }
                                        }
                                        if is_success {
                                            interp.last_status
                                        } else {
                                            interp.last_status = 1;
                                            1
                                        }
                                    },
                                    None => 1,
                                }
                        })
                    },
                    CompoundCommand::If(cond_commands, commands, pairs, else_commands) => {
                        exec.interpret(|exec| {
                                if settings.noexec_flag {
                                    return interp.last_status;
                                }
                                interp.non_simple_comamnd_count += 1;
                                let cond_status = interp.interpret_logical_commands(exec, cond_commands.as_slice(), env, settings);
                                interp.non_simple_comamnd_count -= 1;
                                if cond_status == 0 {
                                    interp.interpret_logical_commands(exec, commands.as_slice(), env, settings)
                                } else {
                                    let mut elif_cond = false;
                                    let mut status = interp.last_status;
                                    for pair in pairs {
                                        if settings.noexec_flag {
                                            return interp.last_status;
                                        }
                                        interp.non_simple_comamnd_count += 1;
                                        let cond_status2 = interp.interpret_logical_commands(exec, pair.cond_commands.as_slice(), env, settings);
                                        interp.non_simple_comamnd_count -= 1;
                                        if cond_status2 == 0 {
                                            elif_cond = true;
                                            status = interp.interpret_logical_commands(exec, pair.commands.as_slice(), env, settings);
                                            break;
                                        }
                                    }
                                    if !elif_cond {
                                        match else_commands {
                                            Some(else_commands) => interp.interpret_logical_commands(exec, else_commands.as_slice(), env, settings),
                                            None => status,
                                        }
                                    } else {
                                        status
                                    }
                                }
                        })
                    },
                    CompoundCommand::While(cond_commands, commands) => {
                        exec.interpret(|exec| {
                                interp.current_loop_count += 1;
                                loop {
                                    if settings.noexec_flag { break; }
                                    interp.non_simple_comamnd_count += 1;
                                    let cond_status = interp.interpret_logical_commands(exec, cond_commands.as_slice(), env, settings);
                                    interp.non_simple_comamnd_count -= 1;
                                    if cond_status == 0 {
                                        if settings.noexec_flag { break; }
                                        interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                                        if interp.has_continue_with_one() {
                                            interp.clear_return_state();
                                            continue;
                                        }
                                        if interp.has_break_or_continue_or_return_or_exit() {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                interp.current_loop_count -= 1;
                                if interp.has_break_or_continue() {
                                    interp.clear_return_state_for_break_or_continue();
                                }
                                interp.last_status
                        })
                    },
                    CompoundCommand::Until(cond_commands, commands) => {
                        exec.interpret(|exec| {
                                interp.current_loop_count += 1;
                                loop {
                                    interp.non_simple_comamnd_count += 1;
                                    let cond_status = interp.interpret_logical_commands(exec, cond_commands.as_slice(), env, settings);
                                    interp.non_simple_comamnd_count -= 1;
                                    if cond_status != 0 {
                                        interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                                        if interp.has_continue_with_one() {
                                            interp.clear_return_state();
                                            continue;
                                        }
                                        if interp.has_break_or_continue_or_return_or_exit() {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                interp.current_loop_count -= 1;
                                if interp.has_break_or_continue() {
                                    interp.clear_return_state_for_break_or_continue();
                                }
                                interp.last_status
                        })
                    },
                }
        })
    }

    fn interpret_fun_def(&mut self, exec: &mut Executor, name_word: &Word, fun_body: &Rc<FunctionBody>, env: &mut Environment, settings: &mut Settings) -> i32
    {
        if settings.noexec_flag {
            return self.last_status;
        }
        match self.perform_word_expansion_as_string(exec, &(*name_word), env, settings) {
            Some(name) => {
                env.set_fun(name.as_str(), fun_body);
                self.last_status = 0;
                0
            },
            None => {
                self.last_status = 1;
                1
            },
        }
    }
    
    fn interpret_command(&mut self, exec: &mut Executor, command: &Command, env: &mut Environment, settings: &mut Settings) -> i32
    {
        env.set_var("LINENO", format!("{}", command.pos().line).as_str(), settings);
        match command {
            Command::Simple(_, _, simple_command) => self.interpret_simple_command(exec, &(*simple_command), env, settings),
            Command::Compound(_, _, compound_command, redirects) => self.interpret_compound_command(exec, &(*compound_command), redirects.as_slice(), env, settings),
            Command::FunctionDefinition(_, _, name_word, fun_body) => self.interpret_fun_def(exec, &(*name_word), fun_body, env, settings),
        }
    }

    fn interpret_pipe_command(&mut self, exec: &mut Executor, command: &PipeCommand, env: &mut Environment, settings: &mut Settings) -> i32
    {
        let mut status = self.last_status;
        if settings.noexec_flag {
            return status;
        }
        if command.commands.len() <= 1 {
            if command.is_negative {
                self.non_simple_comamnd_count += 1;
            }
            if !command.commands.is_empty() {
                status = self.interpret_command(exec, &(*command.commands[0]), env, settings);
            }
            if command.is_negative {
                self.non_simple_comamnd_count -= 1;
            }
        } else {
            exec.interpret(|exec| {
                    let mut pipes: Vec<Pipe> = Vec::new();
                    let mut is_success = true;
                    for _ in 0..(command.commands.len() - 1) {
                        match pipe_with_cloexec() {
                            Ok(pipe_fds) => pipes.push(unsafe { Pipe::from_pipe_fds(&pipe_fds) }),
                            Err(err) => {
                                xcfprintln!(exec, 2, "{}", err);
                                is_success = false;
                            }
                        }
                    }
                    if !is_success { return; }
                    exec.set_pipes(pipes);
                    self.non_simple_comamnd_count += 1;
                    let mut pids: Vec<Option<i32>> = Vec::new();
                    for i in 0..command.commands.len() {
                        let res = exec.create_process(false, settings, |exec, settings| {
                                let mut is_reading_file = false;
                                let mut is_writing_file = false;
                                if i > 0 {
                                    exec.push_file(0, exec.pipes()[i - 1].reading_file.clone());
                                    is_reading_file = true;
                                }
                                if i < exec.pipes().len() {
                                    exec.push_file(1, exec.pipes()[i].writing_file.clone());
                                    is_writing_file = true;
                                }
                                exec.clear_pipes();
                                self.non_simple_comamnd_count += 1;
                                status = self.interpret_command(exec, &(*command.commands[i]), env, settings);
                                self.non_simple_comamnd_count -= 1;
                                if is_writing_file {
                                    exec.pop_file(1);
                                }
                                if is_reading_file {
                                    exec.pop_file(0);
                                }
                                status
                        });
                        match res {
                            Ok(pid) => pids.push(pid),
                            Err(err) => {
                                xcfprintln!(exec, 2, "{}", err);
                                break;
                            },
                        }
                    }
                    self.non_simple_comamnd_count -= 1;
                    exec.clear_pipes();
                    status = 1;
                    for (i, pid) in pids.iter().enumerate() {
                        let tmp_status = self.wait_for_process(exec, *pid, false).unwrap_or(1);
                        if i == command.commands.len() - 1 {
                            status = tmp_status;
                        }
                    }
                    self.last_status = status;
            });
        }
        if command.is_negative {
            if status == 0 {
                status = 1;
            } else {
                status = 0;
            }
            self.last_status = status;
            status
        } else {
            status
        }
    }
    
    fn interpret_logical_command(&mut self, exec: &mut Executor, command: &LogicalCommand, env: &mut Environment, settings: &mut Settings) -> i32
    {
        if settings.noexec_flag {
            return self.last_status;
        }
        let mut f = |exec: &mut Executor, settings: &mut Settings| -> i32 {
            if command.pairs.is_empty() {
                if settings.noexec_flag { return self.last_status; }
                self.interpret_pipe_command(exec, &(*command.first_command), env, settings)
            } else {
                exec.interpret(|exec| {
                        if settings.noexec_flag { return self.last_status; }
                        self.non_simple_comamnd_count += 1;
                        let mut status = self.interpret_pipe_command(exec, &(*command.first_command), env, settings);
                        if !self.has_break_or_continue_or_return_or_exit() {
                            for pair in &command.pairs {
                                if settings.noexec_flag { break; }
                                match pair.op {
                                    LogicalOperator::And => {
                                        if status == 0 {
                                            status = self.interpret_pipe_command(exec, &(*pair.command), env, settings);
                                            if self.has_break_or_continue_or_return_or_exit() { break; }
                                        }
                                    },
                                    LogicalOperator::Or => {
                                        if status != 0 {
                                            status = self.interpret_pipe_command(exec, &(*pair.command), env, settings);
                                            if self.has_break_or_continue_or_return_or_exit() { break; }
                                        }
                                    },
                                }
                            }
                        }
                        self.non_simple_comamnd_count -= 1;
                        status
                })
            }
        };
        if command.is_in_background {
            if settings.noexec_flag { return self.last_status; }
            match exec.create_process(true, settings, f) {
                Ok(Some(pid)) => {
                    match exec.add_job(&Job::new(pid, format!("{}", command).as_str())) {
                        Some(job_id) => {
                            self.last_job_pid = Some(pid);
                            if settings.notify_flag {
                                xsfprintln!(exec, 2, "[{}] {}", job_id, pid);
                            }
                        },
                        None => xcfprintln!(exec, 2, "No free job identifiers"),
                    }
                },
                Err(err) => xcfprintln!(exec, 2, "{}", err),
                _ => (),
            }
            self.last_status
        } else {
            f(exec, settings)
        }
    }
    
    pub fn interpret_logical_commands(&mut self, exec: &mut Executor, commands: &[Rc<LogicalCommand>], env: &mut Environment, settings: &mut Settings) -> i32
    {
        exec.interpret_or(commands.len() > 1, |exec| {
                let mut status = self.last_status;
                for command in commands.iter() {
                    if settings.noexec_flag { break; }
                    status = self.interpret_logical_command(exec, &(**command), env, settings);
                    if self.has_break_or_continue_or_return_or_exit() { break; }
                }
                status
        })
    }

    pub fn interpret_fun_body(&mut self, exec: &mut Executor, fun_body: &FunctionBody, env: &mut Environment, settings: &mut Settings) -> i32
    {
        self.fun_count += 1;
        self.push_loop_count(0);
        let status = self.interpret_compound_command(exec, &fun_body.command, fun_body.redirects.as_slice(), env, settings);
        self.pop_loop_count();
        self.fun_count -= 1;
        if self.has_break_or_continue_or_return() {
            self.clear_return_state();
        }
        status
    }
}

pub fn set_param(param_name: &ParameterName, s: &str, env: &mut Environment, settings: &Settings) -> bool
{
    match param_name {
        ParameterName::Variable(name) => {
            if !env.read_only_var_attr(name) {
                env.set_var(name.as_str(), s, settings);
                true
            } else {
                false
            }
        },
        _ => false,
    }
}

pub fn is_read_only_param(param_name: &ParameterName, env: &Environment) -> bool
{
    match param_name {
        ParameterName::Variable(name) => env.read_only_var_attr(name),
        _ => false,
    }
}

#[cfg(test)]
mod tests;
