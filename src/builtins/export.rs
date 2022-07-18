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
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::fprintln;
use crate::xsfprintln;

struct Options
{
    print_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, _settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "p");
    let mut opts = Options {
        print_flag: false,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('p', _))) => opts.print_flag = true,
            Some(Ok(Opt(c, _))) => {
                xsfprintln!(exec, 2, "unknown option -- {:?}", c);
                return interp.exit(1, false);
            },
            Some(Err(err)) => {
                xsfprintln!(exec, 2, "{}", err);
                return interp.exit(1, false);
            },
            None => break,
        }
    }
    let args: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    for arg in &args[1..] {
        match arg.split_once('=') {
            Some((name, value)) => {
                if !is_name_str(name) {
                    xsfprintln!(exec, 2, "{}: Invalid variable name", name);
                    return interp.exit(1, false);
                }
                if env.read_only_var_attr(name) {
                    xsfprintln!(exec, 2, "{}: Is read only", name);
                    return interp.exit(1, false);
                }
                env.unset_unexported_var(name);
                env.set_exported_var(name, value);
            },
            None => {
                if !is_name_str(arg.as_str()) {
                    xsfprintln!(exec, 2, "{}: Invalid variable name", arg);
                    return interp.exit(1, false);
                }
                match env.var(arg.as_str()) {
                    Some(value) => {
                        env.unset_unexported_var(arg.as_str());
                        env.set_exported_var(arg.as_str(), value.as_str());
                    },
                    None => {
                        env.unset_unexported_var(arg.as_str());
                        env.set_exported_var(arg.as_str(), "");
                    },
                }
            },
        }
    }
    if args.is_empty() || opts.print_flag {
        match exec.current_file(1) {
            Some(stdout_file) => {
                let mut stdout_file_r = stdout_file.borrow_mut();
                let mut line_stdout = LineWriter::new(&mut *stdout_file_r);
                for (name, value) in env.exported_vars() {
                    fprintln!(&mut line_stdout, "export {}={}", name, value);
                }
            },
            None => {
                xsfprintln!(exec, 2, "No standard output");
                return interp.exit(1, false);
            },
        }
    }
    0
}
