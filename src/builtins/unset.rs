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
use getopt;
use getopt::Opt;
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::xsfprintln;

struct Options
{
    fun_flag: bool,
    var_flag: bool,
}

pub fn main(_vars: &[(String, String)], args: &[String], interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, _settings: &mut Settings) -> i32
{
    let mut opt_parser = getopt::Parser::new(args, "fv");
    let mut opts = Options {
        fun_flag: false,
        var_flag: false,
    };
    loop {
        match opt_parser.next() {
            Some(Ok(Opt('f', _))) => opts.fun_flag = true,
            Some(Ok(Opt('v', _))) => opts.var_flag = true,
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
    let names: Vec<&String> = args.iter().skip(opt_parser.index()).collect();
    for name in names {
        if opts.fun_flag && opts.var_flag {
            if env.read_only_var_attr(name.as_str()) {
                xsfprintln!(exec, 2, "{}: Is read only", name);
                return interp.exit(1, false);
            }
            env.unset_var(name.as_str());
            env.unset_fun(name.as_str());
        } else if opts.fun_flag {
            env.unset_fun(name.as_str());
        } else if opts.var_flag {
            if env.read_only_var_attr(name.as_str()) {
                xsfprintln!(exec, 2, "{}: Is read only", name);
                return interp.exit(1, false);
            }
            env.unset_var(name.as_str());
        } else {
            if env.var(name.as_str()).is_some() {
                if env.read_only_var_attr(name.as_str()) {
                    xsfprintln!(exec, 2, "{}: Is read only", name);
                    return interp.exit(1, false);
                }
                env.unset_var(name.as_str());
            } else {
                env.unset_fun(name.as_str());
            }
        }
    }
    0
}
