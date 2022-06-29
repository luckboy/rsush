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
use crate::args::*;
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
pub struct Pipe
{
    pub reading_file: Rc<RefCell<File>>,
    pub writing_file: Rc<RefCell<File>>,
}

impl Pipe
{
    pub fn new(reading_file: Rc<RefCell<File>>, writing_file: Rc<RefCell<File>>) -> Pipe
    { Pipe { reading_file, writing_file, } }
    
    pub unsafe fn from_pipe_fds(pipe_fds: &PipeFds) -> Pipe
    {
        Pipe {
            reading_file: Rc::new(RefCell::new(File::from_raw_fd(pipe_fds.reading_fd))),
            writing_file: Rc::new(RefCell::new(File::from_raw_fd(pipe_fds.writing_fd))),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WaitStatus
{
    None,
    Exited(i32),
    Signaled(i32, bool),
    Stopped(i32),
}

#[derive(Clone)]
pub struct Job
{
    pub pid: i32,
    pub status: WaitStatus,
    pub name: String,
    prev_job_id: Option<u32>,
    next_job_id: Option<u32>,
}

impl Job
{
    pub fn new(pid: i32, name: &str) -> Job
    {
        Job {
            pid,
            status: WaitStatus::None,
            name: String::from(name),
            prev_job_id: None,
            next_job_id: None,
        }
    }
}

pub struct Executor
{
    state_stack: Vec<State>,
    current_state: State,
    virtual_files: HashMap<i32, VirtualFile>,
    pipes: Vec<Pipe>,
    exit_status: i32,
    shell_pid: i32,
    jobs: HashMap<u32, Job>,
    current_job_id: Option<u32>,
}

impl Executor
{
    pub fn new() -> Executor
    {
        Executor {
            state_stack: Vec::new(),
            current_state: State::InInterpreter,
            virtual_files: HashMap::new(),
            pipes: Vec::new(),
            exit_status: 0,
            shell_pid: process::id() as i32,
            jobs: HashMap::new(),
            current_job_id: None,
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
    
    pub fn current_file(&mut self, vfd: i32) -> Option<&Rc<RefCell<File>>>
    { self.virtual_files.get(&vfd).map(|vf| &vf.current_file) }
    
    pub fn saved_file(&mut self, vfd: i32) -> Option<&Rc<RefCell<File>>>
    {
        match self.virtual_files.get(&vfd) {
            Some(virtual_file) => virtual_file.saved_file.as_ref(),
            None => None,
        }
    }
    
    pub fn pipes(&self) -> &[Pipe]
    { self.pipes.as_slice() }
    
    pub fn set_pipes(&mut self, pipes: Vec<Pipe>)
    { self.pipes = pipes; }
    
    pub fn clear_pipes(&mut self)
    { self.pipes.clear(); }
    
    pub fn shell_pid(&self) -> i32
    { self.shell_pid }
    
    pub fn jobs(&self) -> &HashMap<u32, Job>
    { &self.jobs }
    
    pub fn current_job_id(&self) -> Option<u32>
    { self.current_job_id }

    pub fn prev_current_job_id(&self) -> Option<u32>
    {
        match self.current_job_id.map(|id| self.jobs.get(&id)).flatten() {
            Some(job) => job.prev_job_id,
            None => None,
        }
    }

    pub fn add_job(&mut self, job: &Job) -> u32
    {
        let mut job_id: u32 = 1;
        loop {
            if !self.jobs.contains_key(&job_id) {
                break;
            }
            job_id += 1;
        }
        match self.current_job_id.map(|id| self.jobs.get_mut(&id)).flatten() {
            Some(tmp_job) => tmp_job.next_job_id = Some(job_id),
            None => (),
        }
        let mut tmp_job = job.clone();
        tmp_job.prev_job_id = self.current_job_id;
        self.jobs.insert(job_id, tmp_job);
        self.current_job_id = Some(job_id);
        job_id
    }
    
    pub fn set_job_status(&mut self, job_id: u32, status: WaitStatus)
    {
        match self.jobs.get_mut(&job_id) {
            Some(job) => job.status = status,
            None => (),
        }
    }
    
    pub fn remove_job(&mut self, job_id: u32)
    {
        let mut prev_job_id: Option<u32> = None;
        let mut next_job_id: Option<u32> = None;
        match self.jobs.get(&job_id) {
            Some(job) => {
                if self.current_job_id.map(|id| id == job_id).unwrap_or(false) {
                    self.current_job_id = job.prev_job_id;
                }
                prev_job_id = job.prev_job_id;
                next_job_id = job.next_job_id;
            },
            None => (),
        }
        match prev_job_id.map(|id| self.jobs.get_mut(&id)).flatten() {
            Some(prev_job) => prev_job.next_job_id = next_job_id,
            None => (),
        }
        match next_job_id.map(|id| self.jobs.get_mut(&id)).flatten() {
            Some(next_job) => next_job.prev_job_id = prev_job_id,
            None => (),
        }
        self.jobs.remove(&job_id);
    }
    
    pub fn interpret<T, F>(&mut self, f: F) -> T
        where F: FnOnce(&mut Self) -> T
    {
        self.push_state(State::InInterpreter);
        let res = f(self);
        self.pop_state();
        res
    }
    
    pub fn interpret_or<T, F>(&mut self, is_interp: bool, f: F) -> T
        where F: FnOnce(&mut Self) -> T
    {
        if is_interp {
            self.push_state(State::InInterpreter);
        }
        let res = f(self);
        if is_interp {
            self.pop_state();
        }
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
                match pid {
                    Some(None) => {
                        for (_, virtual_file) in self.virtual_files.iter_mut() {
                            virtual_file.file_stack.clear();
                        }
                        self.jobs.clear();
                    },
                    _ => ()
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
            Some(Some(pid)) => Ok(Some(pid)),
            None => {
                self.exit_status = status;
                Ok(None)
            },
        }
    }

    pub fn wait_for_process(&self, pid: Option<i32>, is_hang: bool, is_untraced: bool) -> Result<WaitStatus>
    {
        match pid {
            Some(pid) => {
                let mut status = 0;
                let mut opts = if is_hang {
                    0
                } else {
                    libc::WNOHANG
                };
                opts |= if is_untraced {
                    libc::WUNTRACED
                } else {
                    0
                };
                let mut res = Ok(WaitStatus::None);
                loop {
                    let pid2 = loop {
                        match waitpid(pid, Some(&mut status), opts) {
                            Ok(pid) => break pid,
                            Err(err) if err.kind() == ErrorKind::Interrupted => (),
                            Err(err) => return Err(err),
                        }
                    };
                    match pid2 {
                        Some(_) => {
                            if libc::WIFEXITED(status) {
                                res = Ok(WaitStatus::Exited(libc::WEXITSTATUS(status)));
                                break;
                            } else if libc::WIFSIGNALED(status) {
                                res = Ok(WaitStatus::Signaled(libc::WTERMSIG(status), libc::WCOREDUMP(status)));
                                break;
                            } else if libc::WIFSTOPPED(status) {
                                res = Ok(WaitStatus::Stopped(libc::WSTOPSIG(status)));
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
            _  => Ok(WaitStatus::Exited(self.exit_status)),
        }
    }
    
    fn close_and_move_files_for_execute(&mut self) -> Result<()>
    {
        for (_, virtual_file) in self.virtual_files.iter_mut() {
            virtual_file.saved_file = None;
            virtual_file.file_stack.clear();
        }
        self.pipes.clear();
        let mut fds: HashSet<i32> = HashSet::new();
        for (_, virtual_file) in self.virtual_files.iter() {
            fds.insert(virtual_file.current_file.borrow().as_raw_fd());
        }
        let mut vfds: HashSet<i32> = HashSet::new();
        for (vfd, _) in self.virtual_files.iter() {
            vfds.insert(*vfd);
        }
        let mut new_fd = 0;
        for (vfd, virtual_file) in self.virtual_files.iter_mut() {
            if vfds.contains(&vfd) && *vfd != virtual_file.current_file.borrow().as_raw_fd() {
                loop {
                    if !fds.contains(&new_fd) && !vfds.contains(&new_fd) && !is_fd(new_fd) {
                        break;
                    }
                    new_fd += 1;
                }
                loop {
                    match unsafe { dup2(virtual_file.current_file.borrow().as_raw_fd(), new_fd) } {
                        Ok(()) => break,
                        Err(err) if err.kind() == ErrorKind::Interrupted => (),
                        Err(err) => return Err(err),
                    }
                }
                virtual_file.current_file = Rc::new(RefCell::new(unsafe { File::from_raw_fd(new_fd) }));
                new_fd += 1;
            }
        }
        for (vfd, virtual_file) in self.virtual_files.iter_mut() {
            if *vfd != virtual_file.current_file.borrow().as_raw_fd() {
                loop {
                    match unsafe { dup2(virtual_file.current_file.borrow().as_raw_fd(), *vfd) } {
                        Ok(()) => break,
                        Err(err) if err.kind() == ErrorKind::Interrupted => (),
                        Err(err) => return Err(err),
                    }
                }
                virtual_file.current_file = Rc::new(RefCell::new(unsafe { File::from_raw_fd(*vfd) }));
                let flags = fcntl_f_getfd(*vfd)?;
                unsafe { fcntl_f_setfd(*vfd, flags & !libc::FD_CLOEXEC) }?;
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
                                        env.unset_unexported_var(name.as_str());
                                        env.set_exported_var(name.as_str(), value.as_str());
                                    }
                                    let mut tmp_args = Arguments::new();
                                    tmp_args.set_args(args.iter().map(|a| a.clone()).collect());
                                    settings.push_args(tmp_args);
                                    let status = interp.interpret_fun_body(exec, &(*fun_body), env, settings);
                                    settings.pop_args();
                                    status
                            })?;
                            let wait_status = self.wait_for_process(pid, true, false)?;
                            Ok(wait_status)
                        } else {
                            let mut tmp_args = Arguments::new();
                            tmp_args.set_args(args.iter().map(|a| a.clone()).collect());
                            settings.push_args(tmp_args);
                            let status = interp.interpret_fun_body(self, &(*fun_body), env, settings);
                            settings.pop_args();
                            Ok(WaitStatus::Exited(status))
                        }
                    },
                    None => {
                        let pid = self.create_process(false, settings, |exec, _| {
                                for (name, value) in vars.iter() {
                                    env.unset_unexported_var(name.as_str());
                                    env.set_exported_var(name.as_str(), value.as_str());
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
                                        126
                                    },
                                }
                        })?;
                        let wait_status = self.wait_for_process(pid, true, false)?;
                        Ok(wait_status)
                    },
                }
            },
        }
    }
}
