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
use std::fmt::Arguments;
use std::io::*;
use crate::exec::*;

pub fn fprint_args<W: Write>(w: &mut W, args: Arguments<'_>)
{
    match write!(w, "{}", args) {
        Ok(())   => (),
        Err(err) => eprintln!("{}", err),
    }
}

pub fn fprintln_args<W: Write>(w: &mut W, args: Arguments<'_>)
{
    match write!(w, "{}", args) {
        Ok(())   => (),
        Err(err) => eprintln!("{}", err),
    }
}

pub fn xcfprint_args(exec: &Executor, vfd: i32, args: Arguments<'_>)
{
    match exec.current_file(vfd) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            fprint_args(&mut *file_r, args);
        },
        None => eprintln!("No current file"),
    }
}

pub fn xcfprintln_args(exec: &Executor, vfd: i32, args: Arguments<'_>)
{
    match exec.current_file(vfd) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            fprintln_args(&mut *file_r, args);
        },
        None => eprintln!("No current file"),
    }
}

pub fn xsfprint_args(exec: &Executor, vfd: i32, args: Arguments<'_>)
{
    match exec.saved_file(vfd) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            fprint_args(&mut *file_r, args);
        },
        None => eprintln!("No saved file"),
    }
}

pub fn xsfprintln_args(exec: &Executor, vfd: i32, args: Arguments<'_>)
{
    match exec.saved_file(vfd) {
        Some(file) => {
            let mut file_r = file.borrow_mut();
            fprintln_args(&mut *file_r, args);
        },
        None => eprintln!("No saved file"),
    }
}

#[macro_export]
macro_rules! fprint
{
    ($w: expr) => ({
        $crate::fprint_args($w, std::format_args!());
    });
    ($w: expr, $($arg: tt)*) => ({
        $crate::fprint_args($w, std::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! fprintln
{
    ($w: expr) => ({
        $crate::macros::fprintln_args($w, std::format_args!());
    });
    ($w: expr, $($arg: tt)*) => ({
        $crate::macros::fprintln_args($w, std::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! xcfprint
{
    ($exec: expr, $vfd: expr) => ({
        $crate::macros::xcfprint_args($exec, $vfd, std::format_args!());
    });
    ($exec: expr, $vfd: expr, $($arg: tt)*) => ({
        $crate::macros::xcfprint_args($exec, $vfd, std::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! xcfprintln
{
    ($exec: expr, $vfd: expr) => ({
        $crate::macros::xcfprintln_args($exec, $vfd, std::format_args!());
    });
    ($exec: expr, $vfd: expr, $($arg: tt)*) => ({
        $crate::macros::xcfprintln_args($exec, $vfd, std::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! xsfprint
{
    ($exec: expr, $vfd: expr) => ({
        $crate::macros::xsfprint_args($exec, $vfd, std::format_args!());
    });
    ($exec: expr, $vfd: expr, $($arg: tt)*) => ({
        $crate::macros::xsfprint_args($exec, $vfd, std::format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! xsfprintln
{
    ($exec: expr, $vfd: expr) => ({
        $crate::macros::xsfprintln_args($exec, $vfd, std::format_args!());
    });
    ($exec: expr, $vfd: expr, $($arg: tt)*) => ({
        $crate::macros::xsfprintln_args($exec, $vfd, std::format_args!($($arg)*));
    });
}
