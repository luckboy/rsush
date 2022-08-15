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
fn test_arguments_get_option_parses_options()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("-c")
    ];
    args.set_args(tmp_args);
    let opts = "abc";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('b', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('c', c);
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_option_with_separeted_argument()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-b"),
        String::from("abc")
    ];
    args.set_args(tmp_args);
    let opts = "ab:c";
    match args.get_option(opts, None) {
        Ok(Some((c, Some(opt_arg)))) => {
            assert_eq!('b', c);
            assert_eq!(String::from("abc"), opt_arg); 
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_option_with_joint_argument()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-babc")
    ];
    args.set_args(tmp_args);
    let opts = "ab:c";
    match args.get_option(opts, None) {
        Ok(Some((c, Some(opt_arg)))) => {
            assert_eq!('b', c);
            assert_eq!(String::from("abc"), opt_arg); 
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_option_with_separeted_argument_and_other_option()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("abc")
    ];
    args.set_args(tmp_args);
    let opts = "ab:c";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, Some(opt_arg)))) => {
            assert_eq!('b', c);
            assert_eq!(String::from("abc"), opt_arg); 
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_option_with_joint_argument_and_other_option()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ababc")
    ];
    args.set_args(tmp_args);
    let opts = "ab:c";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, Some(opt_arg)))) => {
            assert_eq!('b', c);
            assert_eq!(String::from("abc"), opt_arg); 
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_options_and_arguments()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("xxx"),
        String::from("yyy"),
    ];
    args.set_args(tmp_args);
    let opts = "abc";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('b', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_options_and_minus_and_argument()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("-"),
        String::from("yyy"),
    ];
    args.set_args(tmp_args);
    let opts = "abc";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('b', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_options_and_minus_minus_and_argument()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("--"),
        String::from("yyy"),
    ];
    args.set_args(tmp_args);
    let opts = "abc";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(2, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('b', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, None) {
        Ok(None) => {
            assert_eq!(2, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_parses_options_for_arguments()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-ab"),
        String::from("-c")
    ];
    let opts = "abc";
    match args.get_option(opts, Some(tmp_args.as_slice())) {
        Ok(Some((c, None))) => {
            assert_eq!('a', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(2, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, Some(tmp_args.as_slice())) {
        Ok(Some((c, None))) => {
            assert_eq!('b', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(1, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, Some(tmp_args.as_slice())) {
        Ok(Some((c, None))) => {
            assert_eq!('c', c);
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(2, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
    match args.get_option(opts, Some(tmp_args.as_slice())) {
        Ok(None) => {
            assert_eq!(0, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(2, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_complains_on_unknown_option()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-d"),
    ];
    args.set_args(tmp_args);
    let opts = "abc";
    match args.get_option(opts, None) {
        Err(OptionError::UnknownOption(c)) => {
            assert_eq!('d', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_complains_on_option_requires_argument()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-b"),
    ];
    args.set_args(tmp_args);
    let opts = "ab:c";
    match args.get_option(opts, None) {
        Err(OptionError::OptionRequiresArgument(c)) => {
            assert_eq!('b', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_returns_ques_character_for_unknown_option()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-d"),
    ];
    args.set_args(tmp_args);
    let opts = ":abc";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!('?', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_arguments_get_option_returns_colon_character_for_unknown_option()
{
    let mut args = Arguments::new();
    let tmp_args = vec![
        String::from("-b"),
    ];
    args.set_args(tmp_args);
    let opts = ":ab:c";
    match args.get_option(opts, None) {
        Ok(Some((c, None))) => {
            assert_eq!(':', c);
            assert_eq!(1, args.arg_option_data.index);
            assert_eq!(0, args.arg_option_data.point);
            assert_eq!(0, args.other_option_data.index);
            assert_eq!(0, args.other_option_data.point);
        },
        _ => assert!(false),
    }
}
