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
use std::fs::*;
use std::io::*;
use std::os::unix::io::FromRawFd;
use std::process::exit;

fn main()
{
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(applet_name) => {
            if applet_name == &String::from("args") {
                for arg in (&args[2..]).iter() {
                    println!("{}", arg);
                }
            } else if applet_name == &String::from("env") {
                for (name, value) in env::vars() {
                    println!("{}={}", name, value);
                }
            } else if applet_name == &String::from("exit") {
                match args.get(2) {
                    Some(s) => {
                        match s.parse::<i32>() {
                            Ok(status) => exit(status),
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1);
                            },
                        }
                    },
                    None => {
                        eprintln!("Too few arguments");
                        exit(1);
                    },
                }
            } else if applet_name == &String::from("read_fd") {
                match args.get(2) {
                    Some(s) => {
                        match s.parse::<i32>() {
                            Ok(fd) => {
                                match args.get(3) {
                                    Some(s2) => {
                                        match s2.parse::<usize>() {
                                            Ok(size) => {
                                                let mut buf: Vec<u8> = vec![0; size];
                                                let mut file = unsafe { File::from_raw_fd(fd) };
                                                match file.read_exact(buf.as_mut_slice()) {
                                                    Ok(()) => {
                                                        println!("{}", String::from_utf8_lossy(buf.as_slice()));
                                                    },
                                                    Err(err) => {
                                                        eprintln!("{}", err);
                                                        exit(1);
                                                    },
                                                }
                                            },
                                            Err(err) => {
                                                eprintln!("{}", err);
                                                exit(1);
                                            },
                                        }
                                    },
                                    None => {
                                        eprintln!("Too few arguments");
                                        exit(1);
                                    },
                                }
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1);
                            },
                        }
                    },
                    None => {
                        eprintln!("Too few arguments");
                        exit(1);
                    },
                }
            } else if applet_name == &String::from("write_fd") {
                match args.get(2) {
                    Some(s) => {
                        match s.parse::<i32>() {
                            Ok(fd) => {
                                let mut file = unsafe { File::from_raw_fd(fd) };
                                for arg in (&args[3..]).iter() {
                                    match writeln!(file, "{}", arg) {
                                        Ok(()) => (),
                                        Err(err) => {
                                            eprintln!("{}", err);
                                            exit(1);
                                        },
                                    }
                                }
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1);
                            },
                        }
                    },
                    None => {
                        eprintln!("Too few arguments");
                        exit(1);
                    },
                }
            } else if applet_name == &String::from("read_2fds") {
                match args.get(2) {
                    Some(s) => {
                        match s.parse::<i32>() {
                            Ok(fd) => {
                                match args.get(3) {
                                    Some(s2) => {
                                        match s2.parse::<usize>() {
                                            Ok(size) => {
                                                match args.get(4) {
                                                    Some(s3) => {
                                                        match s3.parse::<i32>() {
                                                            Ok(fd2) => {
                                                                match args.get(5) {
                                                                    Some(s4) => {
                                                                        match s4.parse::<usize>() {
                                                                            Ok(size2) => {
                                                                                let mut buf: Vec<u8> = vec![0; size];
                                                                                let mut file = unsafe { File::from_raw_fd(fd) };
                                                                                match file.read_exact(buf.as_mut_slice()) {
                                                                                    Ok(()) => {
                                                                                        println!("{}", String::from_utf8_lossy(buf.as_slice()));
                                                                                    },
                                                                                    Err(err) => {
                                                                                        eprintln!("{}", err);
                                                                                        exit(1);
                                                                                    },
                                                                                }
                                                                                let mut buf2: Vec<u8> = vec![0; size2];
                                                                                let mut file2 = unsafe { File::from_raw_fd(fd2) };
                                                                                match file2.read_exact(buf2.as_mut_slice()) {
                                                                                    Ok(()) => {
                                                                                        println!("{}", String::from_utf8_lossy(buf2.as_slice()));
                                                                                    },
                                                                                    Err(err) => {
                                                                                        eprintln!("{}", err);
                                                                                        exit(1);
                                                                                    },
                                                                                }
                                                                            },
                                                                            Err(err) => {
                                                                                eprintln!("{}", err);
                                                                                exit(1);
                                                                            },
                                                                        }
                                                                    },
                                                                    None => {
                                                                        eprintln!("Too few arguments");
                                                                        exit(1);
                                                                    },
                                                                }
                                                            },
                                                            Err(err) => {
                                                                eprintln!("{}", err);
                                                                exit(1);
                                                            },
                                                        }
                                                    },
                                                    None => {
                                                        eprintln!("Too few arguments");
                                                        exit(1);
                                                    },
                                                }
                                            },
                                            Err(err) => {
                                                eprintln!("{}", err);
                                                exit(1);
                                            },
                                        }
                                    },
                                    None => {
                                        eprintln!("Too few arguments");
                                        exit(1);
                                    },
                                }
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1);
                            },
                        }
                    },
                    None => {
                        eprintln!("Too few arguments");
                        exit(1);
                    },
                }
            } else if applet_name == &String::from("write_2fds") {
                match args.get(2) {
                    Some(s) => {
                        match s.parse::<i32>() {
                            Ok(fd) => {
                                match args.get(3) {
                                    Some(s2) => {
                                        match s2.parse::<i32>() {
                                            Ok(fd2) => {
                                                match args.get(4) {
                                                    Some(s3) => {
                                                        match s3.parse::<usize>() {
                                                            Ok(mut n) => {
                                                                n = if n < args.len() - 5 {
                                                                    n
                                                                } else {
                                                                    args.len() - 5
                                                                };
                                                                let args2 = &args[5..];
                                                                let mut file = unsafe { File::from_raw_fd(fd) };
                                                                for arg in (&args2[0..n]).iter() {
                                                                    match writeln!(file, "{}", arg) {
                                                                        Ok(()) => (),
                                                                        Err(err) => {
                                                                            eprintln!("{}", err);
                                                                            exit(1);
                                                                        },
                                                                    }
                                                                }
                                                                let mut file2 = unsafe { File::from_raw_fd(fd2) };
                                                                for arg in (&args2[n..]).iter() {
                                                                    match writeln!(file2, "{}", arg) {
                                                                        Ok(()) => (),
                                                                        Err(err) => {
                                                                            eprintln!("{}", err);
                                                                            exit(1);
                                                                        },
                                                                    }
                                                                }
                                                            },
                                                            Err(err) => {
                                                                eprintln!("{}", err);
                                                                exit(1);
                                                            },
                                                        }
                                                    },
                                                    None => {
                                                        eprintln!("Too few arguments");
                                                        exit(1);
                                                    },
                                                }
                                            },
                                            Err(err) => {
                                                eprintln!("{}", err);
                                                exit(1);
                                            },
                                        }
                                    },
                                    None => {
                                        eprintln!("Too few arguments");
                                        exit(1);
                                    },
                                }
                            },
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1);
                            },
                        }
                    },
                    None => {
                        eprintln!("Too few arguments");
                        exit(1);
                    },
                }
            }
        },
        None => {
            eprintln!("Too few arguments");
            exit(1);
        },
    }
}
