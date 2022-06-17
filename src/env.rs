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
use std::env;
use std::rc::*;
use crate::builtins::*;
use crate::parser::*;
use crate::settings::*;

#[derive(Clone)]
pub struct Environment
{
    local_vars: HashMap<String, String>,
    funs: HashMap<String, Rc<FunctionBody>>,
    builtin_funs: HashMap<String, BuiltinFunction>,
}

impl Environment
{
    pub fn new() -> Environment
    {
        Environment {
            local_vars: HashMap::new(),
            funs: HashMap::new(),
            builtin_funs: HashMap::new(),
        }
    }

    pub fn local_var(&self, name: &str) -> Option<String>
    { self.local_vars.get(&String::from(name)).map(|v| v.clone()) }

    pub fn set_local_var(&mut self, name: &str, value: &str)
    { self.local_vars.insert(String::from(name), String::from(value)); }

    pub fn unset_local_var(&mut self, name: &str)
    { self.local_vars.remove(&String::from(name)); }

    pub fn global_var(&self, name: &str) -> Option<String>
    { env::var(name).ok() }

    pub fn set_global_var(&mut self, name: &str, value: &str)
    { env::set_var(name, value); }

    pub fn unset_global_var(&mut self, name: &str)
    { env::remove_var(name); }

    pub fn var(&self, name: &str) -> Option<String>
    { self.local_var(name).or(self.global_var(name)) }

    pub fn set_var(&mut self, name: &str, value: &str, settings: &Settings)
    {
        if settings.allexport_flag {
            self.set_global_var(name, value);
        } else {
            if self.local_vars.contains_key(&String::from(name)) {
                self.set_local_var(name, value);
            } else {
                match self.global_var(name) {
                    Some(_) => self.set_global_var(name, value),
                    None    => self.set_local_var(name, value),
                }
            }
        }
    }

    pub fn unset_var(&mut self, name: &str)
    {
        self.unset_local_var(name);
        self.unset_global_var(name);
    }

    pub fn fun(&self, name: &str) -> Option<Rc<FunctionBody>>
    { self.funs.get(&String::from(name)).map(|fb| fb.clone()) }

    pub fn set_fun(&mut self, name: &str, fun_body: &Rc<FunctionBody>)
    { self.funs.insert(String::from(name), fun_body.clone()); }

    pub fn unset_fun(&mut self, name: &str)
    { self.funs.remove(&String::from(name)); }


    pub fn builtin_fun(&self, name: &str) -> Option<BuiltinFunction>
    { self.builtin_funs.get(&String::from(name)).map(|bf| *bf) }

    pub fn set_builtin_fun(&mut self, name: &str, builtin_fun: BuiltinFunction)
    { self.builtin_funs.insert(String::from(name), builtin_fun); }

    pub fn unset_builtin_fun(&mut self, name: &str)
    { self.builtin_funs.remove(&String::from(name)); }
}
