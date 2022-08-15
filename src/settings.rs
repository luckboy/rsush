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
use std::fmt;
use std::result;
use crate::args::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OptionType
{
    Minus,
    Plus,
}

impl fmt::Display for OptionType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            OptionType::Minus => write!(f, "-"),
            OptionType::Plus => write!(f, "+"),
        }
    }
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
    pub noexec_flag: bool,
    pub nolog_flag: bool,
    pub notify_flag: bool,
    pub nounset_flag: bool,
    pub verbose_flag: bool,
    pub vi_flag: bool,
    pub xtrace_flag: bool,
    pub strlossy_flag: bool,
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
            noexec_flag: false,
            nolog_flag: false,
            notify_flag: true,
            nounset_flag: false,
            verbose_flag: false,
            vi_flag: false,
            xtrace_flag: false,
            strlossy_flag: false,
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

    pub fn parse_options<F>(&mut self, args: &[String], mut f: F) -> OptionResult
        where F: FnMut(OptionType, char, &mut Self) -> bool
    {
        let mut arg_iter = args.iter();
        if arg_iter.next().is_none() {
            return Ok((0, false));
        }
        let mut i: usize = 1;
        let mut is_minus_minus = false;
        loop {
            match arg_iter.next() {
                Some(arg) => {
                    if arg == &String::from("--") {
                        i += 1;
                        is_minus_minus = true;
                        break;
                    } else if arg == &String::from("-") || arg == &String::from("+") {
                        break;
                    }
                    let mut opt_iter = arg.char_indices();
                    match opt_iter.next() {
                        Some((_, c @ ('-' | '+'))) => {
                            let opt_type = if c == '-' {
                                OptionType::Minus
                            } else {
                                OptionType::Plus
                            };
                            loop {
                                let mut is_stop = false;
                                match opt_iter.next() {
                                    Some((_, 'a')) => self.allexport_flag = opt_type == OptionType::Minus,
                                    Some((_, 'e')) => self.errexit_flag = opt_type == OptionType::Minus,
                                    Some((_, 'm')) => self.monitor_flag = opt_type == OptionType::Minus,
                                    Some((_, 'C')) => self.noclobber_flag = opt_type == OptionType::Minus,
                                    Some((_, 'f')) => self.noglob_flag = opt_type == OptionType::Minus,
                                    Some((_, 'n')) => self.noexec_flag = opt_type == OptionType::Minus,
                                    Some((_, 'b')) => self.notify_flag = opt_type == OptionType::Minus,
                                    Some((_, 'u')) => self.nounset_flag = opt_type == OptionType::Minus,
                                    Some((_, 'v')) => self.verbose_flag = opt_type == OptionType::Minus,
                                    Some((_, 'x')) => self.xtrace_flag = opt_type == OptionType::Minus,
                                    Some((_, 'h')) => (),
                                    Some((_, c2 @ 'o')) => {
                                        let opt_arg = match opt_iter.next() {
                                            Some((j, _)) => Some(String::from(&arg[j..])),
                                            None => {
                                                i += 1;
                                                arg_iter.next().map(|s| s.clone())
                                            },
                                        };
                                        match opt_arg {
                                            Some(opt_arg) => {
                                                if opt_arg == String::from("allexport") {
                                                    self.allexport_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("errexit") {
                                                    self.errexit_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("ignoreeof") {
                                                    self.ignoreeof_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("monitor") {
                                                    self.monitor_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("noclobber") {
                                                    self.noclobber_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("noglob") {
                                                    self.noglob_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("noexec") {
                                                    self.noexec_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("nolog") {
                                                    self.nolog_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("notify") {
                                                    self.notify_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("nounset") {
                                                    self.nounset_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("verbose") {
                                                    self.verbose_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("vi") {
                                                    self.vi_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("xtrace") {
                                                    self.xtrace_flag = opt_type == OptionType::Minus;
                                                } else if opt_arg == String::from("strlossy") {
                                                    self.strlossy_flag = opt_type == OptionType::Minus;
                                                } else {
                                                    return Err(OptionError::InvalidArgument);
                                                }
                                            },
                                            None => {
                                                i -= 1;
                                                if !f(opt_type, c2, self) {
                                                    return Err(OptionError::OptionRequiresArgument(opt_type, c2))
                                                }
                                            },
                                        }
                                        is_stop = true;
                                    },
                                    Some((_, c2)) => {
                                        if !f(opt_type, c2, self) {
                                            return Err(OptionError::UnknownOption(opt_type, c2))
                                        }
                                    },
                                    None => break,
                                }
                                if is_stop { break; }
                            }
                        },
                        _ => break,
                    }
                    i += 1;
                },
                None => break,
            }
        }
        Ok((i, is_minus_minus))
    }
    
    pub fn option_string(&self) -> String
    {
        let mut s = String::new();
        if self.allexport_flag { s.push('a'); }
        if self.errexit_flag { s.push('e'); }
        if self.monitor_flag { s.push('m'); }
        if self.noclobber_flag { s.push('C'); }
        if self.noglob_flag { s.push('f'); }
        if self.noexec_flag { s.push('n'); }
        if self.notify_flag { s.push('b'); }
        if self.nounset_flag { s.push('u'); }
        if self.verbose_flag { s.push('v'); }
        if self.xtrace_flag { s.push('x'); }
        s
    }
}

pub type OptionResult = result::Result<(usize, bool), OptionError>;

pub enum OptionError
{
    UnknownOption(OptionType, char),
    OptionRequiresArgument(OptionType, char),
    InvalidArgument,
}

impl fmt::Display for OptionError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            OptionError::UnknownOption(opt_type, c) => write!(f, "unknown option -- '{}{}'", opt_type, c),
            OptionError::OptionRequiresArgument(opt_type, c) => write!(f, "option requires an argument -- '{}{}'", opt_type, c),
            OptionError::InvalidArgument => write!(f, "Invalid argument"),
        }
    }
}

#[cfg(test)]
mod tests;
