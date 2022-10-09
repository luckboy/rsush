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
use std::mem::MaybeUninit;
use std::ptr::null;
use std::ptr::null_mut;
use libc;

pub const MAX_SIGNAL_COUNT: i32 = 257;

#[derive(Copy, Clone)]
pub struct SigAction
{
    libc_sigaction: libc::sigaction,
}

static mut SIGNAL_FLAGS: [bool; MAX_SIGNAL_COUNT as usize] = [false; MAX_SIGNAL_COUNT as usize];

extern "C" fn signal_handler(sig: libc::c_int)
{
    if sig >= 0 && sig < MAX_SIGNAL_COUNT {
        unsafe { SIGNAL_FLAGS[sig as usize] = true; }
    }
}

pub fn has_signal(sig: i32) -> bool
{
    if sig >= 0 && sig < MAX_SIGNAL_COUNT {
        unsafe { SIGNAL_FLAGS[sig as usize] }
    } else {
        false
    }
}

pub fn set_signal_flag(sig: i32)
{
    if sig >= 0 && sig < MAX_SIGNAL_COUNT {
        unsafe { SIGNAL_FLAGS[sig as usize] = true; }
    }
}

pub fn unset_signal_flag(sig: i32)
{
    if sig >= 0 && sig < MAX_SIGNAL_COUNT {
        unsafe { SIGNAL_FLAGS[sig as usize] = false; }
    }
}

pub fn set_signal(sig: i32, is_handler: bool, is_interactive: bool) -> Result<()>
{
    let mut sigact: libc::sigaction = unsafe { MaybeUninit::uninit().assume_init() };
    if is_handler {
        sigact.sa_sigaction = signal_handler as libc::sighandler_t;
    } else {
        if is_interactive && (sig == libc::SIGINT || sig == libc::SIGTTIN || sig == libc::SIGTTOU) {
            sigact.sa_sigaction = libc::SIG_IGN;
        } else {
            sigact.sa_sigaction = libc::SIG_DFL;
        }
    }
    unsafe { libc::sigfillset(&mut sigact.sa_mask as *mut libc::sigset_t); }
    sigact.sa_flags = 0;
    let res = unsafe { libc::sigaction(sig, &sigact as *const libc::sigaction, null_mut()) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn get_sigaction(sig: i32) -> Result<SigAction>
{
    let mut sigact: SigAction = unsafe { MaybeUninit::uninit().assume_init() };
    let res = unsafe { libc::sigaction(sig, null(), &mut sigact.libc_sigaction as *mut libc::sigaction) };
    if res != -1 {
        Ok(sigact)
    } else {
        Err(Error::last_os_error())
    }
}

pub fn set_sigaction(sig: i32, sigact: &SigAction) -> Result<()>
{
    let res = unsafe { libc::sigaction(sig, &sigact.libc_sigaction as *const libc::sigaction, null_mut()) };
    if res != -1 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub fn get_sigaction_for_interrupt() -> SigAction
{ get_sigaction(libc::SIGINT).unwrap() }

pub fn set_sigaction_for_interrupt(sigact: &SigAction)
{ let _res = set_sigaction(libc::SIGINT, sigact); }

pub fn set_signals_for_execute()
{
    let _res1 = set_signal(libc::SIGINT, false, false);
    let _res2 = set_signal(libc::SIGTTIN, false, false);
    let _res3 = set_signal(libc::SIGTTOU, false, false);
}

pub fn initialize_signals(is_interactive: bool)
{
    let _res1 = set_signal(libc::SIGINT, false, is_interactive);
    let _res2 = set_signal(libc::SIGTTIN, false, is_interactive);
    let _res3 = set_signal(libc::SIGTTOU, false, is_interactive);
}
