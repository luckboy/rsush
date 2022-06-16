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
use crate::parser::*;
use crate::settings::*;

pub struct Interpreter
{}

impl Interpreter
{
    pub fn new() -> Interpreter
    { Interpreter {} }
    
    pub fn interpret_fun_body(&mut self, exec: &mut Executor, fun_body: &FunctionBody, env: &mut Environment, settings: &mut Settings) -> i32
    { 0 }
}
