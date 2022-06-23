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
use std::ffi::*;
use std::io::*;
use std::mem::MaybeUninit;
use std::path::*;
use std::ptr::null_mut;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::slice::*;
use fnmatch_sys;
use libc;
use crate::iter::*;

pub struct PipeFds
{
    pub reading_fd: i32,
    pub writing_fd: i32,
}

pub enum GlobResult
{
    Ok(Vec<PathBuf>),
    Aborted,
    NoMatch,
    NoSpace,
}

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

pub fn close(fd: i32) -> Result<()>
{
    let res = unsafe { libc::close(fd) };
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

pub fn pipe() -> Result<PipeFds>
{
    let mut libc_pipe_fds: [i32; 2] = [-1, -1];
    let res = unsafe { libc::pipe(&mut libc_pipe_fds as *mut i32) };
    if res != -1 {
        Ok(PipeFds { reading_fd: libc_pipe_fds[0], writing_fd: libc_pipe_fds[1], })
    } else {
        Err(Error::last_os_error())        
    }
}

pub fn pipe_with_cloexec() -> Result<PipeFds>
{
    let pipe_fds = pipe()?;
    match fcntl_f_getfd(pipe_fds.reading_fd) {
        Ok(flags) => {
            match fcntl_f_setfd(pipe_fds.reading_fd, flags | libc::FD_CLOEXEC) {
                Ok(()) => {
                    match fcntl_f_getfd(pipe_fds.writing_fd) {
                        Ok(flags2) => {
                            match fcntl_f_setfd(pipe_fds.writing_fd, flags2 | libc::FD_CLOEXEC) {
                                Ok(()) => Ok(pipe_fds),
                                Err(err) => {
                                    let _res = close(pipe_fds.reading_fd);
                                    let _res = close(pipe_fds.writing_fd);
                                    Err(err)
                                },
                            }
                        },
                        Err(err) => {
                            let _res = close(pipe_fds.reading_fd);
                            let _res = close(pipe_fds.writing_fd);
                            Err(err)
                        },
                    }
                },
                Err(err) => {
                    let _res = close(pipe_fds.reading_fd);
                    let _res = close(pipe_fds.writing_fd);
                    Err(err)
                },
            }
        },
        Err(err) => {
            let _res = close(pipe_fds.reading_fd);
            let _res = close(pipe_fds.writing_fd);
            Err(err)
        },
    }
}

pub fn is_fd(fd: i32) -> bool
{
    match fcntl_f_getfd(fd) {
        Ok(_)  => true,
        Err(_) => false,
    }
}

pub fn glob<S: AsRef<OsStr>>(pattern: S, flags: i32, err_f: Option<extern "C" fn(*const libc::c_char, i32) -> i32>) -> GlobResult
{
    let mut tmp_glob: libc::glob_t = unsafe { MaybeUninit::uninit().assume_init() };
    tmp_glob.gl_offs = 0;
    let pattern_cstring = CString::new(pattern.as_ref().as_bytes()).unwrap();
    let res = unsafe { libc::glob(pattern_cstring.as_ptr(), flags, err_f, &mut tmp_glob as *mut libc::glob_t) };
    match res {
        0 => {
            let mut path_bufs: Vec<PathBuf> = Vec::new();
            let tmp_paths: &[*mut libc::c_char] = unsafe { from_raw_parts_mut(tmp_glob.gl_pathv, tmp_glob.gl_pathc) };
            for i in 0..tmp_glob.gl_pathc {
                let path_len = unsafe { libc::strlen(tmp_paths[i] as *const libc::c_char) };
                let path_osstring = OsString::from(&OsStr::from_bytes(unsafe { from_raw_parts(tmp_paths[i] as *const u8, path_len) }));
                let mut path_buf = PathBuf::new();
                path_buf.push(path_osstring);
                path_bufs.push(path_buf);
            }
            unsafe { libc::globfree(&mut tmp_glob as *mut libc::glob_t); };
            GlobResult::Ok(path_bufs)
        },
        libc::GLOB_ABORTED => GlobResult::Aborted,
        libc::GLOB_NOMATCH => GlobResult::NoMatch,
        libc::GLOB_NOSPACE => GlobResult::NoSpace,
        _ => panic!("unknown glob result"),
    }
}

pub fn fnmatch<S: AsRef<OsStr>, P: AsRef<Path>>(pattern: S, path: P, flags: i32) -> bool
{
    let pattern_cstring = CString::new(pattern.as_ref().as_bytes()).unwrap();
    let path_cstring = CString::new(path.as_ref().as_os_str().as_bytes()).unwrap();
    let res = unsafe { fnmatch_sys::fnmatch(pattern_cstring.as_ptr(), path_cstring.as_ptr(), flags) };
    res == 0
}

pub fn escape_str(s: &str) -> String
{
    let mut new_s = String::new();
    for c in s.chars() {
        match c {
            '\\' | '?' | '*' | '[' | ']' | ':' | '!' | '^' | '-' | '~' => new_s.push('\\'),
            _ => (),
        }
        new_s.push(c);
    }
    new_s
}

pub fn unescape_path_pattern<S: AsRef<OsStr>>(s: S) -> PathBuf
{
    let mut buf: Vec<u8> = Vec::new();
    for c in s.as_ref().as_bytes().iter() {
        if *c != b'\\' {
            buf.push(*c);
        }
    }
    let mut path_buf = PathBuf::new();
    path_buf.push(&OsString::from_vec(buf));
    path_buf
}

pub fn split_str_for_ifs<'a>(s: &'a str, delims: &str) -> Vec<&'a str>
{
    let delims_without_spaces = delims.replace(char::is_whitespace, "");
    let is_space = delims.chars().any(char::is_whitespace);
    let mut fields: Vec<&'a str> = Vec::new();
    let new_s = if is_space {
        s.trim()
    } else {
        s
    };
    if !new_s.is_empty() {
        let mut iter = PushbackIter::new(s.char_indices());
        let mut i: usize = 0;
        let mut j: usize;
        loop {
            let mut is_first = true;
            let mut is_stop = false;
            loop {
                match iter.next() {
                    Some((k, c)) if is_space && c.is_whitespace() => {
                        if is_first { i = k; }
                        iter.undo((k, c));
                        j = k;
                        break;
                    },
                    Some((k, c)) if delims_without_spaces.contains(c) => {
                        if is_first { i = k; }
                        iter.undo((k, c));
                        j = k;
                        break;
                    },
                    Some((k, _)) => {
                        if is_first { i = k; }
                        is_first = false;
                    },
                    None => {
                        if is_first { i = new_s.len(); }
                        j = new_s.len();
                        is_stop = true;
                        break;
                    },
                }
            }
            fields.push(&new_s[i..j]);
            if is_stop { break; }
            if is_space {
                loop {
                    match iter.next() {
                        Some((_, c)) if c.is_whitespace() => (),
                        Some((k, c)) => {
                            iter.undo((k, c));
                            break;
                        },
                        None => break,
                    }
                }
            }
            match iter.next() {
                Some((_, c)) if delims_without_spaces.contains(c) => {
                    if is_space {
                        loop {
                            match iter.next() {
                                Some((_, c2)) if c2.is_whitespace() => (),
                                Some((l, c2)) => {
                                    iter.undo((l, c2));
                                    break;
                                }
                                None => break,
                            }
                        }
                    }
                },
                Some((k, c)) => iter.undo((k, c)),
                None => (),
            }
        }
    }
    fields
}
