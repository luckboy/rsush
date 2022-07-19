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
fn test_split_str_for_ifs_returns_fields_for_delimiters_without_space()
{
    let fields = split_str_for_ifs("abc,def:ghi", ",:");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_first_field_for_delimiters_without_space()
{
    let fields = split_str_for_ifs(",abc:def", ",:");
    assert_eq!(vec!["", "abc", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_center_field_for_delimiters_without_space()
{
    let fields = split_str_for_ifs("abc,:def", ",:");
    assert_eq!(vec!["abc", "", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_last_field_for_delimiters_without_space()
{
    let fields = split_str_for_ifs("abc,def:", ",:");
    assert_eq!(vec!["abc", "def", ""], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_empty_string_and_delimiters_without_space()
{
    let fields = split_str_for_ifs("", ",:");
    assert_eq!(Vec::<&str>::new(), fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_without_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc,def:ghi", ",: \t");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc\t,\t def  : ghi", ",: \t");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_first_field_for_string_without_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs(",abc:def", ",: \t");
    assert_eq!(vec!["", "abc", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_first_field_for_string_with_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs(" , abc\t:\tdef", ",: \t");
    assert_eq!(vec!["", "abc", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_center_field_for_string_without_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc,:def", ",: \t");
    assert_eq!(vec!["abc", "", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_center_field_for_string_with_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc , \t:\tdef", ",: \t");
    assert_eq!(vec!["abc", "", "def"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_last_field_for_string_without_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc,def:", ",: \t");
    assert_eq!(vec!["abc", "def", ""], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_with_empty_last_field_for_string_with_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("abc , def\t:\t", ",: \t");
    assert_eq!(vec!["abc", "def", ""], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_only_space_separators_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("  abc\t def\t ghi  ", ",: \t");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_two_empty_fields_for_delimiters_with_spaces()
{
    let fields = split_str_for_ifs(" \t,\t ", ",: \t");
    assert_eq!(vec!["", ""], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_only_spaces_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("  ", ",: \t");
    assert_eq!(Vec::<&str>::new(), fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_empty_string_and_delimiters_with_spaces()
{
    let fields = split_str_for_ifs("", ",: \t");
    assert_eq!(Vec::<&str>::new(), fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_delimiters_with_only_spaces()
{
    let fields = split_str_for_ifs("abc  def\t ghi", " \t");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_first_spaces_and_last_spaces_and_delimiters_with_only_spaces()
{
    let fields = split_str_for_ifs("\t\tabc  def\t ghi  ", " \t");
    assert_eq!(vec!["abc", "def", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_only_spaces_and_delimiters_with_only_spaces()
{
    let fields = split_str_for_ifs("  ", " \t");
    assert_eq!(Vec::<&str>::new(), fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_empty_string_and_delimiters_with_only_spaces()
{
    let fields = split_str_for_ifs("", " \t");
    assert_eq!(Vec::<&str>::new(), fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_string_with_spaces_and_delimiters_with_one_space()
{
    let fields = split_str_for_ifs("abc\t,  \tdef\t  : ghi", ",: ");
    assert_eq!(vec!["abc\t", "\tdef\t", "ghi"], fields);
}

#[test]
fn test_split_str_for_ifs_returns_fields_for_delimiters_with_only_one_space()
{
    let fields = split_str_for_ifs("abc  def\t \tghi", " ");
    assert_eq!(vec!["abc", "def\t", "\tghi"], fields);
}
