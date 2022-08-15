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
use crate::iter::*;

#[derive(Copy, Clone)]
struct OptionData
{
    index: usize,
    point: usize,
}

#[derive(Clone)]
pub struct Arguments
{
    args: Vec<String>,
    arg_option_data: OptionData,
    other_option_data: OptionData,
}

impl Arguments
{
    pub fn new() -> Arguments
    {
        Arguments {
            args: Vec::new(),
            arg_option_data: OptionData {
                index: 0,
                point: 0,
            },
            other_option_data: OptionData {
                index: 0,
                point: 0,
            },
        }
    }
    
    pub fn args(&self) -> &[String]
    { self.args.as_slice() }
    
    pub fn set_args(&mut self, args: Vec<String>)
    { 
        self.args = args;
        self.arg_option_data.index = 0;
        self.arg_option_data.point = 0;
    }
    
    pub fn arg_option_index(&self) -> usize
    { self.arg_option_data.index }
    
    pub fn other_option_index(&self) -> usize
    { self.other_option_data.index }

    pub fn get_option(&mut self, opts: &str, args: Option<&[String]>) -> OptionResult
    {
        let (args, data) = match args {
            Some(args) => (args, &mut self.other_option_data),
            None => (self.args.as_slice(), &mut self.arg_option_data),
        };
        let (opts, is_err) = if opts.starts_with(':') {
            (&opts[1..], false)
        } else {
            (opts, true)
        };
        if data.index > args.len() {
            data.index = 0;
            data.point = 0;
        }
        let mut arg_ics: Option<Vec<(usize, char)>> = args.get(data.index).map(|s| s.char_indices().collect());
        match &arg_ics {
            Some(arg_ics) if  data.point < arg_ics.len() => (),
            Some(_) => {
                data.index = 0;
                data.point = 0;
                arg_ics = args.get(data.index).map(|s| s.char_indices().collect());
            },
            None => (),
        }
        match &arg_ics {
            Some(arg_ics) => {
                if data.point == 0 {
                    if args[data.index] == String::from("--") {
                        data.index += 1;
                        data.point = 0;
                        return Ok(None);
                    }
                    if args[data.index] == String::from("-") {
                        return Ok(None);
                    }
                    if !args[data.index].starts_with('-') {
                        return Ok(None);
                    }
                    data.point += 1;
                }
                let mut opt_iter = PushbackIter::new(opts.chars());
                let mut is_found = false;
                let mut is_opt_arg = false;
                let opt_c = arg_ics[data.point].1;
                loop {
                    match opt_iter.next() {
                        Some(c) => {
                            match opt_iter.next() {
                                Some(':') => is_opt_arg = true,
                                Some(c2) => opt_iter.undo(c2),
                                None => (),
                            }
                            if c == arg_ics[data.point].1 {
                                is_found = true;
                                break;
                            }
                        },
                        None => break,
                    }
                }
                if is_found {
                    if is_opt_arg {
                        let mut opt_arg: Option<String> = None; 
                        match arg_ics.get(data.point + 1) {
                            Some((i, _)) => {
                                opt_arg = Some(String::from(&args[data.index][*i..]));
                                data.index += 1;
                                data.point = 0;
                            },
                            None => {
                                data.index += 1;
                                match args.get(data.index) {
                                    Some(s) => {
                                        opt_arg = Some(s.clone());
                                        data.index += 1;
                                        data.point = 0;
                                    },
                                    None => data.point = 0,
                                }
                            },
                        }
                        match opt_arg {
                            Some(opt_arg) => Ok(Some((opt_c, Some(opt_arg)))),
                            None => {
                                if is_err {
                                    Err(OptionError::OptionRequiresArgument(opt_c))
                                } else {
                                    Ok(Some((':', None)))
                                }
                            },
                        }
                    } else {
                        data.point += 1;
                        if data.point >= arg_ics.len() {
                            data.index += 1;
                            data.point = 0;
                        }
                        Ok(Some((opt_c, None)))
                    }
                } else {
                    data.point += 1;
                    if data.point >= arg_ics.len() {
                        data.index += 1;
                        data.point = 0;
                    }
                    if is_err {
                        Err(OptionError::UnknownOption(opt_c))
                    } else {
                        Ok(Some(('?', None)))
                    }
                }
            },
            None => Ok(None),
        }
    }
}

pub type OptionResult = result::Result<Option<(char, Option<String>)>, OptionError>;

pub enum OptionError
{
    UnknownOption(char),
    OptionRequiresArgument(char),
}

impl fmt::Display for OptionError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            OptionError::UnknownOption(c) => write!(f, "unknown option -- {:?}", c),
            OptionError::OptionRequiresArgument(c) => write!(f, "option requires an argument -- {:?}", c),
        }
    }
}

#[cfg(test)]
mod tests;
