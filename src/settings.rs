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
#[derive(Clone)]
pub struct Arguments
{
    pub args: Vec<String>,
}

impl Arguments
{
    pub fn new() -> Arguments
    { Arguments { args: Vec::new(), } }
}

#[derive(Clone)]
pub struct Settings
{
    pub allexport_flag: bool,
    pub errexit_flag: bool,
    pub ignoreeof_flag: bool,
    pub monitor_flag: bool,
    pub noclobber_flag: bool,
    pub noglob_flag: bool,
    pub nolog_flag: bool,
    pub notify_flag: bool,
    pub nounset_flag: bool,
    pub verbose_flag: bool,
    pub vi_flag: bool,
    pub emacs_flag: bool,
    pub xtrace_flag: bool,
    pub arg0: String,
    arg_vec_stack: Vec<Arguments>,
    current_args: Arguments,
}

impl Settings
{
    pub fn new() -> Settings
    {
        Settings {
            allexport_flag: false,
            errexit_flag: false,
            ignoreeof_flag: false,
            monitor_flag: true,
            noclobber_flag: false,
            noglob_flag: false,
            nolog_flag: false,
            notify_flag: false,
            nounset_flag: false,
            verbose_flag: false,
            vi_flag: false,
            emacs_flag: false,
            xtrace_flag: false,
            arg0: String::new(),
            arg_vec_stack: Vec::new(),
            current_args: Arguments::new(),
        }
    }
    
    pub fn push_args(&mut self, args: Arguments)
    {
        self.arg_vec_stack.push(self.current_args.clone());
        self.current_args = args;
    }
    
    pub fn pop_args(&mut self)
    {
        match self.arg_vec_stack.pop() {
            Some(args) => self.current_args = args,
            None => (),
        }
    }
    
    pub fn current_args(&self) -> &Arguments
    { &self.current_args }

    pub fn current_args_mut(&mut self) -> &mut Arguments
    { &mut self.current_args }
}
