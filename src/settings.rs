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
#[derive(Clone)]
pub struct Settings
{
    pub allexport_flag: bool,
    pub errexit_flag: bool,
    pub ignoreeof_flag: bool,
    pub monitor_flag: bool,
    pub noclobber_flag: bool,
    pub noglob_flag: bool,
    pub nolog_flag: bool,
    pub notify_flag: bool,
    pub nounset_flag: bool,
    pub verbose_flag: bool,
    pub vi_flag: bool,
    pub emacs_flag: bool,
    pub xtrace_flag: bool,
}
