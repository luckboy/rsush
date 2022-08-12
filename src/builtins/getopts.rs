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
use crate::utils::*;
use crate::xcfprintln;

pub fn main(_vars: &[(String, String)], args: &[String], _interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    match args.get(1) {
        Some(opts) => {
            match args.get(2) {
                Some(name) => {
                    if !is_name_str(name.as_str()) {
                        xcfprintln!(exec, 2, "{}: Invalid variable name", name);
                        return 1;
                    }
                    if env.read_only_var_attr(name.as_str()) {
                        xcfprintln!(exec, 2, "{}: Is read only", name);
                        return 1;
                    }
                    if env.read_only_var_attr("OPTARG") {
                        xcfprintln!(exec, 2, "OPTARG: Is read only");
                        return 1;
                    }
                    if env.read_only_var_attr("OPTIND") {
                        xcfprintln!(exec, 2, "OPTIND: Is read only");
                        return 1;
                    }
                    let args = if args.len() > 3 {
                        Some(&args[3..])
                    } else {
                        None
                    };
                    match settings.current_args_mut().get_option(opts.as_str(), args) {
                        Ok(None) => {
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            1
                        },
                        Ok(Some((c, opt_arg))) => {
                            env.set_var(name.as_str(), format!("{}", c).as_str(), settings);
                            match opt_arg {
                                Some(opt_arg) => env.set_var("OPTARG", format!("{}", opt_arg).as_str(), settings),
                                None => env.set_var("OPTARG", "", settings),
                            }
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            0
                        },
                        Err(err) => {
                            env.set_var(name.as_str(), "?", settings);
                            env.set_var("OPTARG", "", settings);
                            let opt_index = if args.is_some() {
                                settings.current_args().other_option_index() + 1
                            } else {
                                settings.current_args().arg_option_index() + 1
                            };
                            env.set_var("OPTIND", format!("{}", opt_index).as_str(), settings);
                            xcfprintln!(exec, 2, "{}", err);
                            0
                        },
                    }
                },
                None => {
                    xcfprintln!(exec, 2, "Too few arguments");
                    1
                },
            }
        },
        None => {
            xcfprintln!(exec, 2, "Too few arguments");
            1
        },
    }
}
