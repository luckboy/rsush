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
use std::env;
use std::fs;
use std::fs::*;
use std::io::*;
use std::path::*;
use std::process::exit;
use std::os::unix::fs::symlink;
use crate::utils::*;

pub fn open_file<P: AsRef<Path>>(path: P) -> File
{ File::open(path).unwrap() }

pub fn create_file<P: AsRef<Path>>(path: P) -> File
{
    let mut opts = OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    opts.open(path).unwrap()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> String
{ fs::read_to_string(path).unwrap() }

pub fn write_file<P: AsRef<Path>>(path: P, s: &str)
{ write(path, s.as_bytes()).unwrap(); }

pub fn read_stream<R: Read>(r: &mut R) -> String
{
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    s
}

pub fn write_stream<W: Write>(w: &mut W, s: &str)
{ w.write_all(s.as_bytes()).unwrap(); }

pub fn current_dir() -> PathBuf
{ env::current_dir().unwrap() }

pub fn make_dir<P: AsRef<Path>>(path: P)
{ fs::create_dir(path).unwrap(); }

pub fn make_dir_all<P: AsRef<Path>>(path: P)
{ fs::create_dir_all(path).unwrap(); }

pub fn symlink_rsush_test()
{
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    symlink(format!("{}/target/debug/rsush_test", cargo_manifest_dir), "./rsush_test").unwrap();
}

pub fn remove_rsush_test()
{ remove_file("./rsush_test").unwrap(); }

pub fn create_process_and_wait_for_process<F>(f: F) -> i32
    where F: FnOnce() -> i32
{
    match fork().unwrap() {
        None => {
            let status = f();
            exit(status);
        },
        Some(pid) => {
            let mut status = 0;
            let pid2 = loop {
                match waitpid(pid, Some(&mut status), 0) {
                    Ok(pid) => break pid,
                    Err(err) if err.kind() == ErrorKind::Interrupted => (),
                    res @ Err(_) => {
                        res.unwrap();
                    },
                }
            };
            match pid2 {
                Some(_) => {
                    if libc::WIFEXITED(status) {
                        libc::WEXITSTATUS(status)
                    } else if libc::WIFSIGNALED(status) {
                        libc::WTERMSIG(status) + 128
                    } else if libc::WIFSTOPPED(status) {
                        libc::WSTOPSIG(status) + 128
                    } else {
                        128
                    }
                },
                None => pid2.unwrap(),
            }
        }
    }
}
