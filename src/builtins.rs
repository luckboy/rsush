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
use crate::interp::*;
use crate::settings::*;

mod dot;
mod colon;
mod r#break;
mod r#continue;
mod eval;
mod exec;
mod exit;
mod export;
mod readonly;
mod r#return;
mod shift;
mod unset;

pub type BuiltinFunction = fn(&[(String, String)], &[String], &mut Interpreter, &mut Executor, &mut Environment, &mut Settings) -> i32;

pub fn initialize_builtin_funs(env: &mut Environment)
{
    env.set_builtin_fun(".", dot::main);
    env.set_builtin_fun(":", colon::main);
    env.set_builtin_fun("break", r#break::main);
    env.set_builtin_fun("continue", r#continue::main);
    env.set_builtin_fun("eval", eval::main);
    env.set_builtin_fun("exec", exec::main);
    env.set_builtin_fun("exit", exit::main);
    env.set_builtin_fun("export", export::main);
    env.set_builtin_fun("readonly", readonly::main);
    env.set_builtin_fun("return", r#return::main);
    env.set_builtin_fun("shift", shift::main);
    env.set_builtin_fun("unset", unset::main);
}
