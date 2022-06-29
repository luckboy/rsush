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
use std::cell::*;
use std::io::*;
use std::fs::*;
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value
{
    String(String),
    AtArray(Vec<String>),
    StarArray(Vec<String>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ReturnState
{
    None,
    Break(usize),
    Return,
    Exit,
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
    loop_count_stack: Vec<usize>,
    current_loop_count: usize,
    signal_names: HashMap<i32, String>,
}

fn set_vars(vars: &[(String, String)], env: &mut Environment, settings: &Settings)
{
    for (name, value) in vars.iter() {
        env.set_var(name.as_str(), value.as_str(), settings);
    }
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
        Interpreter {
            last_status: 0,
            non_simple_comamnd_count: 0,
            return_state: ReturnState::None,
            loop_count_stack: Vec::new(),
            current_loop_count: 0,
            signal_names: sig_names,
        }
    }

    pub fn last_status(&self) -> i32
    { self.last_status }

    pub fn has_break(&self) -> bool
    { 
        match self.return_state {
            ReturnState::Break(_) => true,
            _ => false,
        }
    }
    
    pub fn has_break_or_return(&self) -> bool
    { 
        match self.return_state {
            ReturnState::Break(_) | ReturnState::Return => true,
            _ => false,
        }
    }
    
    pub fn has_break_or_return_or_exit(&self) -> bool
    { 
        match self.return_state {
            ReturnState::Break(_) | ReturnState::Return | ReturnState::Exit => true,
            _ => false,
        }
    }
    
    pub fn exit(&mut self, status: i32) -> i32
    {
        self.return_state = ReturnState::Exit;
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
    
    pub fn clear_return_state(&mut self)
    { self.return_state = ReturnState::None; }

    pub fn clear_return_state_for_break(&mut self)
    {
        match self.return_state {
            ReturnState::Break(n) if n > 1 => self.return_state = ReturnState::Break(n - 1),
            _ => self.return_state = ReturnState::None,
        }
    }
    
    pub fn is_in_loop(&self) -> bool
    { self.current_loop_count > 0 }
    
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

    fn execute(&mut self, exec: &mut Executor, vars: &[(String, String)], arg0: &str, args: &[String], env: &mut Environment, settings: &mut Settings) -> i32
    {
        if arg0 == "." {
            for (name, value) in vars.iter() {
                env.unset_unexported_var(name.as_str());
                env.set_exported_var(name.as_str(), value.as_str());
            }
            match args.first() {
                Some(path) => {
                    match File::open(path) {
                        Ok(mut file) => {
                            let mut br = BufReader::new(&mut file);
                            let mut cr = CharReader::new(&mut br);
                            let mut lexer = Lexer::new(path, &Position::new(0, 0), &mut cr, 0, false);
                            let mut parser = Parser::new();
                            loop {
                                match parser.parse_logical_commands_for_line(&mut lexer, settings) {
                                    Ok(None) => break self.last_status,
                                    Ok(Some(commands)) => {
                                        if settings.verbose_flag {
                                            eprint!("{}", lexer.content_for_verbose());
                                        }
                                        self.interpret_logical_commands(exec, &commands, env, settings);
                                    },
                                    Err(err) => {
                                        eprintln!("{}", err);
                                        break 1;
                                    },
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("{}", err);
                            1
                        },
                    }
                },
                None => 0,
            }
        } else {
            match exec.execute(self, vars, arg0, args, env, settings) {
                Ok(WaitStatus::None) => panic!("wait status is none"),
                Ok(WaitStatus::Exited(status)) => status,
                Ok(WaitStatus::Signaled(sig, is_coredump)) => {
                    eprintln!("{}", self.signal_string(sig, is_coredump));
                    sig + 128
                },
                Ok(WaitStatus::Stopped(_)) => panic!("wait status is stopped"),
                Err(err) => {
                    eprintln!("{}", err);
                    1
                }
            }
        }
    }
    
    fn wait_for_process(&mut self, exec: &mut Executor, pid: Option<i32>) -> i32
    {
        match exec.wait_for_process(pid, true, false) {
            Ok(WaitStatus::None) => panic!("wait status is none"),
            Ok(WaitStatus::Exited(status)) => {
                status
            },
            Ok(WaitStatus::Signaled(sig, is_coredump)) => {
                eprintln!("{}", self.signal_string(sig, is_coredump));
                sig + 128
            },
            Ok(WaitStatus::Stopped(_)) => panic!("wait status is stopped"),
            Err(err) => {
                eprintln!("{}", err);
                1
            },
        }
    }

    fn performe_var_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        None
    }

    fn performe_pattern_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        None
    }

    fn performe_word_expansion(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<Vec<String>>
    {
        None
    }
        
    fn performe_word_expansion_as_string(&mut self, exec: &mut Executor, word: &Word, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
       match self.performe_word_expansion(exec, word, env, settings) {
           Some(ss) => {
               let mut s = String::new();
               let mut is_first = true;
               for t in &ss {
                   if !is_first { s.push(' '); }
                   s.push_str(t.as_str());
                   is_first = false;
               }
               Some(s)
           },
           None => None,
       }
    }

    fn performe_word_expansions(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<Vec<String>>
    {
        None
    }
    
    fn performe_word_expansions_as_string(&mut self, exec: &mut Executor, words: &[Rc<Word>], env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
       match self.performe_word_expansions(exec, words, env, settings) {
           Some(ss) => {
               let mut s = String::new();
               let mut is_first = true;
               for t in &ss {
                   if !is_first { s.push(' '); }
                   s.push_str(t.as_str());
                   is_first = false;
               }
               Some(s)
           },
           None => None,
       }
    }

    fn performe_here_doc_expansion(&mut self, exec: &mut Executor, here_doc: &HereDocument, env: &mut Environment, settings: &mut Settings) -> Option<String>
    {
        None
    }
    
    fn interpret_redirects<F>(&mut self, exec: &mut Executor, redirects: &[Rc<Redirection>], env: &mut Environment, settings: &mut Settings, f: F) -> i32
        where F: FnOnce(&mut Self, &mut Executor, &mut Environment, &mut Settings) -> i32
    {
        let mut is_success = true;
        let mut interp_redirects: Vec<InterpreterRedirection> = Vec::new();
        for redirect in redirects.iter() {
            match &(**redirect) {
                Redirection::Input(_, _, n, word) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Input(n.unwrap_or(0), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::Output(_, _, n, word, is_bar) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Output(n.unwrap_or(1), path, *is_bar)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::InputAndOutput(_, _, n, word) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::InputAndOutput(n.unwrap_or(0), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::Appending(_, _, n, word) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(path) => interp_redirects.push(InterpreterRedirection::Appending(n.unwrap_or(1), path)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::InputDuplicating(_, _, n, word) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(fd_s) => {
                            if is_io_number_str(fd_s.as_str()) {
                                match fd_s.parse::<i32>() {
                                    Ok(fd) => interp_redirects.push(InterpreterRedirection::Duplicating(n.unwrap_or(0), fd)),
                                    Err(err) => {
                                        eprintln!("Too large I/O number");
                                    },
                                }
                            } else {
                                eprintln!("Invalid I/O number");
                                is_success = false;
                            }
                        },
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
                Redirection::OutputDuplicating(_, _, n, word) => {
                    match self.performe_word_expansion_as_string(exec, &(*word), env, settings) {
                        Some(fd_s) => {
                            if is_io_number_str(fd_s.as_str()) {
                                match fd_s.parse::<i32>() {
                                    Ok(fd) => interp_redirects.push(InterpreterRedirection::Duplicating(n.unwrap_or(1), fd)),
                                    Err(_) => {
                                        eprintln!("Too large I/O number");
                                    },
                                }
                            } else {
                                eprintln!("Invalid I/O number");
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
                    match self.performe_here_doc_expansion(exec, &here_doc.borrow(), env, settings) {
                        Some(s) => interp_redirects.push(InterpreterRedirection::HereDocument(n.unwrap_or(1), s)),
                        None => {
                            is_success = false;
                            break;
                        },
                    }
                },
            }
        }
        if !is_success { return 1; }
        let mut pipes: Vec<Pipe> = Vec::new();
        let mut i = 0;
        for interp_redirect in &interp_redirects {
            match interp_redirect {
                InterpreterRedirection::Input(vfd, path) => {
                    match File::open(path) {
                        Ok(file) => exec.push_file(*vfd, Rc::new(RefCell::new(file))),
                        Err(err) => {
                            eprintln!("{}: {}", path, err);
                            is_success = false;
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
                            eprintln!("{}: {}", path, err);
                            is_success = false;
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
                            eprintln!("{}: {}", path, err);
                            is_success = false;
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
                            eprintln!("{}: {}", path, err);
                            is_success = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::Duplicating(new_vfd, old_vfd) => {
                    let old_file = exec.current_file(*old_vfd).map(|f| f.clone());
                    match old_file {
                        Some(file) => exec.push_file(*new_vfd, file),
                        None => {
                            eprintln!("{}: Bad fd number", *old_vfd);
                            is_success = false;
                            break;
                        },
                    }
                },
                InterpreterRedirection::HereDocument(_, _) => {
                    match pipe_with_cloexec() {
                        Ok(pipe_fds) => pipes.push(Pipe::from_pipe_fds(&pipe_fds)),
                        Err(err) => {
                            eprintln!("{}", err);
                            is_success = false;
                            break;
                        }
                    }
                },
            }
            i += 1;
        }
        if !is_success { return 1; }
        let status: i32;
        if pipes.is_empty() {
            status = f(self, exec, env, settings);
        } else {
            status = exec.interpret(|exec| {
                    let mut j = 0;
                    let mut k = 0;
                    let mut pids: Vec<Option<i32>> = Vec::new(); 
                    for interp_redirect in &interp_redirects {
                        match interp_redirect {
                            InterpreterRedirection::HereDocument(_, s) => {
                                let res = exec.create_process(false, settings, |exec, settings| {
                                        let file = exec.pipes()[j].writing_file.clone();
                                        exec.clear_pipes();
                                        let mut tmp_file = file.borrow_mut();
                                        match tmp_file.write_all(s.as_bytes()) {
                                            Ok(()) => 0,
                                            Err(err) => {
                                                eprintln!("{}", err);
                                                1
                                            },
                                        }
                                });
                                match res {
                                    Ok(pid) => pids.push(pid),
                                    Err(err) => {
                                        eprintln!("{}", err);
                                        is_success = false;
                                        break
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
                                eprintln!("{}", err);
                                is_success = false;
                            },
                        }
                    }
                    exec.clear_pipes();
                    interp_redirects.reverse();
                    for interp_redirect in &interp_redirects[(interp_redirects.len() - k)..] {
                        match interp_redirect {
                            InterpreterRedirection::HereDocument(_, _) => {
                                j -= 1;
                                self.wait_for_process(exec, pids[j]);
                            },
                            _ => (),
                        }
                    }
                    if is_success {
                        self.wait_for_process(exec, pid)
                    } else {
                        1
                    }
            });
        }
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
        status
    }
    
    fn add_vars<'a>(&mut self, exec: &mut Executor, word_iter: &mut slice::Iter<'a, Rc<Word>>, vars: &mut Vec<(String, String)>,  env: &mut Environment, settings: &mut Settings) -> Option<Option<Rc<Word>>>
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
                            match self.performe_var_word_expansion_as_string(exec, &new_word, env, settings) {
                                Some(value) => vars.push((String::from(name), value)),
                                None => break None,
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
                match self.performe_word_expansion(exec, &(*prog_word), env, settings) {
                    Some(mut args) => {
                        let mut redirects: Vec<Rc<Redirection>> = Vec::new();
                        let mut is_success = true;
                        if args.is_empty() {
                            loop {
                                match word_iter.next() {
                                    Some(prog_word) => {
                                        match self.performe_word_expansion(exec, &(*prog_word), env, settings) {
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
                                                            match self.performe_word_expansion(exec, &(*alias_prog_word), env, settings) {
                                                                Some(alias_args2) => alias_args.extend(alias_args2),
                                                                None => is_success = false,
                                                            }
                                                            if is_success {
                                                                let tmp_alias_words: Vec<Rc<Word>> = alias_word_iter.map(|we| we.clone()).collect();
                                                                match self.performe_word_expansions(exec, tmp_alias_words.as_slice(), env, settings) {
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
                                                        Some(None) => {
                                                        },
                                                        None => is_success = false,
                                                    }
                                                },
                                                Err(err) => {
                                                    eprintln!("{}", err);
                                                    is_success = false;
                                                },
                                            }
                                        },
                                        None => (),
                                    }
                                },
                                None => set_vars(vars.as_slice(), env, settings),
                            }
                        }
                        if is_success {
                            let tmp_words: Vec<Rc<Word>> = word_iter.map(|we| we.clone()).collect();
                            match self.performe_word_expansions(exec, tmp_words.as_slice(), env, settings) {
                                Some(args2) => args.extend(args2),
                                None => is_success = false,
                            }
                            redirects.extend(command.redirects.clone());
                        }
                        if is_success {
                            match args.first() {
                                Some(arg0) => {
                                    self.interpret_redirects(exec, redirects.as_slice(), env, settings, |interp, exec, env, settings| {
                                            interp.execute(exec, vars.as_slice(), arg0.as_str(), &args[1..], env, settings)
                                    })
                                },
                                None => {
                                    set_vars(vars.as_slice(), env, settings);
                                    0
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
                set_vars(vars.as_slice(), env, settings);
                0
            },
            None => 1,
        };
        self.last_status = status;
        if settings.errexit_flag && self.non_simple_comamnd_count == 0 {
            self.exit(status)
        } else {
            status
        }
    }

    fn interpret_compound_command(&mut self, exec: &mut Executor, command: &CompoundCommand, redirects: &[Rc<Redirection>], env: &mut Environment, settings: &mut Settings) -> i32
    {
        self.interpret_redirects(exec, redirects, env, settings, |interp, exec, env, settings| {
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
                                let status = interp.wait_for_process(exec, pid);
                                interp.last_status = status;
                                status
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                1
                            },
                        }
                    },
                    CompoundCommand::For(name_word, words, commands) => {
                        exec.interpret(|exec| {
                                if settings.noexec_flag {
                                    return interp.last_status;
                                }
                                match interp.performe_word_expansion_as_string(exec, &(*name_word), env, settings) {
                                    Some(name) => {
                                        match interp.performe_word_expansions(exec, words.as_slice(), env, settings) {
                                            Some(elems) => {
                                                interp.current_loop_count += 1;
                                                for elem in elems {
                                                    env.set_var(name.as_str(), elem.as_str(), settings);
                                                    if settings.noexec_flag { break; }
                                                    interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                                                    if interp.has_break_or_return_or_exit() {
                                                        break;
                                                    }
                                                }
                                                interp.current_loop_count -= 1;
                                                if interp.has_break() {
                                                    interp.clear_return_state_for_break();
                                                }
                                                interp.last_status
                                            },
                                            None => 1,
                                        }
                                    },
                                    None => 1,
                                }
                        })
                    },
                    CompoundCommand::Case(name_word, pairs) => {
                        exec.interpret(|exec| {
                                if settings.noexec_flag {
                                    return interp.last_status;
                                }
                                match interp.performe_word_expansion_as_string(exec, &(*name_word), env, settings) {
                                    Some(value) => {
                                        let mut is_success = true;
                                        for pair in pairs.iter() {
                                            let mut is_matched = true;
                                            for word_pattern in &pair.pattern_words {
                                                match interp.performe_word_expansion_as_string(exec, &(*name_word), env, settings) {
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
                                        if interp.has_break_or_return_or_exit() {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                interp.current_loop_count -= 1;
                                if interp.has_break() {
                                    interp.clear_return_state_for_break();
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
                                        if interp.has_break_or_return_or_exit() {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                interp.current_loop_count -= 1;
                                if interp.has_break() {
                                    interp.clear_return_state_for_break();
                                }
                                interp.last_status
                        })
                    },
                }
        })
    }

    fn interpret_fun_def(&mut self, exec: &mut Executor, name_word: &Word, fun_body: &FunctionBody, env: &mut Environment, settings: &mut Settings) -> i32
    {
        0
    }
    
    fn interpret_command(&mut self, exec: &mut Executor, command: &Command, env: &mut Environment, settings: &mut Settings) -> i32
    {
        match command {
            Command::Simple(_, _, simple_command) => self.interpret_simple_command(exec, &(*simple_command), env, settings),
            Command::Compound(_, _, compound_command, redirects) => self.interpret_compound_command(exec, &(*compound_command), redirects.as_slice(), env, settings),
            Command::FunctionDefinition(_, _, name_word, fun_body) => self.interpret_fun_def(exec, &(*name_word), &(*fun_body), env, settings),
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
                            Ok(pipe_fds) => pipes.push(Pipe::from_pipe_fds(&pipe_fds)),
                            Err(err) => {
                                eprintln!("{}", err);
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
                                eprintln!("{}", err);
                            },
                        }
                    }
                    self.non_simple_comamnd_count -= 1;
                    exec.clear_pipes();
                    status = 1;
                    for (i, pid) in pids.iter().enumerate() {
                        let tmp_status = self.wait_for_process(exec, *pid);
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
                        if !self.has_break_or_return_or_exit() 
                        {
                            for pair in &command.pairs {
                                if settings.noexec_flag { break; }
                                match pair.op {
                                    LogicalOperator::And => {
                                        if status == 0 {
                                            status = self.interpret_pipe_command(exec, &(*pair.command), env, settings);
                                            if self.has_break_or_return_or_exit() { break; }
                                        }
                                    },
                                    LogicalOperator::Or => {
                                        if status != 0 {
                                            status = self.interpret_pipe_command(exec, &(*pair.command), env, settings);
                                            if self.has_break_or_return_or_exit() { break; }
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
                    exec.add_job(&Job::new(pid, ""));
                },
                Err(err) => eprintln!("{}", err),
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
                    if self.has_break_or_return_or_exit() { break; }
                }
                status
        })
    }

    pub fn interpret_fun_body(&mut self, exec: &mut Executor, fun_body: &FunctionBody, env: &mut Environment, settings: &mut Settings) -> i32
    {
        self.push_loop_count(0);
        let status = self.interpret_compound_command(exec, &fun_body.command, fun_body.redirects.as_slice(), env, settings);
        self.pop_loop_count();
        if self.has_break_or_return() {
            self.clear_return_state();
        }
        status
    }
}
