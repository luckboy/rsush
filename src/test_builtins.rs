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

mod test_builtin_args;
mod test_builtin_env;
mod test_builtin_exit;
mod test_builtin_vars;

pub fn initialize_test_builtin_funs(env: &mut Environment)
{
    env.set_builtin_fun("test_builtin_args", test_builtin_args::main);
    env.set_builtin_fun("test_builtin_env", test_builtin_env::main);
    env.set_builtin_fun("test_builtin_exit", test_builtin_exit::main);
    env.set_builtin_fun("test_builtin_vars", test_builtin_vars::main);
}
