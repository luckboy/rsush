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
use std::num::ParseIntError;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::*;
use std::ptr::null_mut;
use std::result;
use std::slice::*;
use fnmatch_sys;
use libc;
use crate::iter::*;

pub struct PipeFds
{
    pub reading_fd: i32,
    pub writing_fd: i32,
}

pub struct Tms
{
    pub utime: i64,
    pub stime: i64,
    pub cutime: i64,
    pub cstime: i64,
}

pub enum GlobResult
{
    Ok(Vec<PathBuf>),
    Aborted,
    NoMatch,
    NoSpace,
}

pub fn is_name_str(s: &str) -> bool
{
    let mut char_iter = s.chars();
    match char_iter.next() {
        Some(c) if c.is_alphabetic() || c == '_' => char_iter.all(|c| c.is_alphanumeric() || c == '_'),
        _ => false,
    }
}

pub fn is_number_str(s: &str) -> bool
{
    let t = if s.starts_with('+') || s.starts_with('-') {
        &s[1..]
    } else {
        s
    };
    if t.starts_with("0X") || t.starts_with("0x") {
        !t[2..].is_empty() && t[2..].chars().all(|c| (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' || c <= 'f'))
    } else if t.starts_with('0') {
        t[1..].chars().all(|c| c >= '0' && c <= '7')
    } else {
        !t.is_empty() && t.chars().all(|c| c >= '0' && c <= '9')
    }
}

pub fn str_to_number(s: &str) -> result::Result<i64, ParseIntError>
{
    let (sign_c, t) = if s.starts_with('+') || s.starts_with('-') {
        (s.chars().next(), &s[1..])
    } else {
        (None, s)
    };
    if t.starts_with("0X") || t.starts_with("0x") {
        let mut new_s = String::new();
        match sign_c {
            Some(sign_c) => new_s.push(sign_c),
            None         => (),
        }
        new_s.push_str(&t[2..]);
        i64::from_str_radix(new_s.as_str(), 16) 
    } else if t.starts_with('0') {
        i64::from_str_radix(s, 8)
    } else {
        s.parse::<i64>()
    }
}

pub fn is_io_number_str(s: &str) -> bool
{ !s.is_empty() && s.chars().all(|c| c >= '0' && c <= '9') }

#[derive(Clone)]
pub enum Mode
{
    Number(u32),
    Symbol(Vec<ModeClause>),
}

#[derive(Clone)]
pub struct ModeClause
{
    who_list: ModeWhoList,
    action_list: Vec<ModeAction>,
}

#[derive(Copy, Clone)]
struct ModeWhoList
{
    has_user: bool,
    has_group: bool,
    has_other: bool,
}

#[derive(Copy, Clone)]
struct ModeAction
{
    op: ModeOp,
    perm: ModePermListOrPermCopy,
}

#[derive(Copy, Clone)]
enum ModeOp
{
    Add,
    Delete,
    Set,
}

#[derive(Copy, Clone)]
enum ModePermListOrPermCopy
{
    List(ModePermList),
    Copy(ModePermCopy),
}

#[derive(Copy, Clone)]
struct ModePermList
{
    has_reading: bool,
    has_writing: bool,
    has_executing: bool,
    has_searching: bool,
    has_set_id: bool,
    has_sticky: bool,
}

#[derive(Copy, Clone)]
enum ModePermCopy
{
    User,
    Group,
    Other,
}

impl Mode
{
    pub fn parse(s: &str) -> Option<Mode>
    {
        match u32::from_str_radix(s, 8) {
            Ok(x)  => Some(Mode::Number(x & 0o7777)),
            Err(_) => {
                let mut clauses: Vec<ModeClause> = Vec::new();
                for clause_s in s.split(',') {
                    let mut clause_s_iter = PushbackIter::new(clause_s.chars());
                    let mut who_list = ModeWhoList {
                        has_user: false,
                        has_group: false,
                        has_other: false,
                    };
                    let mut action_list: Vec<ModeAction> = Vec::new();
                    loop {
                        match clause_s_iter.next() {
                            Some('a') => {
                                who_list.has_user = true;
                                who_list.has_group = true;
                                who_list.has_other = true;
                            },
                            Some('u') => who_list.has_user = true,
                            Some('g') => who_list.has_group = true,
                            Some('o') => who_list.has_other = true,
                            Some(c)   => {
                                clause_s_iter.undo(c);
                                break;
                            },
                            None      => break,
                        }
                    }
                    loop {
                        let op = match clause_s_iter.next() {
                            Some('+') => ModeOp::Add,
                            Some('-') => ModeOp::Delete,
                            Some('=') => ModeOp::Set,
                            Some(_)   => return None,
                            None      => break,
                        };
                        let perm_copy = match clause_s_iter.next() {
                            Some('u') => Some(ModePermCopy::User),
                            Some('g') => Some(ModePermCopy::Group),
                            Some('o') => Some(ModePermCopy::Other),
                            Some(c)   => {
                                clause_s_iter.undo(c);
                                None
                            },
                            None      => None,
                        };
                        let action = match perm_copy {
                            Some(perm_copy) => {
                                ModeAction {
                                    op,
                                    perm: ModePermListOrPermCopy::Copy(perm_copy),
                                }
                            },
                            None => {
                                let mut perm_list = ModePermList {
                                    has_reading: false,
                                    has_writing: false,
                                    has_executing: false,
                                    has_searching: false,
                                    has_set_id: false,
                                    has_sticky: false,
                                };
                                loop {
                                    match clause_s_iter.next() {
                                        Some('r') => perm_list.has_reading = true,
                                        Some('w') => perm_list.has_writing = true,
                                        Some('x') => perm_list.has_executing = true,
                                        Some('X') => perm_list.has_searching = true,
                                        Some('s') => perm_list.has_set_id = true,
                                        Some('t') => perm_list.has_sticky = true,
                                        Some(c)   => {
                                            clause_s_iter.undo(c);
                                            break;
                                        },
                                        None      => break,
                                    }
                                }
                                ModeAction {
                                    op,
                                    perm: ModePermListOrPermCopy::List(perm_list),
                                }
                            },
                        };
                        action_list.push(action);
                    }
                    if !action_list.is_empty() {
                        clauses.push(ModeClause {
                                who_list,
                                action_list,
                        });
                    } else {
                        return None
                    }
                }
                Some(Mode::Symbol(clauses))
            },
        }
    }

    pub fn change_mode(&self, mode: u32, is_dir: bool) -> u32
    {
        match self {
            Mode::Number(new_mode) => *new_mode,
            Mode::Symbol(clauses) => {
                let mut current_mode = mode;
                for clause in clauses {
                    let mut who_mode = 0;
                    if clause.who_list.has_user { who_mode |= 0o4700; }
                    if clause.who_list.has_group { who_mode |= 0o2070; }
                    if clause.who_list.has_other { who_mode |= 0o1007; }
                    if !clause.who_list.has_user && !clause.who_list.has_group && !clause.who_list.has_other {
                        let mask = umask(0);
                        umask(mask);
                        who_mode |= 0o7777 & !mask;
                    }
                    for action in &clause.action_list {
                        let mut perm_mode = 0;
                        match action.perm {
                            ModePermListOrPermCopy::List(perm_list) => {
                                if perm_list.has_reading { perm_mode |= 0o444; }
                                if perm_list.has_writing { perm_mode |= 0o222; }
                                if perm_list.has_executing { perm_mode |= 0o111; }
                                if perm_list.has_searching && (is_dir || (current_mode & 0o111) != 0)  { perm_mode |= 0o111; }
                                if perm_list.has_set_id { perm_mode |= 0o6000; }
                                if perm_list.has_sticky { perm_mode |= 0o1000; }
                            },
                            ModePermListOrPermCopy::Copy(ModePermCopy::User) => {
                                perm_mode |= current_mode & 0o700;
                                perm_mode |= (current_mode & 0o700) >> 3;
                                perm_mode |= (current_mode & 0o700) >> 6;
                            },
                            ModePermListOrPermCopy::Copy(ModePermCopy::Group) => {
                                perm_mode |= (current_mode & 0o70) << 3;
                                perm_mode |= current_mode & 0o70;
                                perm_mode |= (current_mode & 0o70) >> 3;
                            },
                            ModePermListOrPermCopy::Copy(ModePermCopy::Other) => {
                                perm_mode |= (current_mode & 0o7) << 6;
                                perm_mode |= (current_mode & 0o7) << 3;
                                perm_mode |= current_mode & 0o7;
                            },
                        }
                        match action.op {
                            ModeOp::Add    => current_mode |= who_mode & perm_mode,
                            ModeOp::Delete => current_mode &= !(who_mode & perm_mode),
                            ModeOp::Set    => current_mode = (current_mode & !who_mode) | (who_mode & perm_mode),
                        }
                    }
                }
                current_mode
            },
        }
    }
}

pub fn mode_to_string(mode: u32) -> String
{
    let mut s = String::new();
    s.push_str("u=");
    if (mode & 0o4000) != 0 { s.push('s'); }
    if (mode & 0o400) != 0 { s.push('r'); }
    if (mode & 0o200) != 0 { s.push('w'); }
    if (mode & 0o100) != 0 { s.push('x'); }
    s.push_str(",g=");
    if (mode & 0o2000) != 0 { s.push('s'); }
    if (mode & 0o40) != 0 { s.push('r'); }
    if (mode & 0o20) != 0 { s.push('w'); }
    if (mode & 0o10) != 0 { s.push('x'); }
    s.push_str(",o=");
    if (mode & 0o1000) != 0 { s.push('t'); }
    if (mode & 0o4) != 0 { s.push('r'); }
    if (mode & 0o2) != 0 { s.push('w'); }
    if (mode & 0o1) != 0 { s.push('x'); }
    s
}

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

pub fn getppid() -> i32
{ unsafe { libc::getppid() } }

pub fn setpgid(pid: i32, pgid: i32) -> Result<()>
{
    let res = unsafe { libc::setpgid(pid, pgid) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn kill(pid: i32, sig: i32) -> Result<()>
{
    let res = unsafe { libc::kill(pid, sig) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn umask(mask: u32) -> u32 
{ unsafe { libc::umask(mask as libc::mode_t) as u32 } }

pub unsafe fn dup2(old_fd: i32, new_fd: i32) -> Result<()>
{
    let res = libc::dup2(old_fd, new_fd);
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub unsafe fn close(fd: i32) -> Result<()>
{
    let res = libc::close(fd);
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

pub unsafe fn fcntl_f_setfd(fd: i32, flags: i32) -> Result<()>
{
    let res = libc::fcntl(fd, libc::F_SETFD, flags);
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

pub fn times() -> Result<Tms>
{
    let mut libc_tms: libc::tms = unsafe { MaybeUninit::uninit().assume_init() };
    let res = unsafe { libc::times(&mut libc_tms as *mut libc::tms) };
    if res != -1 {
        let tms = Tms {
            utime: libc_tms.tms_utime as i64,
            stime: libc_tms.tms_stime as i64,
            cutime: libc_tms.tms_cutime as i64,
            cstime: libc_tms.tms_cstime as i64,
        };
        Ok(tms)
    } else {
        Err(Error::last_os_error())        
    }
}

pub fn clk_tck() -> Result<i64>
{
    let res = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    if res != -1 {
        Ok(res as i64)
    } else {
        Err(Error::last_os_error())        
    }
}

pub fn isatty(fd: i32) -> Result<bool>
{
    let res = unsafe { libc::isatty(fd) };
    if res != -1 {
        Ok(res != 0)
    } else {
        Err(Error::last_os_error())        
    }
}

pub fn getuid() -> u32
{ unsafe { libc::getuid() as u32 } }

pub fn tcsetpgrp(fd: i32, pgrp: i32) -> Result<()>
{
    let res = unsafe { libc::tcsetpgrp(fd, pgrp) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn pipe_with_cloexec() -> Result<PipeFds>
{
    let pipe_fds = pipe()?;
    match fcntl_f_getfd(pipe_fds.reading_fd) {
        Ok(flags) => {
            match unsafe { fcntl_f_setfd(pipe_fds.reading_fd, flags | libc::FD_CLOEXEC) } {
                Ok(()) => {
                    match fcntl_f_getfd(pipe_fds.writing_fd) {
                        Ok(flags2) => {
                            match unsafe { fcntl_f_setfd(pipe_fds.writing_fd, flags2 | libc::FD_CLOEXEC) } {
                                Ok(()) => Ok(pipe_fds),
                                Err(err) => {
                                    let _res = unsafe { close(pipe_fds.reading_fd) };
                                    let _res2 = unsafe { close(pipe_fds.writing_fd) };
                                    Err(err)
                                },
                            }
                        },
                        Err(err) => {
                            let _res = unsafe { close(pipe_fds.reading_fd) };
                            let _res2 = unsafe { close(pipe_fds.writing_fd) };
                            Err(err)
                        },
                    }
                },
                Err(err) => {
                    let _res = unsafe { close(pipe_fds.reading_fd) };
                    let _res2 = unsafe { close(pipe_fds.writing_fd) };
                    Err(err)
                },
            }
        },
        Err(err) => {
            let _res = unsafe { close(pipe_fds.reading_fd) };
            let _res2 = unsafe { close(pipe_fds.writing_fd) };
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
    let mut iter = s.as_ref().as_bytes().iter();
    loop {
        match iter.next() {
            Some(b'\\') => {
                match iter.next() {
                    Some(c) => buf.push(*c),
                    None => break,
                }
            },
            Some(c) => buf.push(*c),
            None => break,
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
    let spaces = delims.replace(|c: char| !c.is_whitespace(), "");
    let mut fields: Vec<&'a str> = Vec::new();
    let t = if is_space {
        s.trim_matches(|c: char| spaces.contains(c))
    } else {
        s
    };
    if !t.is_empty() {
        let mut iter = PushbackIter::new(t.char_indices());
        let mut i: usize = 0;
        let mut j: usize;
        loop {
            let mut is_first = true;
            let mut is_stop = false;
            loop {
                match iter.next() {
                    Some((k, c)) if is_space && spaces.contains(c) => {
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
                        if is_first { i = t.len(); }
                        j = t.len();
                        is_stop = true;
                        break;
                    },
                }
            }
            fields.push(&t[i..j]);
            if is_stop { break; }
            if is_space {
                loop {
                    match iter.next() {
                        Some((_, c)) if spaces.contains(c) => (),
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
                                Some((_, c2)) if spaces.contains(c2) => (),
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

pub fn is_first_char(s: &str, delims: &str) -> bool
{ s.chars().next().map(|c| delims.contains(c)).unwrap_or(false) }

pub fn is_last_char(s: &str, delims: &str) -> bool
{ s.chars().last().map(|c| delims.contains(c)).unwrap_or(false) }

pub fn str_without_newline(s: &str) -> &str
{
    if s.ends_with('\n') {
        &s[0..(s.len() - 1)]
    } else {
        s
    }
}

pub fn singly_quote_str(s: &str) -> String
{
    let mut new_s = String::new();
    new_s.push('\'');
    new_s.push_str(s.replace('\'', "'\\''").as_str());
    new_s.push('\'');
    new_s
}

#[cfg(test)]
mod tests;
