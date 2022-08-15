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
use super::*;

#[test]
fn test_settings_parse_options_parses_options()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    settings.noclobber_flag = true;
    settings.noglob_flag = true;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("+Cf")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
            assert_eq!(false, settings.noclobber_flag);
            assert_eq!(false, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_a_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-a")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_a_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+a")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_e_option()
{
    let mut settings = Settings::new();
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-e")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_e_option()
{
    let mut settings = Settings::new();
    settings.errexit_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+e")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_m_option()
{
    let mut settings = Settings::new();
    settings.monitor_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-m")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.monitor_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_m_option()
{
    let mut settings = Settings::new();
    settings.monitor_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+m")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.monitor_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_c_option()
{
    let mut settings = Settings::new();
    settings.noclobber_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-C")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noclobber_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_c_option()
{
    let mut settings = Settings::new();
    settings.noclobber_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+C")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noclobber_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_f_option()
{
    let mut settings = Settings::new();
    settings.noglob_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-f")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_f_option()
{
    let mut settings = Settings::new();
    settings.noglob_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+f")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_n_option()
{
    let mut settings = Settings::new();
    settings.noexec_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-n")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noexec_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_n_option()
{
    let mut settings = Settings::new();
    settings.noexec_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+n")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noexec_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_b_option()
{
    let mut settings = Settings::new();
    settings.notify_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-b")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.notify_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_b_option()
{
    let mut settings = Settings::new();
    settings.notify_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+b")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.notify_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_u_option()
{
    let mut settings = Settings::new();
    settings.nounset_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-u")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.nounset_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_u_option()
{
    let mut settings = Settings::new();
    settings.nounset_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+u")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.nounset_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_v_option()
{
    let mut settings = Settings::new();
    settings.verbose_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-v")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.verbose_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_v_option()
{
    let mut settings = Settings::new();
    settings.verbose_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+v")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.verbose_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_x_option()
{
    let mut settings = Settings::new();
    settings.xtrace_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-x")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.xtrace_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_x_option()
{
    let mut settings = Settings::new();
    settings.xtrace_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+x")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.xtrace_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_h_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("-h")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_h_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("+h")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_with_arguments()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = true;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("allexport"),
        String::from("+o"),
        String::from("errexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(5, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(false, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_allexport()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("allexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_allexport()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("allexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_errexit()
{
    let mut settings = Settings::new();
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("errexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_errexit()
{
    let mut settings = Settings::new();
    settings.errexit_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("errexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_ignoreeof()
{
    let mut settings = Settings::new();
    settings.ignoreeof_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("ignoreeof")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.ignoreeof_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_ignoreeof()
{
    let mut settings = Settings::new();
    settings.ignoreeof_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("ignoreeof")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.ignoreeof_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_monitor()
{
    let mut settings = Settings::new();
    settings.monitor_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("monitor")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.monitor_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_monitor()
{
    let mut settings = Settings::new();
    settings.monitor_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("monitor")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.monitor_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_noclobber()
{
    let mut settings = Settings::new();
    settings.noclobber_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("noclobber")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noclobber_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_noclobber()
{
    let mut settings = Settings::new();
    settings.noclobber_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("noclobber")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noclobber_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_noglob()
{
    let mut settings = Settings::new();
    settings.noglob_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("noglob")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_noglob()
{
    let mut settings = Settings::new();
    settings.noglob_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("noglob")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_noexec()
{
    let mut settings = Settings::new();
    settings.noexec_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("noexec")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.noexec_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_noexec()
{
    let mut settings = Settings::new();
    settings.noexec_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("noexec")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.noexec_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_nolog()
{
    let mut settings = Settings::new();
    settings.nolog_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("nolog")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.nolog_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_nolog()
{
    let mut settings = Settings::new();
    settings.nolog_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("nolog")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.nolog_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_notify()
{
    let mut settings = Settings::new();
    settings.notify_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("notify")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.notify_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_notify()
{
    let mut settings = Settings::new();
    settings.notify_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("notify")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.nolog_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_nounset()
{
    let mut settings = Settings::new();
    settings.nounset_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("nounset")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.nounset_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_nounset()
{
    let mut settings = Settings::new();
    settings.nounset_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("nounset")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.nounset_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_verbose()
{
    let mut settings = Settings::new();
    settings.verbose_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("verbose")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.verbose_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_verbose()
{
    let mut settings = Settings::new();
    settings.verbose_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("verbose")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.verbose_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_vi()
{
    let mut settings = Settings::new();
    settings.vi_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("vi")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.vi_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_vi()
{
    let mut settings = Settings::new();
    settings.vi_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("vi")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.vi_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_xtrace()
{
    let mut settings = Settings::new();
    settings.xtrace_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("xtrace")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.xtrace_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_xtrace()
{
    let mut settings = Settings::new();
    settings.xtrace_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("xtrace")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.xtrace_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_options_with_strlossy()
{
    let mut settings = Settings::new();
    settings.strlossy_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("strlossy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.strlossy_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_options_with_strlossy()
{
    let mut settings = Settings::new();
    settings.strlossy_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("strlossy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.strlossy_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_option_with_separeted_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("allexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_option_with_separeted_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("allexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_option_with_joint_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-oallexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_option_with_joint_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    let args = vec![
        String::from("test"),
        String::from("+oallexport")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_option_with_separeted_argument_and_other_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-ao"),
        String::from("errexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_option_with_separeted_argument_and_other_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("+ao"),
        String::from("errexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
            assert_eq!(false, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_option_with_joint_argument_and_other_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-aoerrexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_option_with_joint_argument_and_other_option()
{
    let mut settings = Settings::new();
    settings.allexport_flag = true;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("+aoerrexit")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(false, settings.allexport_flag);
            assert_eq!(false, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_and_arguments()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("xxx"),
        String::from("yyy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_and_argument_and_ignored_options()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    settings.noclobber_flag = true;
    settings.noglob_flag = true;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("xxx"),
        String::from("+Cf")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
            assert_eq!(true, settings.noclobber_flag);
            assert_eq!(true, settings.noglob_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_and_minus_and_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("-"),
        String::from("yyy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_and_plus_and_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("+"),
        String::from("yyy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_options_and_minus_minus_and_argument()
{
    let mut settings = Settings::new();
    settings.allexport_flag = false;
    settings.errexit_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-ae"),
        String::from("--"),
        String::from("yyy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(3, i);
            assert_eq!(true, is_minus_minus);
            assert_eq!(true, settings.allexport_flag);
            assert_eq!(true, settings.errexit_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_minus_and_arguments()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("--"),
        String::from("xxx"),
        String::from("yyy")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(true, is_minus_minus);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_other_option()
{
    let mut settings = Settings::new();
    let mut i_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-i")
    ];
    let res = settings.parse_options(args.as_slice(), |opt_type, c, _| {
            match (opt_type, c) {
                (OptionType::Minus, 'i') => {
                    i_flag = true;
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, i_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_other_option()
{
    let mut settings = Settings::new();
    let mut i_flag = false;
    let args = vec![
        String::from("test"),
        String::from("+i")
    ];
    let res = settings.parse_options(args.as_slice(), |opt_type, c, _| {
            match (opt_type, c) {
                (OptionType::Plus, 'i') => {
                    i_flag = true;
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, i_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_minus_o_option_without_argument()
{
    let mut settings = Settings::new();
    let mut o_flag = false;
    let args = vec![
        String::from("test"),
        String::from("-o")
    ];
    let res = settings.parse_options(args.as_slice(), |opt_type, c, _| {
            match (opt_type, c) {
                (OptionType::Minus, 'o') => {
                    o_flag = true;
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, o_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_parses_plus_o_option_without_argument()
{
    let mut settings = Settings::new();
    let mut o_flag = false;
    let args = vec![
        String::from("test"),
        String::from("+o")
    ];
    let res = settings.parse_options(args.as_slice(), |opt_type, c, _| {
            match (opt_type, c) {
                (OptionType::Plus, 'o') => {
                    o_flag = true;
                    true
                },
                _ => false,
            }
    });
    match res {
        Ok((i, is_minus_minus)) => {
            assert_eq!(2, i);
            assert_eq!(false, is_minus_minus);
            assert_eq!(true, o_flag);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_minus_unknown_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("-i")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::UnknownOption(opt_type, c)) => {
            assert_eq!(OptionType::Minus, opt_type);
            assert_eq!('i', c);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_plus_unknown_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("+i")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::UnknownOption(opt_type, c)) => {
            assert_eq!(OptionType::Plus, opt_type);
            assert_eq!('i', c);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_minus_option_requires_argument()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("-o")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::OptionRequiresArgument(opt_type, c)) => {
            assert_eq!(OptionType::Minus, opt_type);
            assert_eq!('o', c);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_plus_option_requires_argument()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("+o")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::OptionRequiresArgument(opt_type, c)) => {
            assert_eq!(OptionType::Plus, opt_type);
            assert_eq!('o', c);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_invalid_argument_for_minus_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("-o"),
        String::from("xxx")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::InvalidArgument) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_settings_parse_options_complains_on_invalid_argument_for_plus_option()
{
    let mut settings = Settings::new();
    let args = vec![
        String::from("test"),
        String::from("+o"),
        String::from("xxx")
    ];
    let res = settings.parse_options(args.as_slice(), |_, _, _| { false });
    match res {
        Err(OptionError::InvalidArgument) => assert!(true),
        _ => assert!(false),
    }
}
