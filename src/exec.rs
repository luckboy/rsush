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
use std::collections::HashSet;
use std::collections::hash_map;
use std::cell::*;
use std::io::*;
use std::fs::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;
use std::process::exit;
use std::rc::*;
use libc;
use crate::env::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State
{
    InInterpreter,
    InNewProcess,
}

#[derive(Clone)]
struct VirtualFile
{
    saved_file: Option<Rc<RefCell<File>>>,
    file_stack: Vec<Rc<RefCell<File>>>,
    current_file: Rc<RefCell<File>>,
}

#[derive(Clone)]
struct PipeFiles
{
    reading_file: Rc<RefCell<File>>,
    writing_file: Rc<RefCell<File>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WaitStatus
{
    None,
    Exited(i32),
    Signaled(i32),
}

pub struct Executor
{
    state_stack: Vec<State>,
    current_state: State,
    virtual_files: HashMap<i32, VirtualFile>,
    pipe_file_stack: Vec<PipeFiles>,
    interp_status: i32,
    shell_pid: i32,
    jobs: HashMap<i32, i32>,
}

impl Executor
{
    pub fn new() -> Executor
    {
        Executor {
            state_stack: Vec::new(),
            current_state: State::InInterpreter,
            virtual_files: HashMap::new(),
            pipe_file_stack: Vec::new(),
            interp_status: 0,
            shell_pid: process::id() as i32,
            jobs: HashMap::new(),
       }
    }
   
    fn push_state(&mut self, state: State)
    {
        self.state_stack.push(self.current_state);
        self.current_state = state;
    }
    
    fn pop_state(&mut self)
    {
        match self.state_stack.pop() {
            Some(state) => self.current_state = state,
            None => (),
        }
    }
    
    pub fn push_file(&mut self, vfd: i32, file: Rc<RefCell<File>>)
    {
        match self.virtual_files.get_mut(&vfd) {
            Some(virtual_file) => {
                virtual_file.file_stack.push(virtual_file.current_file.clone());
                virtual_file.current_file = file;
            },
            None => {
                let virtual_file = VirtualFile {
                    file_stack: Vec::new(),
                    current_file: file,
                    saved_file: None,
                };
                self.virtual_files.insert(vfd, virtual_file);
            },
        }
    }
    
    pub fn push_file_and_set_saved_file(&mut self, vfd: i32, file: Rc<RefCell<File>>)
    {
        match self.virtual_files.get_mut(&vfd) {
            Some(virtual_file) => {
                virtual_file.saved_file = Some(file.clone());
                virtual_file.file_stack.push(virtual_file.current_file.clone());
                virtual_file.current_file = file;
            },
            None => {
                let virtual_file = VirtualFile {
                    saved_file: Some(file.clone()),
                    file_stack: Vec::new(),
                    current_file: file,
                };
                self.virtual_files.insert(vfd, virtual_file);
            },
        }
    }   
    
    pub fn pop_file(&mut self, vfd: i32)
    {
        match self.virtual_files.get_mut(&vfd) {
            Some(virtual_file) => {
                match virtual_file.file_stack.pop() {
                    Some(file) => virtual_file.current_file = file,
                    None => {
                        self.virtual_files.remove(&vfd);
                    },
                }
            },
            None => (),
        }
    }
    
    pub fn current_file(&mut self, vfd: i32) -> Option<Rc<RefCell<File>>>
    { self.virtual_files.get(&vfd).map(|vf| vf.current_file.clone()) }
    
    pub fn saved_file(&mut self, vfd: i32) -> Option<Rc<RefCell<File>>>
    {
        match self.virtual_files.get(&vfd) {
            Some(virtual_file) => virtual_file.saved_file.as_ref().map(|f| f.clone()),
            None => None,
        }
    }
    
    pub fn push_pipe(&mut self, reading_file: Rc<RefCell<File>>, writing_file: Rc<RefCell<File>>)
    {   
        let pipe_files = PipeFiles {
            reading_file,
            writing_file,
        };
        self.pipe_file_stack.push(pipe_files);
    }
    
    pub fn pop_pipe(&mut self)
    { self.pipe_file_stack.pop(); }
    
    pub fn jobs(&self) -> hash_map::Iter<'_, i32, i32>
    { self.jobs.iter() }
    
    pub fn add_job(&mut self, pid: i32)
    {
        let mut job_id = 1;
        loop {
            if !self.jobs.contains_key(&job_id) {
                break;
            }
            job_id += 1;
        }
        self.jobs.insert(job_id, pid);
    }
    
    pub fn interpret<T, F>(&mut self, f: F) -> T
        where F: FnOnce(&mut Self) -> T
    {
        self.push_state(State::InInterpreter);
        let res = f(self);
        self.pop_state();
        res
    }
    
    pub fn create_process<F>(&mut self, is_in_background: bool, settings: &mut Settings, f: F) -> Result<Option<i32>>
        where F: FnOnce(&mut Self, &mut Settings) -> i32
    {
        if is_in_background {
            self.push_state(State::InInterpreter);
        }
        let pid = match self.current_state {
            State::InInterpreter => Some(fork()?),
            State::InNewProcess  => None,
        };
        let mut status = 0;
        match pid {
            Some(None) => {
                if settings.monitor_flag {
                    let _res = setpgid(0, self.shell_pid);
                }
            },
            _ => (),
        }
        match pid {
            Some(None) | None => {
                for (_, virtual_file) in self.virtual_files.iter_mut() {
                    virtual_file.file_stack.clear();
                }
                self.push_state(State::InNewProcess);
                status = f(self, settings);
                self.pop_state();
            },
            Some(Some(_)) => (),
        }
        if is_in_background {
            self.pop_state();
        }
        match pid {
            Some(None) => exit(status),
            Some(Some(pid)) => {
                if is_in_background {
                    self.add_job(pid);
                }
                Ok(Some(pid))
            },
            None => {
                self.interp_status = status;
                Ok(None)
            },
        }
    }

    pub fn wait_for_process(&self, pid: Option<i32>, is_hang: bool) -> Result<WaitStatus>
    {
        match (pid, self.current_state) {
            (Some(pid), State::InInterpreter) => {
                let mut status = 0;
                let opts = if is_hang {
                    0
                } else {
                    libc::WNOHANG
                };
                let mut res = Ok(WaitStatus::None);
                loop {
                    let pid2 = waitpid(pid, Some(&mut status), opts)?;
                    match pid2 {
                        Some(_) => {
                            if libc::WIFEXITED(status) {
                                res = Ok(WaitStatus::Exited(libc::WEXITSTATUS(status)));
                                break;
                            } else if libc::WIFSIGNALED(status) {
                                res = Ok(WaitStatus::Signaled(libc::WTERMSIG(status)));
                                break;
                            } else {
                                if !is_hang {
                                    break;
                                }
                            }
                        },
                        None => break,
                    }
                }
                res
            },
            _  => Ok(WaitStatus::Exited(self.interp_status)),
        }
    }
    
    fn close_and_move_files_for_execute(&mut self) -> Result<()>
    {
        for (_, virtual_file) in self.virtual_files.iter_mut() {
            virtual_file.saved_file = None;
            virtual_file.file_stack.clear();
        }
        self.pipe_file_stack.clear();
        let mut fds: HashSet<i32> = HashSet::new();
        for (_, virtual_file) in self.virtual_files.iter() {
            fds.insert(virtual_file.current_file.borrow().as_raw_fd());
        }
        let mut vfds: HashSet<i32> = HashSet::new();
        for (vfd, _) in self.virtual_files.iter() {
            vfds.insert(*vfd);
        }
        for (vfd, virtual_file) in self.virtual_files.iter_mut() {
            if vfds.contains(&vfd) && *vfd != virtual_file.current_file.borrow().as_raw_fd() {
                let mut new_fd = 0;
                loop {
                    if !fds.contains(&new_fd) && !vfds.contains(&new_fd) && !is_fd(new_fd) {
                        break;
                    }
                    new_fd += 1;
                }
                dup2(virtual_file.current_file.borrow().as_raw_fd(), new_fd)?;
                virtual_file.current_file = Rc::new(RefCell::new(unsafe { File::from_raw_fd(new_fd) }));
            }
        }
        for (vfd, virtual_file) in self.virtual_files.iter_mut() {
            if *vfd != virtual_file.current_file.borrow().as_raw_fd() {
                dup2(virtual_file.current_file.borrow().as_raw_fd(), *vfd)?;
                virtual_file.current_file = Rc::new(RefCell::new(unsafe { File::from_raw_fd(*vfd) }));
                let flags = fcntl_f_getfd(*vfd)?;
                fcntl_f_setfd(*vfd, flags & !libc::FD_CLOEXEC)?;
            }
        }
        Ok(())
    }
    
    pub fn execute(&mut self, interp: &mut Interpreter, vars: &[(String, String)], arg0: &str, args: &[String], env: &mut Environment, settings: &mut Settings) -> Result<WaitStatus>
    {
        match env.builtin_fun(arg0) {
            Some(builtin_fun) => {
                let mut tmp_args = vec![String::from(arg0)];
                for arg in args.iter() {
                    tmp_args.push(arg.clone());
                }
                let status = builtin_fun(tmp_args.as_slice(), interp, self, env, settings);
                Ok(WaitStatus::Exited(status))
            },
            None => {
                match env.fun(arg0) {
                    Some(fun_body) => {
                        if !vars.is_empty() {
                            let pid = self.create_process(false, settings, |exec, settings| {
                                    for (name, value) in vars.iter() {
                                        env.set_global_var(name.as_str(), value.as_str());
                                    }
                                    let mut tmp_args = Arguments::new();
                                    for arg in args.iter() {
                                        tmp_args.args.push(arg.clone());
                                    }
                                    settings.push_args(tmp_args);
                                    interp.interpret_fun_body(exec, &(*fun_body), env, settings)
                            })?;
                            let wait_status = self.wait_for_process(pid, true)?;
                            Ok(wait_status)
                        } else {
                            let mut tmp_args = Arguments::new();
                            for arg in args.iter() {
                                tmp_args.args.push(arg.clone());
                            }
                            settings.push_args(tmp_args);
                            let status = interp.interpret_fun_body(self, &(*fun_body), env, settings);
                            Ok(WaitStatus::Exited(status))
                        }
                    },
                    None => {
                        let pid = self.create_process(false, settings, |exec, settings| {
                                for (name, value) in vars.iter() {
                                    env.set_global_var(name.as_str(), value.as_str());
                                }
                                match exec.close_and_move_files_for_execute() {
                                    Ok(()) => {
                                        let mut cmd = Command::new(arg0);
                                        cmd.args(args);
                                        let err = cmd.exec();
                                        eprintln!("{}: {}", arg0, err);
                                        if err.kind() == ErrorKind::NotFound { 127 } else { 126 }
                                    },
                                    Err(err) => {
                                        eprintln!("{}: {}", arg0, err);
                                        127
                                    },
                                }
                        })?;
                        let wait_status = self.wait_for_process(pid, true)?;
                        Ok(wait_status)
                    },
                }
            },
        }
    }
}