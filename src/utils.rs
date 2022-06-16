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
use std::ptr::null_mut;
use libc;

pub fn is_number_str(s: &str) -> bool
{ s.chars().all(|c| c >= '0' && c <= '9') }

pub fn fork() -> Result<Option<i32>>
{
    let res = unsafe { libc::fork() };
    if res != -1 {
        if res == 0 {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    } else {
        Err(Error::last_os_error())
    }
}

pub fn waitpid(pid: i32, status: Option<&mut i32>, opts: i32) -> Result<Option<i32>>
{
    let res = match status {
        Some(status) => unsafe { libc::waitpid(pid, status as *mut i32, opts) },
        None         => unsafe { libc::waitpid(pid, null_mut() as *mut i32, opts) },
    };
    if res == -1 {
        if res == 0 {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    } else {
        Err(Error::last_os_error())
    }
}

pub fn setpgid(pid: i32, pgid: i32) -> Result<()>
{
    let res = unsafe { libc::setpgid(pid, pgid) };
    if res == -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn dup2(old_fd: i32, new_fd: i32) -> Result<()>
{
    let res = unsafe { libc::dup2(old_fd, new_fd) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn fcntl_f_getfd(fd: i32) -> Result<i32>
{
    let res = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if res != -1 {
        Ok(res)
    } else {
        Err(Error::last_os_error())
    }
}

pub fn fcntl_f_setfd(fd: i32, flags: i32) -> Result<()>
{
    let res = unsafe { libc::fcntl(fd, libc::F_SETFD, flags) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn is_fd(fd: i32) -> bool
{
    match fcntl_f_getfd(fd) {
        Ok(_)  => true,
        Err(_) => false,
    }
}
