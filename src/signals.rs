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
use libc;

pub fn set_signals_for_execute()
{
    unsafe { libc::signal(libc::SIGTTIN, libc::SIG_DFL); }
    unsafe { libc::signal(libc::SIGTTOU, libc::SIG_DFL); }
}

pub fn initialize_signals()
{
    unsafe { libc::signal(libc::SIGTTIN, libc::SIG_IGN); }
    unsafe { libc::signal(libc::SIGTTOU, libc::SIG_IGN); }
}
