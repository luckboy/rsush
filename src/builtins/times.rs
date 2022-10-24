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
use crate::env::*;
use crate::exec::*;
use crate::interp::*;
use crate::settings::*;
use crate::utils::*;
use crate::xcfprintln;
use crate::xsfprintln;

fn minutes_and_seconds_and_mseconds(clk: i64, clk_tck: i64) -> (i64, i64, i64)
{
    let sec = clk / clk_tck;
    let msec = ((clk % clk_tck) * 1000) / clk_tck;
    (sec / 60, sec % 60, msec)
}

pub fn main(_vars: &[(String, String)], _args: &[String], interp: &mut Interpreter, exec: &mut Executor, _env: &mut Environment, _settings: &mut Settings) -> i32
{
    let tmp_clk_tck = match clk_tck() {
        Ok(n) => n,
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            return interp.exit(1, false);
        },
    };
    match times() {
        Ok(tms) => {
            let (utime_min, utime_sec, utime_msec) = minutes_and_seconds_and_mseconds(tms.utime, tmp_clk_tck);
            let (stime_min, stime_sec, stime_msec) = minutes_and_seconds_and_mseconds(tms.stime, tmp_clk_tck);
            xcfprintln!(exec, 1, "{}m{}.{:03}s {}m{}.{:03}s", utime_min, utime_sec, utime_msec, stime_min, stime_sec, stime_msec);
            let (cutime_min, cutime_sec, cutime_msec) = minutes_and_seconds_and_mseconds(tms.cutime, tmp_clk_tck);
            let (cstime_min, cstime_sec, cstime_msec) = minutes_and_seconds_and_mseconds(tms.cstime, tmp_clk_tck);
            xcfprintln!(exec, 1, "{}m{}.{:03}s {}m{}.{:03}s", cutime_min, cutime_sec, cutime_msec, cstime_min, cstime_sec, cstime_msec);
            0
        },
        Err(err) => {
            xsfprintln!(exec, 2, "{}", err);
            interp.exit(1, false)
        },
    }
}
