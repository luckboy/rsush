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
use std::cell::*;
use std::fs::*;
use std::io::*;
use std::os::unix::io::FromRawFd;
use std::process::exit;
use std::rc::*;
use rustyline;
use rustyline::config::EditMode;
use rustyline::error::ReadlineError;
use rustyline::Config;
use rustyline::Editor;

#[allow(dead_code)]
mod args;
#[allow(dead_code)]
mod builtins;
#[allow(dead_code)]
mod env;
#[allow(dead_code)]
mod exec;
#[allow(dead_code)]
mod exec_utils;
#[allow(dead_code)]
mod interp;
#[allow(dead_code)]
mod io;
#[allow(dead_code)]
mod iter;
#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod macros;
#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod settings;
#[allow(dead_code)]
mod signals;
#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod vars;

#[allow(dead_code)]
#[cfg(test)]
mod test_builtins;
#[allow(dead_code)]
#[cfg(test)]
mod test_helpers;

use builtins::initialize_builtin_funs;
use env::*;
use exec::*;
use interp::*;
use io::*;
use lexer::*;
use parser::*;
use settings::*;
use signals::set_signal_flag;
use signals::get_sigaction_for_interrupt;
use signals::set_sigaction_for_interrupt;
use signals::initialize_signals;
use utils::*;
use vars::initialize_vars;

const DEFAULT_PS2: &'static str = "> ";

enum CommandFlag
{
    None,
    String,
    Stdin,
}

struct Options
{
    command_flag: CommandFlag,
    interactive_flag: Option<bool>,
}

enum ShellCommands
{
    FromString(String),
    FromFile(Option<String>),
}

struct EditModeFlags
{
    vi_flag: bool,
    emacs_flag: bool,
}

impl EditModeFlags
{
    fn from_settings(settings: &Settings) -> EditModeFlags
    {
        EditModeFlags {
            vi_flag: settings.vi_flag,
            emacs_flag: settings.emacs_flag,
        }
    }
}

fn default_ps1() -> &'static str
{
    if getuid() == 0 {
        "# "
    } else {
        "$ "
    }
}

fn update_jobs(interp: &mut Interpreter, exec: &mut Executor, settings: &Settings)
{
    let jobs: Vec<(u32, Job)> = exec.jobs().iter().map(|p| (*(p.0), p.1.clone())).collect();
    for (job_id, job) in &jobs {
        let mut is_show = false;
        for (i, (pid, status)) in job.pids.iter().zip(job.statuses.iter()).enumerate() {
            match status {
                WaitStatus::None | WaitStatus::Stopped(_) => {
                    match exec.wait_for_process(Some(*pid), false, true, false, settings) {
                        Ok(tmp_wait_status) => {
                            match tmp_wait_status {
                                WaitStatus::None => (),
                                _ => is_show = true,
                            }
                            match tmp_wait_status {
                                WaitStatus::None => (),
                                _ => exec.set_job_status(*job_id, i, tmp_wait_status),
                            }
                        },
                        Err(err) => xsfprint!(exec, 2, "{}", err),
                    }
                },
                _ => (),
            }
        }
        let mut wait_status = WaitStatus::None;
        match job.last_status {
            WaitStatus::None | WaitStatus::Stopped(_) => {
                match exec.wait_for_process(Some(job.last_pid), false, true, false, settings) {
                    Ok(tmp_wait_status) => {
                        wait_status = if is_show || job.show_flag {
                            match tmp_wait_status {
                                WaitStatus::None => job.last_status,
                                _ => tmp_wait_status,
                            }
                        } else {
                            tmp_wait_status
                        };
                        match tmp_wait_status {
                            WaitStatus::None => (),
                            _ => exec.set_job_last_status(*job_id, tmp_wait_status),
                        }
                    },
                    Err(err) => xsfprint!(exec, 2, "{}", err),
                }
            },
            _ => {
                if is_show || job.show_flag {
                    wait_status = job.last_status;
                }
            },
        }
        exec.set_job_show_flag(*job_id, false);
        if settings.notify_flag {
            match wait_status {
                WaitStatus::None => (),
                _ => {
                    let current = if exec.current_job_id().map(|id| id == *job_id).unwrap_or(false) {
                        '+'
                    } else if exec.prev_current_job_id().map(|id| id == *job_id).unwrap_or(false) {
                        '-'
                    } else {
                        ' '
                    };
                    let status = match wait_status {
                        WaitStatus::None => String::new(),
                        WaitStatus::Exited(_) => String::from("Done"),
                        WaitStatus::Signaled(sig, is_coredump) => interp.signal_string(sig, is_coredump),
                        WaitStatus::Stopped(sig) => interp.signal_string(sig, false),
                    };
                    xsfprintln!(exec, 2, "[{}]{} {} {}", job_id, current, status, job.name);
                },
            }
        }
    }
    let jobs: Vec<(u32, Job)> = exec.jobs().iter().map(|p| (*(p.0), p.1.clone())).collect();
    for (job_id, job) in &jobs {
        if job.is_done() {
            exec.remove_job(*job_id);
        }
    }
}

fn intepret_str(s: &str, interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    initialize_signals(false);
    interp.set_action_flag();
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("(command string)", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    parser.set_error_cont(false);
    match parser.parse_logical_commands(&mut lexer, settings) {
        Ok(commands) => interp.interpret_logical_commands(exec, commands.as_slice(), env, settings),
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            1
        },
    }
}

fn interpret_stream(path: &str, cr: &mut dyn CharRead, interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings, is_ignored_eof: bool) -> (i32, bool)
{
    initialize_signals(false);
    interp.set_action_flag();
    let mut lexer = Lexer::new(path, &Position::new(1, 1), cr, 0, is_ignored_eof);
    let mut parser = Parser::new();
    loop {
        match parser.parse_logical_commands_for_line(&mut lexer, settings) {
            Ok(None) => {
                let status = interp.last_status();
                break (status, false);
            },
            Ok(Some(commands)) => {
                if settings.verbose_flag {
                    xsfprint!(exec, 2, "{}", lexer.content_for_verbose());
                    lexer.clear_content_for_verbose();
                }
                let status = interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                if interp.has_break_or_continue_or_return_or_exit() {
                    if interp.has_exit() {
                        break (status, true);
                    }
                    interp.clear_return_state();
                }
                update_jobs(interp, exec, settings);
            }
            Err(err) => {
                xsfprintln!(exec, 2, "{}", err);
                break (1, false);
            }
        }
    }
}

fn interpret_file(path: &str, interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings, is_ignored_eof: bool) -> Result<(i32, bool)>
{
    let mut file = File::open(path)?;
    let mut br = BufReader::new(&mut file);
    let mut cr = CharReader::new(&mut br);
    Ok(interpret_stream(path, &mut cr, interp, exec, env, settings, is_ignored_eof))
}

fn new_rustyline_editor(settings: &Settings) -> rustyline::Result<Editor<()>>
{
    let mut config_builder = Config::builder();
    config_builder = config_builder.auto_add_history(false);
    if settings.vi_flag {
        config_builder = config_builder.edit_mode(EditMode::Vi);
    } else if settings.emacs_flag {
        config_builder = config_builder.edit_mode(EditMode::Emacs);
    }
    let config = config_builder.build();
    Editor::<()>::with_config(config)
}

fn update_rustyline_edit_mode(editor: Editor<()>, old_edit_mode_flags: &EditModeFlags, settings: &Settings) -> rustyline::Result<Editor<()>>
{
    if old_edit_mode_flags.vi_flag != settings.vi_flag || old_edit_mode_flags.emacs_flag != settings.emacs_flag {
        let history: Vec<String> = editor.history().iter().map(|s| s.clone()).collect();
        let mut new_editor = new_rustyline_editor(settings)?;
        for entry in history {
            new_editor.add_history_entry(entry);
        }
        Ok(new_editor)
    } else {
        Ok(editor)
    }
}

fn parse_stdin_str(s: &str, line: u64, settings: &Settings) -> ParserResult<Option<Vec<Rc<LogicalCommand>>>>
{
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("(standard input)", &Position::new(line, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    parser.parse_logical_commands_for_line(&mut lexer, settings)
}

fn interactively_interpret(interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings) -> i32
{
    settings.interactive_flag = true;
    initialize_signals(true);
    interp.set_action_flag();
    exec.set_foreground();
    let _res = setpgid(exec.shell_pid(), exec.shell_pid());
    exec.set_foreground_for_shell(settings);
    match interpret_file("/etc/rsushrc", interp, exec, env, settings, false) {
        Ok((status, is_exit)) => {
            if is_exit { return status; }
        }
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => xsfprintln!(exec, 2, "/etc/rsushrc: {}", err),
    }
    let home = env.var("HOME").unwrap_or(String::from("/"));
    let path = format!("{}/.rsushrc", home);
    match interpret_file(path.as_str(), interp, exec, env, settings, false) {
        Ok((status, is_exit)) => {
            if is_exit { return status; }
        }
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => xsfprintln!(exec, 2, "{}: {}", path, err),
    }
    let mut saved_shell_sigaction = get_sigaction_for_interrupt();
    let mut editor = match new_rustyline_editor(settings) {
        Ok(tmp_editor) => tmp_editor,
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            return 1;
        },
    };
    let mut line: u64 = 1;
    loop {
        let ps1 = env.var("PS1").unwrap_or(String::from(default_ps1()));
        match editor.readline(ps1.as_str()) {
            Ok(buf) => {
                let saved_editor_sigaction = get_sigaction_for_interrupt();
                set_sigaction_for_interrupt(&saved_shell_sigaction);
                if !settings.nolog_flag {
                    editor.add_history_entry(&buf);
                }
                let mut new_line = line;
                let mut lines = buf.clone();
                lines.push('\n');
                new_line += 1;
                let commands = match parse_stdin_str(lines.as_str(), line, settings) {
                    Ok(None) => break interp.last_status(),
                    Ok(Some(tmp_commands)) => Some(tmp_commands),
                    Err(mut err @ ParserError::Syntax(_, _, _, true)) => {
                        loop {
                            saved_shell_sigaction = get_sigaction_for_interrupt();
                            set_sigaction_for_interrupt(&saved_editor_sigaction);
                            let ps2 = env.var("PS2").unwrap_or(String::from(DEFAULT_PS2));
                            match editor.readline(ps2.as_str()) {
                                Ok(buf2) => {
                                    if !settings.nolog_flag {
                                        editor.add_history_entry(&buf2);
                                    }
                                    lines.push_str(buf2.as_str());
                                    lines.push('\n');
                                    new_line += 1;
                                    match parse_stdin_str(lines.as_str(), line, settings) { 
                                        Ok(None) => break None,
                                        Ok(Some(tmp_commands)) => break Some(tmp_commands),
                                        Err(err2 @ ParserError::Syntax(_, _, _, true)) => err = err2,
                                        Err(err2 @ ParserError::Syntax(_, _, _, false)) => {
                                            xsfprintln!(exec, 2, "{}", err2);
                                            break None;
                                        },
                                        Err(err2) => {
                                            xsfprintln!(exec, 2, "{}", err2);
                                            return 1;
                                        },
                                    }
                                },
                                Err(ReadlineError::Interrupted) => {
                                    set_sigaction_for_interrupt(&saved_shell_sigaction);
                                    set_signal_flag(libc::SIGINT);
                                    let saved_last_status = interp.last_status();
                                    match interp.do_actions(exec, env, settings) {
                                        Some(action_status) => {
                                            if interp.has_exit_with_interactive() {
                                                return action_status
                                            }
                                            interp.clear_return_state();
                                            interp.set_last_status(saved_last_status);
                                        },
                                        None => (),
                                    }
                                    set_sigaction_for_interrupt(&saved_editor_sigaction);
                                    xsfprintln!(exec, 2, "{}", err);
                                    break None
                                },
                                Err(ReadlineError::Eof) => {
                                    xsfprintln!(exec, 2, "{}", err);
                                    break None
                                },
                                Err(err2) => {
                                    xsfprintln!(exec, 2, "{}", err2);
                                    return 1;
                                },
                            }
                        }
                    },
                    Err(err @ ParserError::Syntax(_, _, _, false)) => {
                        xsfprintln!(exec, 2, "{}", err);
                        None
                    },
                    Err(err) => {
                        xsfprintln!(exec, 2, "{}", err);
                        return 1;
                    },
                };
                if settings.verbose_flag {
                    xsfprint!(exec, 2, "{}", lines);
                }
                line = new_line;
                match commands {
                    Some(commands) => {
                        let old_edit_mode_flags = EditModeFlags::from_settings(settings);
                        let status = interp.interpret_logical_commands(exec, commands.as_slice(), env, settings);
                        match update_rustyline_edit_mode(editor, &old_edit_mode_flags, settings) {
                            Ok(tmp_editor) => editor = tmp_editor,
                            Err(err) => {
                                xsfprintln!(exec, 2, "{}", err);
                                return 1;
                            }
                        }
                        if interp.has_break_or_continue_or_return_or_exit() {
                            if interp.has_exit_with_interactive() {
                                break status;
                            }
                            interp.clear_return_state();
                        }
                    },
                    None => (),
                }
                saved_shell_sigaction = get_sigaction_for_interrupt();
                set_sigaction_for_interrupt(&saved_editor_sigaction);
                update_jobs(interp, exec, settings);
            }
            Err(ReadlineError::Interrupted) => {
                let saved_editor_sigaction = get_sigaction_for_interrupt();
                set_sigaction_for_interrupt(&saved_shell_sigaction);
                set_signal_flag(libc::SIGINT);
                let saved_last_status = interp.last_status();
                match interp.do_actions(exec, env, settings) {
                    Some(action_status) => {
                        if interp.has_exit_with_interactive() {
                            return action_status
                        }
                        interp.clear_return_state();
                        interp.set_last_status(saved_last_status);
                    },
                    None => (),
                }
                saved_shell_sigaction = get_sigaction_for_interrupt();
                set_sigaction_for_interrupt(&saved_editor_sigaction);
                update_jobs(interp, exec, settings);
            },
            Err(ReadlineError::Eof) => {
                if !settings.ignoreeof_flag {
                    break interp.last_status();
                } else {
                    update_jobs(interp, exec, settings);
                }
            },
            Err(err) => {
                xsfprintln!(exec, 2, "{}", err);
                return 1;
            },
        }
    }
}

fn interpret(shell_commands: ShellCommands, interp: &mut Interpreter, exec: &mut Executor, env: &mut Environment, settings: &mut Settings, opts: &Options) -> i32
{
    exec.push_file_and_set_saved_file(0, Rc::new(RefCell::new(unsafe { File::from_raw_fd(0) })));
    exec.push_file_and_set_saved_file(1, Rc::new(RefCell::new(unsafe { File::from_raw_fd(1) })));
    exec.push_file_and_set_saved_file(2, Rc::new(RefCell::new(unsafe { File::from_raw_fd(2) })));
    let mut status = match shell_commands {
        ShellCommands::FromString(s) => intepret_str(s.as_str(), interp, exec, env, settings),
        ShellCommands::FromFile(None) => {
            if opts.interactive_flag.unwrap_or(isatty(0).unwrap_or(false)) {
                interactively_interpret(interp, exec, env, settings)
            } else {
                let mut br = BufReader::new(stdin());
                let mut cr = CharReader::new(&mut br);
                interpret_stream("(standard input)", &mut cr, interp, exec, env, settings, true).0
            }
        },
        ShellCommands::FromFile(Some(path)) => {
            match interpret_file(path.as_str(), interp, exec, env, settings, true) {
                Ok((tmp_status, _)) => tmp_status,
                Err(err) => {
                    xsfprintln!(exec, 2, "{}: {}", path, err);
                    1
                }
            }
        },
    };
    match interp.do_action(exec, 0, env, settings) {
        Some(action_status) => status = action_status,
        None => (),
    }
    exec.clear_files();
    status
}

fn main()
{
    let shell_args: Vec<String> = std::env::args().collect();
    let mut opts = Options {
        command_flag: CommandFlag::None,
        interactive_flag: None,
    };
    let mut exec = Executor::new();
    let mut interp = Interpreter::new();
    let mut env = Environment::new();
    let mut settings = Settings::new();
    initialize_builtin_funs(&mut env);
    initialize_vars(&mut env);
    match shell_args.get(0) {
        Some(arg0) => settings.arg0 = arg0.clone(),
        None => {
            eprintln!("No shell name");
            exit(1);
        },
    }
    let res = settings.parse_options(shell_args.as_slice(), |opt_type, c, _| {
            match (opt_type, c) {
                (OptionType::Minus, 'c') => {
                    opts.command_flag = CommandFlag::String;
                    true
                },
                (OptionType::Minus, 'i') => {
                    opts.interactive_flag = Some(true);
                    true
                },
                (OptionType::Plus, 'i') => {
                    opts.interactive_flag = Some(false);
                    true
                },
                (OptionType::Minus, 's') => {
                    opts.command_flag = CommandFlag::Stdin;
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, _)) => {
            match opts.command_flag {
                CommandFlag::None => {
                    let file = shell_args.get(i).map(|p| p.clone());
                    let mut args = Vec::new();
                    if shell_args.len() >= i + 1 {
                        args.extend_from_slice(&shell_args[(i + 1)..]);
                    }
                    match &file {
                        Some(file) => settings.arg0 = file.clone(),
                        None => (),
                    }
                    settings.current_args_mut().set_args(args);
                    let status = interpret(ShellCommands::FromFile(file), &mut interp, &mut exec, &mut env, &mut settings, &opts);
                    exit(status);
                },
                CommandFlag::String => {
                    match shell_args.get(i) {
                        Some(s) => {
                            let mut args = Vec::new();
                            if shell_args.len() >= i + 1 {
                                settings.arg0 = shell_args[i + 1].clone();
                            }
                            if shell_args.len() >= i + 2 {
                                args.extend_from_slice(&shell_args[(i + 2)..]);
                            }
                            settings.current_args_mut().set_args(args);
                            let status = interpret(ShellCommands::FromString(s.clone()), &mut interp, &mut exec, &mut env, &mut settings, &opts);
                            exit(status);
                        },
                        None => {
                            eprintln!("No command string");
                            exit(1);
                        },
                    }
                },
                CommandFlag::Stdin => {
                    let mut args = Vec::new();
                    if shell_args.len() >= i {
                        args.extend_from_slice(&shell_args[i..]);
                    }
                    settings.current_args_mut().set_args(args);
                    let status = interpret(ShellCommands::FromFile(None), &mut interp, &mut exec, &mut env, &mut settings, &opts);
                    exit(status);
                },
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
}
