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
use std::io::*;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::io::*;
use crate::lexer::*;
use crate::parser::*;
use crate::settings::*;
use crate::xsfprintln;

pub fn main(vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    for (name, value) in vars.iter() {
        if env.read_only_var_attr(name) {
            xsfprintln!(exec, 2, "{}: Is read only", name);
            return interp.exit(1, false);
        }
        env.unset_unexported_var(name.as_str());
        env.set_exported_var(name.as_str(), value.as_str());
    }
    if args.len() < 1 {
        xsfprintln!(exec, 2, "No built-in function name");
        return interp.exit(1, false);
    }
    let s = (&args[1..]).join(" ");
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("evaluation", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    parser.set_error_cont(false);
    match parser.parse_logical_commands(&mut lexer, settings) {
        Ok(commands) => interp.interpret_logical_commands(exec, commands.as_slice(), env, settings),
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            interp.exit(1, false)
        },
    }
}
