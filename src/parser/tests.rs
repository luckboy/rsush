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
use crate::io::*;

#[test]
fn test_parser_parse_words_parses_words()
{
    let s = "abc def ghi";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_words(&mut lexer, &settings) {
        Ok(words) => {
            assert_eq!(3, words.len());
            assert_eq!(String::from("test.sh"), words[0].path);
            assert_eq!(1, words[0].pos.line);
            assert_eq!(1, words[0].pos.column);
            assert_eq!(1, words[0].word_elems.len());
            match &words[0].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), words[1].path);
            assert_eq!(1, words[1].pos.line);
            assert_eq!(5, words[1].pos.column);
            assert_eq!(1, words[1].word_elems.len());
            match &words[1].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("def"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), words[2].path);
            assert_eq!(1, words[2].pos.line);
            assert_eq!(9, words[2].pos.column);
            assert_eq!(1, words[2].word_elems.len());
            match &words[2].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("ghi"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_words_complains_on_unexpected_token()
{
    let s = "abc ) def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_words(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(5, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_command()
{
    let s = "echo abc def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(3, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                    assert_eq!(1, simple_command.words[2].pos.line);
                    assert_eq!(10, simple_command.words[2].pos.column);
                    assert_eq!(1, simple_command.words[2].word_elems.len());
                    match &simple_command.words[2].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_two_commands()
{
    let s = "echo abc; echo def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(1, logical_commands[1].pos.line);
            assert_eq!(11, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(1, logical_commands[1].first_command.pos.line);
            assert_eq!(11, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(11, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(16, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_for_line_parses_command()
{
    let s = "echo abc def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands_for_line(&mut lexer, &settings) {
        Ok(Some(logical_commands)) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(3, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                    assert_eq!(1, simple_command.words[2].pos.line);
                    assert_eq!(10, simple_command.words[2].pos.column);
                    assert_eq!(1, simple_command.words[2].word_elems.len());
                    match &simple_command.words[2].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_for_line_parses_two_commands()
{
    let s = "echo abc; echo def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands_for_line(&mut lexer, &settings) {
        Ok(Some(logical_commands)) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(1, logical_commands[1].pos.line);
            assert_eq!(11, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(1, logical_commands[1].first_command.pos.line);
            assert_eq!(11, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(11, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(16, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_for_line_parses_zero_commands()
{
    let s = "\n";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands_for_line(&mut lexer, &settings) {
        Ok(Some(logical_commands)) => {
            assert_eq!(true, logical_commands.is_empty());
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}    

#[test]
fn test_parser_parse_logical_commands_for_line_parses_eof()
{
    let s = "";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands_for_line(&mut lexer, &settings) {
        Ok(None) => (),
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_two_commands_with_newlines()
{
    let s = "
echo abc
echo def
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(2, logical_commands[1].pos.line);
            assert_eq!(1, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(2, logical_commands[1].first_command.pos.line);
            assert_eq!(1, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(2, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(2, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_two_commands_with_comments()
{
    let s = "
# first comment
echo abc
# second comment
echo def
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(2, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(2, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(2, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(2, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(4, logical_commands[1].pos.line);
            assert_eq!(1, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(4, logical_commands[1].first_command.pos.line);
            assert_eq!(1, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(4, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(4, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(4, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_two_commands_for_command_that_is_in_background()
{
    let s = "
echo abc &
echo def
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(true, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(2, logical_commands[1].pos.line);
            assert_eq!(1, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(2, logical_commands[1].first_command.pos.line);
            assert_eq!(1, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(2, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(2, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_with_logical_operators()
{
    let s = "
echo abc && echo def ||
echo ghi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(2, logical_commands[0].pairs.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(LogicalOperator::And, logical_commands[0].pairs[0].op);
            assert_eq!(String::from("test.sh"), logical_commands[0].pairs[0].command.path);
            assert_eq!(1, logical_commands[0].pairs[0].command.pos.line);
            assert_eq!(13, logical_commands[0].pairs[0].command.pos.column);
            assert_eq!(false, logical_commands[0].pairs[0].command.is_negative);
            assert_eq!(1, logical_commands[0].pairs[0].command.commands.len());
            match &(*logical_commands[0].pairs[0].command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(13, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(13, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(18, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(LogicalOperator::Or, logical_commands[0].pairs[1].op);
            assert_eq!(String::from("test.sh"), logical_commands[0].pairs[1].command.path);
            assert_eq!(2, logical_commands[0].pairs[1].command.pos.line);
            assert_eq!(1, logical_commands[0].pairs[1].command.pos.column);
            assert_eq!(false, logical_commands[0].pairs[1].command.is_negative);
            assert_eq!(1, logical_commands[0].pairs[1].command.commands.len());
            match &(*logical_commands[0].pairs[1].command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(2, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(2, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("ghi"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_with_pipe_operators()
{
    let s = "
echo abc | cat |
tee xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(3, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            match &(*logical_commands[0].first_command.commands[1]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(12, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(12, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            match &(*logical_commands[0].first_command.commands[2]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(2, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("tee"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(2, simple_command.words[1].pos.line);
                    assert_eq!(5, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("xxx"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_which_are_negative()
{
    let s = "
! echo abc | cat
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(true, logical_commands[0].first_command.is_negative);
            assert_eq!(2, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(3, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(3, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(8, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
            match &(*logical_commands[0].first_command.commands[1]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(14, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, simple_command.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_redirections()
{
    let s = "
echo abc > xxx 2>> yyy
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Output(path2, pos2, None, word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(12, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    match &(*simple_command.redirects[1]) {
                        Redirection::Appending(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(16, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(20, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("yyy"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}    

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_redirection()
{
    let s = "
echo abc < xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Input(path2, pos2, None, word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(12, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_redirection()
{
    let s = "
echo abc > xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Output(path2, pos2, None, word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(12, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_redirection_with_bar()
{
    let s = "
echo abc >| xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Output(path2, pos2, None, word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(true, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_and_output_redirection()
{
    let s = "
echo abc <> xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::InputAndOutput(path2, pos2, None, word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_appending_redirection()
{
    let s = "
echo abc >> xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Appending(path2, pos2, None, word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_duplicating_redirection()
{
    let s = "
echo abc <& 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::InputDuplicating(path2, pos2, None, word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("2"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_duplicating_redirection()
{
    let s = "
echo abc >& 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::OutputDuplicating(path2, pos2, None, word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("2"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_here_document_redirection()
{
    let s = "
cat << EOT
abcdef
ghijkl
EOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, None, here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(String::from("EOT"), here_doc.borrow().delim);
                            assert_eq!(false, here_doc.borrow().has_minus);
                            assert_eq!(2, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("abcdef\n"), s);
                                },
                                _ => assert!(false),
                            }
                            match &here_doc.borrow().simple_word_elems[1] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("ghijkl\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_here_document_redirection_with_minus()
{
    let s = "
cat <<- EOT
\tabcdef
\t\tghijkl
\tEOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, None, here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(String::from("EOT"), here_doc.borrow().delim);
                            assert_eq!(true, here_doc.borrow().has_minus);
                            assert_eq!(2, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("abcdef\n"), s);
                                },
                                _ => assert!(false),
                            }
                            match &here_doc.borrow().simple_word_elems[1] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("ghijkl\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_redirection_with_number()
{
    let s = "
echo abc 2< xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Input(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_redirection_with_number()
{
    let s = "
echo abc 2> xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Output(path2, pos2, Some(n), word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(13, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_redirection_with_number_and_bar()
{
    let s = "
echo abc 2>| xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Output(path2, pos2, Some(n), word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(true, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(14, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_and_output_redirection_with_number()
{
    let s = "
echo abc 2<> xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::InputAndOutput(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(14, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_appending_redirection_with_number()
{
    let s = "
echo abc 2>> xxx
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::Appending(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(14, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_input_duplicating_redirection_with_number()
{
    let s = "
echo abc 1<& 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::InputDuplicating(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(1, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(14, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("2"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_output_duplicating_redirection_with_number()
{
    let s = "
echo abc 1>& 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("echo"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                    assert_eq!(1, simple_command.words[1].pos.line);
                    assert_eq!(6, simple_command.words[1].pos.column);
                    assert_eq!(1, simple_command.words[1].word_elems.len());
                    match &simple_command.words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::OutputDuplicating(path2, pos2, Some(n), word) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(10, pos2.column);
                            assert_eq!(1, *n);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(1, word.pos.line);
                            assert_eq!(14, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("2"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_here_document_redirection_with_number()
{
    let s = "
cat 0<< EOT
abcdef
ghijkl
EOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, Some(n), here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(0, *n);
                            assert_eq!(String::from("EOT"), here_doc.borrow().delim);
                            assert_eq!(false, here_doc.borrow().has_minus);
                            assert_eq!(2, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("abcdef\n"), s);
                                },
                                _ => assert!(false),
                            }
                            match &here_doc.borrow().simple_word_elems[1] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("ghijkl\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_command_with_here_document_redirection_with_number_and_minus()
{
    let s = "
cat 0<<- EOT
\tabcdef
\t\tghijkl
\tEOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, Some(n), here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(0, *n);
                            assert_eq!(String::from("EOT"), here_doc.borrow().delim);
                            assert_eq!(true, here_doc.borrow().has_minus);
                            assert_eq!(2, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("abcdef\n"), s);
                                },
                                _ => assert!(false),
                            }
                            match &here_doc.borrow().simple_word_elems[1] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("ghijkl\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_two_commands_with_here_document_redirections()
{
    let s = "
cat << EOT; cat << EOT2
abcdef
EOT
ghijkl
EOT2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(2, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(1, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, None, here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(String::from("EOT"), here_doc.borrow().delim);
                            assert_eq!(false, here_doc.borrow().has_minus);
                            assert_eq!(1, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("abcdef\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), logical_commands[1].path);
            assert_eq!(1, logical_commands[1].pos.line);
            assert_eq!(13, logical_commands[1].pos.column);
            assert_eq!(false, logical_commands[1].is_in_background);
            assert_eq!(true, logical_commands[1].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[1].first_command.path);
            assert_eq!(1, logical_commands[1].first_command.pos.line);
            assert_eq!(13, logical_commands[1].first_command.pos.column);
            assert_eq!(false, logical_commands[1].first_command.is_negative);
            assert_eq!(1, logical_commands[1].first_command.commands.len());
            match &(*logical_commands[1].first_command.commands[0]) {
                Command::Simple(path, pos, simple_command) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(13, pos.column);
                    assert_eq!(1, simple_command.words.len());
                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                    assert_eq!(1, simple_command.words[0].pos.line);
                    assert_eq!(13, simple_command.words[0].pos.column);
                    assert_eq!(1, simple_command.words[0].word_elems.len());
                    match &simple_command.words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("cat"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, simple_command.redirects.len());
                    match &(*simple_command.redirects[0]) {
                        Redirection::HereDocument(path2, pos2, None, here_doc) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(1, pos2.line);
                            assert_eq!(17, pos2.column);
                            assert_eq!(String::from("EOT2"), here_doc.borrow().delim);
                            assert_eq!(false, here_doc.borrow().has_minus);
                            assert_eq!(1, here_doc.borrow().simple_word_elems.len());
                            match &here_doc.borrow().simple_word_elems[0] {
                                SimpleWordElement::String(s) => {
                                    assert_eq!(&String::from("ghijkl\n"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_in_brace_group()
{
    let s = "
{ echo abc; echo def; }
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(1, logical_commands2[0].pos.line);
                    assert_eq!(3, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(1, logical_commands2[0].first_command.pos.line);
                    assert_eq!(3, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(3, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(3, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(1, simple_command.words[1].pos.line);
                            assert_eq!(8, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(1, logical_commands2[1].pos.line);
                    assert_eq!(13, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(1, logical_commands2[1].first_command.pos.line);
                    assert_eq!(13, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(13, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(13, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(1, simple_command.words[1].pos.line);
                            assert_eq!(18, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_in_brace_group_with_newlines()
{
    let s = "
{
    echo abc
    echo def
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_in_brace_group_with_redirections()
{
    let s = "
{
    echo abc
    echo def
} > xxx 2> yyy
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, redirects.len());
                    match &(*redirects[0]) {
                        Redirection::Output(path2, pos2, None, word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(4, pos2.line);
                            assert_eq!(3, pos2.column);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(4, word.pos.line);
                            assert_eq!(5, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    match &(*redirects[1]) {
                        Redirection::Output(path2, pos2, Some(n), word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(4, pos2.line);
                            assert_eq!(9, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(4, word.pos.line);
                            assert_eq!(12, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("yyy"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_in_subshell()
{
    let s = "
(echo abc; echo def)
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Subshell(logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(1, logical_commands2[0].pos.line);
                    assert_eq!(2, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(1, logical_commands2[0].first_command.pos.line);
                    assert_eq!(2, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(2, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(2, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(1, simple_command.words[1].pos.line);
                            assert_eq!(7, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(1, logical_commands2[1].pos.line);
                    assert_eq!(12, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(1, logical_commands2[1].first_command.pos.line);
                    assert_eq!(12, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(12, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(12, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(1, simple_command.words[1].pos.line);
                            assert_eq!(17, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_commands_in_subshell_with_newlines()
{
    let s = "
(
    echo abc
    echo def
)
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Subshell(logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_for_clause()
{
    let s = "
for i in 1 2 3; do
    echo $i
    echo abc
done
    ";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(3, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(10, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("1"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("2"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[2].path);
                    assert_eq!(1, words[2].pos.line);
                    assert_eq!(14, words[2].pos.column);
                    assert_eq!(1, words[2].word_elems.len());
                    match &words[2].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("3"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                                    assert_eq!(&String::from("i"), var_name);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_for_clause_with_newline()
{
    let s = "
for i in 1 2 3
do
    echo $i
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(3, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(10, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("1"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("2"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[2].path);
                    assert_eq!(1, words[2].pos.line);
                    assert_eq!(14, words[2].pos.column);
                    assert_eq!(1, words[2].word_elems.len());
                    match &words[2].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("3"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(3, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(3, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                                    assert_eq!(&String::from("i"), var_name);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(4, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(4, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_for_clause_without_words()
{
    let s = "
for i in; do
    echo $i
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, words.is_empty());
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                                    assert_eq!(&String::from("i"), var_name);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}    

#[test]
fn test_parser_parse_logical_commands_parses_for_clause_without_in_and_words()
{
    let s = "
for i; do
    echo $i
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, words.is_empty());
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                                    assert_eq!(&String::from("i"), var_name);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_for_clause_with_do_instead_of_in_without_words()
{
    let s = "
for i do
    echo $i
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, words.is_empty());
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                                    assert_eq!(&String::from("i"), var_name);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(3, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(3, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_for_clause_with_nested_compound_command()
{
    let s = "
for i in 1 2 3; do
    { echo abc; echo def; }
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::For(var_word, words, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), var_word.path);
                    assert_eq!(1, var_word.pos.line);
                    assert_eq!(5, var_word.pos.column);
                    assert_eq!(1, var_word.word_elems.len());
                    match &var_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("i"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(3, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(10, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("1"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("2"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[2].path);
                    assert_eq!(1, words[2].pos.line);
                    assert_eq!(14, words[2].pos.column);
                    assert_eq!(1, words[2].word_elems.len());
                    match &words[2].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("3"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands3), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, logical_commands3.len());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].path);
                            assert_eq!(2, logical_commands3[0].pos.line);
                            assert_eq!(7, logical_commands3[0].pos.column);
                            assert_eq!(false, logical_commands3[0].is_in_background);
                            assert_eq!(true, logical_commands3[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].first_command.path);
                            assert_eq!(2, logical_commands3[0].first_command.pos.line);
                            assert_eq!(7, logical_commands3[0].first_command.pos.column);
                            assert_eq!(false, logical_commands3[0].first_command.is_negative);
                            assert_eq!(1, logical_commands3[0].first_command.commands.len());
                            match &(*logical_commands3[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(7, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(7, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(12, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands3[1].path);
                            assert_eq!(2, logical_commands3[1].pos.line);
                            assert_eq!(17, logical_commands3[1].pos.column);
                            assert_eq!(false, logical_commands3[1].is_in_background);
                            assert_eq!(true, logical_commands3[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[1].first_command.path);
                            assert_eq!(2, logical_commands3[1].first_command.pos.line);
                            assert_eq!(17, logical_commands3[1].first_command.pos.column);
                            assert_eq!(false, logical_commands3[1].first_command.is_negative);
                            assert_eq!(1, logical_commands3[1].first_command.commands.len());
                            match &(*logical_commands3[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(17, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(22, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_case_clause()
{
    let s = "
case abc in
    abc | def) echo abc;;
    (ghi | jkl) echo ghi;;
esac
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Case(word, pairs), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), word.path);
                    assert_eq!(1, word.pos.line);
                    assert_eq!(6, word.pos.column);
                    assert_eq!(1, word.word_elems.len());
                    match &word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, pairs.len());
                    assert_eq!(2, pairs[0].pattern_words.len());
                    assert_eq!(String::from("test.sh"), pairs[0].pattern_words[0].path);
                    assert_eq!(2, pairs[0].pattern_words[0].pos.line);
                    assert_eq!(5, pairs[0].pattern_words[0].pos.column);
                    assert_eq!(1, pairs[0].pattern_words[0].word_elems.len());
                    match &pairs[0].pattern_words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), pairs[0].pattern_words[1].path);
                    assert_eq!(2, pairs[0].pattern_words[1].pos.line);
                    assert_eq!(11, pairs[0].pattern_words[1].pos.column);
                    assert_eq!(1, pairs[0].pattern_words[1].word_elems.len());
                    match &pairs[0].pattern_words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[0].commands.len());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].path);
                    assert_eq!(2, pairs[0].commands[0].pos.line);
                    assert_eq!(16, pairs[0].commands[0].pos.column);
                    assert_eq!(false, pairs[0].commands[0].is_in_background);
                    assert_eq!(true, pairs[0].commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].first_command.path);
                    assert_eq!(2, pairs[0].commands[0].first_command.pos.line);
                    assert_eq!(16, pairs[0].commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[0].commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[0].commands[0].first_command.commands.len());
                    match &(*pairs[0].commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(16, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(16, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(21, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, pairs[1].pattern_words.len());
                    assert_eq!(String::from("test.sh"), pairs[1].pattern_words[0].path);
                    assert_eq!(3, pairs[1].pattern_words[0].pos.line);
                    assert_eq!(6, pairs[1].pattern_words[0].pos.column);
                    assert_eq!(1, pairs[1].pattern_words[0].word_elems.len());
                    match &pairs[1].pattern_words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("ghi"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), pairs[1].pattern_words[1].path);
                    assert_eq!(3, pairs[1].pattern_words[1].pos.line);
                    assert_eq!(12, pairs[1].pattern_words[1].pos.column);
                    assert_eq!(1, pairs[1].pattern_words[1].word_elems.len());
                    match &pairs[1].pattern_words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("jkl"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[1].commands.len());
                    assert_eq!(String::from("test.sh"), pairs[1].commands[0].path);
                    assert_eq!(3, pairs[1].commands[0].pos.line);
                    assert_eq!(17, pairs[1].commands[0].pos.column);
                    assert_eq!(false, pairs[1].commands[0].is_in_background);
                    assert_eq!(true, pairs[1].commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[1].commands[0].first_command.path);
                    assert_eq!(3, pairs[1].commands[0].first_command.pos.line);
                    assert_eq!(17, pairs[1].commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[1].commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[1].commands[0].first_command.commands.len());
                    match &(*pairs[1].commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(17, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(17, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(3, simple_command.words[1].pos.line);
                            assert_eq!(22, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_case_clause_with_nested_compound_command()
{
    let s = "
case abc in
    abc)
        { echo abc; echo def; }
        ;;
esac
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Case(word, pairs), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), word.path);
                    assert_eq!(1, word.pos.line);
                    assert_eq!(6, word.pos.column);
                    assert_eq!(1, word.word_elems.len());
                    match &word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs.len());
                    assert_eq!(1, pairs[0].pattern_words.len());
                    assert_eq!(String::from("test.sh"), pairs[0].pattern_words[0].path);
                    assert_eq!(2, pairs[0].pattern_words[0].pos.line);
                    assert_eq!(5, pairs[0].pattern_words[0].pos.column);
                    assert_eq!(1, pairs[0].pattern_words[0].word_elems.len());
                    match &pairs[0].pattern_words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[0].commands.len());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].path);
                    assert_eq!(3, pairs[0].commands[0].pos.line);
                    assert_eq!(9, pairs[0].commands[0].pos.column);
                    assert_eq!(false, pairs[0].commands[0].is_in_background);
                    assert_eq!(true, pairs[0].commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].first_command.path);
                    assert_eq!(3, pairs[0].commands[0].first_command.pos.line);
                    assert_eq!(9, pairs[0].commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[0].commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[0].commands[0].first_command.commands.len());
                    match &(*pairs[0].commands[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands2), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(9, pos.column);
                            assert_eq!(2, logical_commands2.len());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                            assert_eq!(3, logical_commands2[0].pos.line);
                            assert_eq!(11, logical_commands2[0].pos.column);
                            assert_eq!(false, logical_commands2[0].is_in_background);
                            assert_eq!(true, logical_commands2[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                            assert_eq!(3, logical_commands2[0].first_command.pos.line);
                            assert_eq!(11, logical_commands2[0].first_command.pos.column);
                            assert_eq!(false, logical_commands2[0].first_command.is_negative);
                            assert_eq!(1, logical_commands2[0].first_command.commands.len());
                            match &(*logical_commands2[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(3, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(3, simple_command.words[0].pos.line);
                                    assert_eq!(11, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(3, simple_command.words[1].pos.line);
                                    assert_eq!(16, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                            assert_eq!(3, logical_commands2[1].pos.line);
                            assert_eq!(21, logical_commands2[1].pos.column);
                            assert_eq!(false, logical_commands2[1].is_in_background);
                            assert_eq!(true, logical_commands2[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                            assert_eq!(3, logical_commands2[1].first_command.pos.line);
                            assert_eq!(21, logical_commands2[1].first_command.pos.column);
                            assert_eq!(false, logical_commands2[1].first_command.is_negative);
                            assert_eq!(1, logical_commands2[1].first_command.commands.len());
                            match &(*logical_commands2[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(3, pos.line);
                                    assert_eq!(21, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(3, simple_command.words[0].pos.line);
                                    assert_eq!(21, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(3, simple_command.words[1].pos.line);
                                    assert_eq!(26, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }                    
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_if_clause()
{
    let s = "
if
    echo abc
    true; then
    echo def
    echo ghi
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::If(cond_logical_commands2, logical_commands21, pairs, None), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands21.len());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].path);
                    assert_eq!(4, logical_commands21[0].pos.line);
                    assert_eq!(5, logical_commands21[0].pos.column);
                    assert_eq!(false, logical_commands21[0].is_in_background);
                    assert_eq!(true, logical_commands21[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].first_command.path);
                    assert_eq!(4, logical_commands21[0].first_command.pos.line);
                    assert_eq!(5, logical_commands21[0].first_command.pos.column);
                    assert_eq!(false, logical_commands21[0].first_command.is_negative);
                    assert_eq!(1, logical_commands21[0].first_command.commands.len());
                    match &(*logical_commands21[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands21[1].path);
                    assert_eq!(5, logical_commands21[1].pos.line);
                    assert_eq!(5, logical_commands21[1].pos.column);
                    assert_eq!(false, logical_commands21[1].is_in_background);
                    assert_eq!(true, logical_commands21[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[1].first_command.path);
                    assert_eq!(5, logical_commands21[1].first_command.pos.line);
                    assert_eq!(5, logical_commands21[1].first_command.pos.column);
                    assert_eq!(false, logical_commands21[1].first_command.is_negative);
                    assert_eq!(1, logical_commands21[1].first_command.commands.len());
                    match &(*logical_commands21[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, pairs.is_empty());
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_if_clause_with_newline()
{
    let s = "
if
    echo abc
    true
then
    echo def
    echo ghi
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::If(cond_logical_commands2, logical_commands21, pairs, None), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands21.len());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].path);
                    assert_eq!(5, logical_commands21[0].pos.line);
                    assert_eq!(5, logical_commands21[0].pos.column);
                    assert_eq!(false, logical_commands21[0].is_in_background);
                    assert_eq!(true, logical_commands21[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].first_command.path);
                    assert_eq!(5, logical_commands21[0].first_command.pos.line);
                    assert_eq!(5, logical_commands21[0].first_command.pos.column);
                    assert_eq!(false, logical_commands21[0].first_command.is_negative);
                    assert_eq!(1, logical_commands21[0].first_command.commands.len());
                    match &(*logical_commands21[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands21[1].path);
                    assert_eq!(6, logical_commands21[1].pos.line);
                    assert_eq!(5, logical_commands21[1].pos.column);
                    assert_eq!(false, logical_commands21[1].is_in_background);
                    assert_eq!(true, logical_commands21[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[1].first_command.path);
                    assert_eq!(6, logical_commands21[1].first_command.pos.line);
                    assert_eq!(5, logical_commands21[1].first_command.pos.column);
                    assert_eq!(false, logical_commands21[1].first_command.is_negative);
                    assert_eq!(1, logical_commands21[1].first_command.commands.len());
                    match &(*logical_commands21[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, pairs.is_empty());
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_if_clause_with_elif_keywords()
{
    let s = "
if true; then
    echo abc
elif false; then
    echo def
elif true; then
    echo ghi
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::If(cond_logical_commands2, logical_commands21, pairs, None), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(1, cond_logical_commands2[0].pos.line);
                    assert_eq!(4, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(1, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(4, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(4, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, logical_commands21.len());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].path);
                    assert_eq!(2, logical_commands21[0].pos.line);
                    assert_eq!(5, logical_commands21[0].pos.column);
                    assert_eq!(false, logical_commands21[0].is_in_background);
                    assert_eq!(true, logical_commands21[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].first_command.path);
                    assert_eq!(2, logical_commands21[0].first_command.pos.line);
                    assert_eq!(5, logical_commands21[0].first_command.pos.column);
                    assert_eq!(false, logical_commands21[0].first_command.is_negative);
                    assert_eq!(1, logical_commands21[0].first_command.commands.len());
                    match &(*logical_commands21[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, pairs.len());
                    assert_eq!(1, pairs[0].cond_commands.len());
                    assert_eq!(String::from("test.sh"), pairs[0].cond_commands[0].path);
                    assert_eq!(3, pairs[0].cond_commands[0].pos.line);
                    assert_eq!(6, pairs[0].cond_commands[0].pos.column);
                    assert_eq!(false, pairs[0].cond_commands[0].is_in_background);
                    assert_eq!(true, pairs[0].cond_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[0].cond_commands[0].first_command.path);
                    assert_eq!(3, pairs[0].cond_commands[0].first_command.pos.line);
                    assert_eq!(6, pairs[0].cond_commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[0].cond_commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[0].cond_commands[0].first_command.commands.len());
                    match &(*pairs[0].cond_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(6, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(6, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("false"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[0].commands.len());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].path);
                    assert_eq!(4, pairs[0].commands[0].pos.line);
                    assert_eq!(5, pairs[0].commands[0].pos.column);
                    assert_eq!(false, pairs[0].commands[0].is_in_background);
                    assert_eq!(true, pairs[0].commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].first_command.path);
                    assert_eq!(4, pairs[0].commands[0].first_command.pos.line);
                    assert_eq!(5, pairs[0].commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[0].commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[0].commands[0].first_command.commands.len());
                    match &(*pairs[0].commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[1].cond_commands.len());
                    assert_eq!(String::from("test.sh"), pairs[1].cond_commands[0].path);
                    assert_eq!(5, pairs[1].cond_commands[0].pos.line);
                    assert_eq!(6, pairs[1].cond_commands[0].pos.column);
                    assert_eq!(false, pairs[1].cond_commands[0].is_in_background);
                    assert_eq!(true, pairs[1].cond_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[1].cond_commands[0].first_command.path);
                    assert_eq!(5, pairs[1].cond_commands[0].first_command.pos.line);
                    assert_eq!(6, pairs[1].cond_commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[1].cond_commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[1].cond_commands[0].first_command.commands.len());
                    match &(*pairs[1].cond_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(6, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(6, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, pairs[1].commands.len());
                    assert_eq!(String::from("test.sh"), pairs[1].commands[0].path);
                    assert_eq!(6, pairs[1].commands[0].pos.line);
                    assert_eq!(5, pairs[1].commands[0].pos.column);
                    assert_eq!(false, pairs[1].commands[0].is_in_background);
                    assert_eq!(true, pairs[1].commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), pairs[0].commands[0].first_command.path);
                    assert_eq!(6, pairs[1].commands[0].first_command.pos.line);
                    assert_eq!(5, pairs[1].commands[0].first_command.pos.column);
                    assert_eq!(false, pairs[1].commands[0].first_command.is_negative);
                    assert_eq!(1, pairs[1].commands[0].first_command.commands.len());
                    match &(*pairs[1].commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(6, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_if_clause_with_else_keyword()
{
    let s = "
if
    echo abc
    true; then
    echo def
    echo ghi
else
    echo jkl
    echo mno
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::If(cond_logical_commands2, logical_commands21, pairs, Some(logical_commands22)), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands21.len());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].path);
                    assert_eq!(4, logical_commands21[0].pos.line);
                    assert_eq!(5, logical_commands21[0].pos.column);
                    assert_eq!(false, logical_commands21[0].is_in_background);
                    assert_eq!(true, logical_commands21[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].first_command.path);
                    assert_eq!(4, logical_commands21[0].first_command.pos.line);
                    assert_eq!(5, logical_commands21[0].first_command.pos.column);
                    assert_eq!(false, logical_commands21[0].first_command.is_negative);
                    assert_eq!(1, logical_commands21[0].first_command.commands.len());
                    match &(*logical_commands21[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands21[1].path);
                    assert_eq!(5, logical_commands21[1].pos.line);
                    assert_eq!(5, logical_commands21[1].pos.column);
                    assert_eq!(false, logical_commands21[1].is_in_background);
                    assert_eq!(true, logical_commands21[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[1].first_command.path);
                    assert_eq!(5, logical_commands21[1].first_command.pos.line);
                    assert_eq!(5, logical_commands21[1].first_command.pos.column);
                    assert_eq!(false, logical_commands21[1].first_command.is_negative);
                    assert_eq!(1, logical_commands21[1].first_command.commands.len());
                    match &(*logical_commands21[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, pairs.is_empty());
                    assert_eq!(2, logical_commands22.len());
                    assert_eq!(String::from("test.sh"), logical_commands22[0].path);
                    assert_eq!(7, logical_commands22[0].pos.line);
                    assert_eq!(5, logical_commands22[0].pos.column);
                    assert_eq!(false, logical_commands22[0].is_in_background);
                    assert_eq!(true, logical_commands22[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands22[0].first_command.path);
                    assert_eq!(7, logical_commands22[0].first_command.pos.line);
                    assert_eq!(5, logical_commands22[0].first_command.pos.column);
                    assert_eq!(false, logical_commands22[0].first_command.is_negative);
                    assert_eq!(1, logical_commands22[0].first_command.commands.len());
                    match &(*logical_commands22[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(7, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(7, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(7, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("jkl"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands22[1].path);
                    assert_eq!(8, logical_commands22[1].pos.line);
                    assert_eq!(5, logical_commands22[1].pos.column);
                    assert_eq!(false, logical_commands22[1].is_in_background);
                    assert_eq!(true, logical_commands22[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands22[1].first_command.path);
                    assert_eq!(8, logical_commands22[1].first_command.pos.line);
                    assert_eq!(5, logical_commands22[1].first_command.pos.column);
                    assert_eq!(false, logical_commands22[1].first_command.is_negative);
                    assert_eq!(1, logical_commands22[1].first_command.commands.len());
                    match &(*logical_commands22[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(8, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(8, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(8, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("mno"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }                    
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_if_clause_with_nested_compound_commands()
{
    let s = "
if true; then
    { echo abc; echo def; }
else
    { echo ghi; echo jkl; }
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::If(cond_logical_commands2, logical_commands21, pairs, Some(logical_commands22)), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(1, cond_logical_commands2[0].pos.line);
                    assert_eq!(4, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(1, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(4, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(4, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, logical_commands21.len());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].path);
                    assert_eq!(2, logical_commands21[0].pos.line);
                    assert_eq!(5, logical_commands21[0].pos.column);
                    assert_eq!(false, logical_commands21[0].is_in_background);
                    assert_eq!(true, logical_commands21[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands21[0].first_command.path);
                    assert_eq!(2, logical_commands21[0].first_command.pos.line);
                    assert_eq!(5, logical_commands21[0].first_command.pos.column);
                    assert_eq!(false, logical_commands21[0].first_command.is_negative);
                    assert_eq!(1, logical_commands21[0].first_command.commands.len());
                    match &(*logical_commands21[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands3), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, logical_commands3.len());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].path);
                            assert_eq!(2, logical_commands3[0].pos.line);
                            assert_eq!(7, logical_commands3[0].pos.column);
                            assert_eq!(false, logical_commands3[0].is_in_background);
                            assert_eq!(true, logical_commands3[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].first_command.path);
                            assert_eq!(2, logical_commands3[0].first_command.pos.line);
                            assert_eq!(7, logical_commands3[0].first_command.pos.column);
                            assert_eq!(false, logical_commands3[0].first_command.is_negative);
                            assert_eq!(1, logical_commands3[0].first_command.commands.len());
                            match &(*logical_commands3[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(7, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(7, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(12, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands3[1].path);
                            assert_eq!(2, logical_commands3[1].pos.line);
                            assert_eq!(17, logical_commands3[1].pos.column);
                            assert_eq!(false, logical_commands3[1].is_in_background);
                            assert_eq!(true, logical_commands3[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[1].first_command.path);
                            assert_eq!(2, logical_commands3[1].first_command.pos.line);
                            assert_eq!(17, logical_commands3[1].first_command.pos.column);
                            assert_eq!(false, logical_commands3[1].first_command.is_negative);
                            assert_eq!(1, logical_commands3[1].first_command.commands.len());
                            match &(*logical_commands3[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(17, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(22, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, pairs.is_empty());
                    assert_eq!(1, logical_commands22.len());
                    assert_eq!(String::from("test.sh"), logical_commands22[0].path);
                    assert_eq!(4, logical_commands22[0].pos.line);
                    assert_eq!(5, logical_commands22[0].pos.column);
                    assert_eq!(false, logical_commands22[0].is_in_background);
                    assert_eq!(true, logical_commands22[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands22[0].first_command.path);
                    assert_eq!(4, logical_commands22[0].first_command.pos.line);
                    assert_eq!(5, logical_commands22[0].first_command.pos.column);
                    assert_eq!(false, logical_commands22[0].first_command.is_negative);
                    assert_eq!(1, logical_commands22[0].first_command.commands.len());
                    match &(*logical_commands22[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands3), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, logical_commands3.len());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].path);
                            assert_eq!(4, logical_commands3[0].pos.line);
                            assert_eq!(7, logical_commands3[0].pos.column);
                            assert_eq!(false, logical_commands3[0].is_in_background);
                            assert_eq!(true, logical_commands3[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].first_command.path);
                            assert_eq!(4, logical_commands3[0].first_command.pos.line);
                            assert_eq!(7, logical_commands3[0].first_command.pos.column);
                            assert_eq!(false, logical_commands3[0].first_command.is_negative);
                            assert_eq!(1, logical_commands3[0].first_command.commands.len());
                            match &(*logical_commands3[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(4, pos.line);
                                    assert_eq!(7, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(4, simple_command.words[0].pos.line);
                                    assert_eq!(7, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(4, simple_command.words[1].pos.line);
                                    assert_eq!(12, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("ghi"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands3[1].path);
                            assert_eq!(4, logical_commands3[1].pos.line);
                            assert_eq!(17, logical_commands3[1].pos.column);
                            assert_eq!(false, logical_commands3[1].is_in_background);
                            assert_eq!(true, logical_commands3[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[1].first_command.path);
                            assert_eq!(4, logical_commands3[1].first_command.pos.line);
                            assert_eq!(17, logical_commands3[1].first_command.pos.column);
                            assert_eq!(false, logical_commands3[1].first_command.is_negative);
                            assert_eq!(1, logical_commands3[1].first_command.commands.len());
                            match &(*logical_commands3[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(4, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(4, simple_command.words[0].pos.line);
                                    assert_eq!(17, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(4, simple_command.words[1].pos.line);
                                    assert_eq!(22, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("jkl"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_while_clause()
{
    let s = "
while
    echo abc
    true; do
    echo def
    echo ghi
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::While(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(4, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(4, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(5, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(5, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_while_clause_with_newline()
{
    let s = "
while
    echo abc
    true
do
    echo def
    echo ghi
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::While(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(5, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(5, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(6, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(6, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_while_clause_with_nested_compound_command()
{
    let s = "
while true; do
    { echo abc; echo def; }
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::While(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(1, cond_logical_commands2[0].pos.line);
                    assert_eq!(7, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(1, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(7, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(7, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(7, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("true"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands3), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, logical_commands3.len());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].path);
                            assert_eq!(2, logical_commands3[0].pos.line);
                            assert_eq!(7, logical_commands3[0].pos.column);
                            assert_eq!(false, logical_commands3[0].is_in_background);
                            assert_eq!(true, logical_commands3[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].first_command.path);
                            assert_eq!(2, logical_commands3[0].first_command.pos.line);
                            assert_eq!(7, logical_commands3[0].first_command.pos.column);
                            assert_eq!(false, logical_commands3[0].first_command.is_negative);
                            assert_eq!(1, logical_commands3[0].first_command.commands.len());
                            match &(*logical_commands3[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(7, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(7, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(12, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands3[1].path);
                            assert_eq!(2, logical_commands3[1].pos.line);
                            assert_eq!(17, logical_commands3[1].pos.column);
                            assert_eq!(false, logical_commands3[1].is_in_background);
                            assert_eq!(true, logical_commands3[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[1].first_command.path);
                            assert_eq!(2, logical_commands3[1].first_command.pos.line);
                            assert_eq!(17, logical_commands3[1].first_command.pos.column);
                            assert_eq!(false, logical_commands3[1].first_command.is_negative);
                            assert_eq!(1, logical_commands3[1].first_command.commands.len());
                            match &(*logical_commands3[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(17, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(22, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_until_clause()
{
    let s = "
until
    echo abc
    false; do
    echo def
    echo ghi
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Until(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("false"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(4, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(4, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(4, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(4, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(5, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(5, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_until_clause_with_newline()
{
    let s = "
until
    echo abc
    false
do
    echo def
    echo ghi
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Until(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(2, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(2, cond_logical_commands2[0].pos.line);
                    assert_eq!(5, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(2, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(2, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(2, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("abc"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].path);
                    assert_eq!(3, cond_logical_commands2[1].pos.line);
                    assert_eq!(5, cond_logical_commands2[1].pos.column);
                    assert_eq!(false, cond_logical_commands2[1].is_in_background);
                    assert_eq!(true, cond_logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[1].first_command.path);
                    assert_eq!(3, cond_logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, cond_logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[1].first_command.commands.len());
                    match &(*cond_logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(3, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("false"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(5, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(5, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(5, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                            assert_eq!(5, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("def"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                    assert_eq!(6, logical_commands2[1].pos.line);
                    assert_eq!(5, logical_commands2[1].pos.column);
                    assert_eq!(false, logical_commands2[1].is_in_background);
                    assert_eq!(true, logical_commands2[1].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                    assert_eq!(6, logical_commands2[1].first_command.pos.line);
                    assert_eq!(5, logical_commands2[1].first_command.pos.column);
                    assert_eq!(false, logical_commands2[1].first_command.is_negative);
                    assert_eq!(1, logical_commands2[1].first_command.commands.len());
                    match &(*logical_commands2[1].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[0].pos.line);
                            assert_eq!(5, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("echo"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(6, simple_command.words[1].pos.line);
                            assert_eq!(10, simple_command.words[1].pos.column);
                            assert_eq!(1, simple_command.words[1].word_elems.len());
                            match &simple_command.words[1].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("ghi"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_until_clause_with_nested_compound_command()
{
    let s = "
until false; do
    { echo abc; echo def; }
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::Compound(path, pos, CompoundCommand::Until(cond_logical_commands2, logical_commands2), redirects) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(1, cond_logical_commands2.len());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].path);
                    assert_eq!(1, cond_logical_commands2[0].pos.line);
                    assert_eq!(7, cond_logical_commands2[0].pos.column);
                    assert_eq!(false, cond_logical_commands2[0].is_in_background);
                    assert_eq!(true, cond_logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), cond_logical_commands2[0].first_command.path);
                    assert_eq!(1, cond_logical_commands2[0].first_command.pos.line);
                    assert_eq!(7, cond_logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, cond_logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, cond_logical_commands2[0].first_command.commands.len());
                    match &(*cond_logical_commands2[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(7, pos.column);
                            assert_eq!(1, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(7, simple_command.words[0].pos.column);
                            assert_eq!(1, simple_command.words[0].word_elems.len());
                            match &simple_command.words[0].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("false"), s);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, simple_command.redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(1, logical_commands2.len());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                    assert_eq!(2, logical_commands2[0].pos.line);
                    assert_eq!(5, logical_commands2[0].pos.column);
                    assert_eq!(false, logical_commands2[0].is_in_background);
                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                    assert_eq!(2, logical_commands2[0].first_command.pos.line);
                    assert_eq!(5, logical_commands2[0].first_command.pos.column);
                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                    match &(*logical_commands2[0].first_command.commands[0]) {
                        Command::Compound(path, pos, CompoundCommand::BraceGroup(logical_commands3), redirects) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(2, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(2, logical_commands3.len());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].path);
                            assert_eq!(2, logical_commands3[0].pos.line);
                            assert_eq!(7, logical_commands3[0].pos.column);
                            assert_eq!(false, logical_commands3[0].is_in_background);
                            assert_eq!(true, logical_commands3[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[0].first_command.path);
                            assert_eq!(2, logical_commands3[0].first_command.pos.line);
                            assert_eq!(7, logical_commands3[0].first_command.pos.column);
                            assert_eq!(false, logical_commands3[0].first_command.is_negative);
                            assert_eq!(1, logical_commands3[0].first_command.commands.len());
                            match &(*logical_commands3[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(7, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(7, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(12, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands3[1].path);
                            assert_eq!(2, logical_commands3[1].pos.line);
                            assert_eq!(17, logical_commands3[1].pos.column);
                            assert_eq!(false, logical_commands3[1].is_in_background);
                            assert_eq!(true, logical_commands3[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands3[1].first_command.path);
                            assert_eq!(2, logical_commands3[1].first_command.pos.line);
                            assert_eq!(17, logical_commands3[1].first_command.pos.column);
                            assert_eq!(false, logical_commands3[1].first_command.is_negative);
                            assert_eq!(1, logical_commands3[1].first_command.commands.len());
                            match &(*logical_commands3[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(17, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(22, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, redirects.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_function_definition()
{
    let s = "
fun() {
    echo abc
    echo def
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::FunctionDefinition(path, pos, name_word, fun_body) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), name_word.path);
                    assert_eq!(1, name_word.pos.line);
                    assert_eq!(1, name_word.pos.column);
                    assert_eq!(1, name_word.word_elems.len());
                    match &name_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("fun"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), fun_body.path);
                    assert_eq!(1, fun_body.pos.line);
                    assert_eq!(7, fun_body.pos.column);
                    match &fun_body.command {
                        CompoundCommand::BraceGroup(logical_commands2) => {
                            assert_eq!(2, logical_commands2.len());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                            assert_eq!(2, logical_commands2[0].pos.line);
                            assert_eq!(5, logical_commands2[0].pos.column);
                            assert_eq!(false, logical_commands2[0].is_in_background);
                            assert_eq!(true, logical_commands2[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                            assert_eq!(2, logical_commands2[0].first_command.pos.line);
                            assert_eq!(5, logical_commands2[0].first_command.pos.column);
                            assert_eq!(false, logical_commands2[0].first_command.is_negative);
                            assert_eq!(1, logical_commands2[0].first_command.commands.len());
                            match &(*logical_commands2[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                            assert_eq!(3, logical_commands2[1].pos.line);
                            assert_eq!(5, logical_commands2[1].pos.column);
                            assert_eq!(false, logical_commands2[1].is_in_background);
                            assert_eq!(true, logical_commands2[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                            assert_eq!(3, logical_commands2[1].first_command.pos.line);
                            assert_eq!(5, logical_commands2[1].first_command.pos.column);
                            assert_eq!(false, logical_commands2[1].first_command.is_negative);
                            assert_eq!(1, logical_commands2[1].first_command.commands.len());
                            match &(*logical_commands2[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(3, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(3, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(3, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, fun_body.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_function_definition_with_newline()
{
    let s = "
fun()
{
    echo abc
    echo def
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::FunctionDefinition(path, pos, name_word, fun_body) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), name_word.path);
                    assert_eq!(1, name_word.pos.line);
                    assert_eq!(1, name_word.pos.column);
                    assert_eq!(1, name_word.word_elems.len());
                    match &name_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("fun"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), fun_body.path);
                    assert_eq!(2, fun_body.pos.line);
                    assert_eq!(1, fun_body.pos.column);
                    match &fun_body.command {
                        CompoundCommand::BraceGroup(logical_commands2) => {
                            assert_eq!(2, logical_commands2.len());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                            assert_eq!(3, logical_commands2[0].pos.line);
                            assert_eq!(5, logical_commands2[0].pos.column);
                            assert_eq!(false, logical_commands2[0].is_in_background);
                            assert_eq!(true, logical_commands2[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                            assert_eq!(3, logical_commands2[0].first_command.pos.line);
                            assert_eq!(5, logical_commands2[0].first_command.pos.column);
                            assert_eq!(false, logical_commands2[0].first_command.is_negative);
                            assert_eq!(1, logical_commands2[0].first_command.commands.len());
                            match &(*logical_commands2[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(3, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(3, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(3, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                            assert_eq!(4, logical_commands2[1].pos.line);
                            assert_eq!(5, logical_commands2[1].pos.column);
                            assert_eq!(false, logical_commands2[1].is_in_background);
                            assert_eq!(true, logical_commands2[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                            assert_eq!(4, logical_commands2[1].first_command.pos.line);
                            assert_eq!(5, logical_commands2[1].first_command.pos.column);
                            assert_eq!(false, logical_commands2[1].first_command.is_negative);
                            assert_eq!(1, logical_commands2[1].first_command.commands.len());
                            match &(*logical_commands2[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(4, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(4, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(4, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, fun_body.redirects.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_logical_commands_parses_function_definition_with_redirections()
{
    let s = "
fun() {
    echo abc
    echo def
} > xxx 2> yyy
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Ok(logical_commands) => {
            assert_eq!(1, logical_commands.len());
            assert_eq!(String::from("test.sh"), logical_commands[0].path);
            assert_eq!(1, logical_commands[0].pos.line);
            assert_eq!(1, logical_commands[0].pos.column);
            assert_eq!(false, logical_commands[0].is_in_background);
            assert_eq!(true, logical_commands[0].pairs.is_empty());
            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
            assert_eq!(1, logical_commands[0].first_command.pos.line);
            assert_eq!(1, logical_commands[0].first_command.pos.column);
            assert_eq!(false, logical_commands[0].first_command.is_negative);
            assert_eq!(1, logical_commands[0].first_command.commands.len());
            match &(*logical_commands[0].first_command.commands[0]) {
                Command::FunctionDefinition(path, pos, name_word, fun_body) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("test.sh"), name_word.path);
                    assert_eq!(1, name_word.pos.line);
                    assert_eq!(1, name_word.pos.column);
                    assert_eq!(1, name_word.word_elems.len());
                    match &name_word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("fun"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), fun_body.path);
                    assert_eq!(1, fun_body.pos.line);
                    assert_eq!(7, fun_body.pos.column);
                    match &fun_body.command {
                        CompoundCommand::BraceGroup(logical_commands2) => {
                            assert_eq!(2, logical_commands2.len());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                            assert_eq!(2, logical_commands2[0].pos.line);
                            assert_eq!(5, logical_commands2[0].pos.column);
                            assert_eq!(false, logical_commands2[0].is_in_background);
                            assert_eq!(true, logical_commands2[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                            assert_eq!(2, logical_commands2[0].first_command.pos.line);
                            assert_eq!(5, logical_commands2[0].first_command.pos.column);
                            assert_eq!(false, logical_commands2[0].first_command.is_negative);
                            assert_eq!(1, logical_commands2[0].first_command.commands.len());
                            match &(*logical_commands2[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(2, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(2, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(2, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("test.sh"), logical_commands2[1].path);
                            assert_eq!(3, logical_commands2[1].pos.line);
                            assert_eq!(5, logical_commands2[1].pos.column);
                            assert_eq!(false, logical_commands2[1].is_in_background);
                            assert_eq!(true, logical_commands2[1].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands2[1].first_command.path);
                            assert_eq!(3, logical_commands2[1].first_command.pos.line);
                            assert_eq!(5, logical_commands2[1].first_command.pos.column);
                            assert_eq!(false, logical_commands2[1].first_command.is_negative);
                            assert_eq!(1, logical_commands2[1].first_command.commands.len());
                            match &(*logical_commands2[1].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(3, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(3, simple_command.words[0].pos.line);
                                    assert_eq!(5, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(3, simple_command.words[1].pos.line);
                                    assert_eq!(10, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("def"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, simple_command.redirects.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(2, fun_body.redirects.len());
                    match &(*fun_body.redirects[0]) {
                        Redirection::Output(path2, pos2, None, word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(4, pos2.line);
                            assert_eq!(3, pos2.column);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(4, word.pos.line);
                            assert_eq!(5, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("xxx"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    match &(*fun_body.redirects[1]) {
                        Redirection::Output(path2, pos2, Some(n), word, is_bar) => {
                            assert_eq!(&String::from("test.sh"), path2);
                            assert_eq!(4, pos2.line);
                            assert_eq!(9, pos2.column);
                            assert_eq!(2, *n);
                            assert_eq!(false, *is_bar);
                            assert_eq!(String::from("test.sh"), word.path);
                            assert_eq!(4, word.pos.line);
                            assert_eq!(12, word.pos.column);
                            assert_eq!(1, word.word_elems.len());
                            match &word.word_elems[0] {
                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                    assert_eq!(&String::from("yyy"), s);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_command()
{
    let s = "echo xxx;;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_for_line_parses_complains_on_unexpected_token_for_command()
{
    let s = "echo xxx;;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands_for_line(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_logical_operator()
{
    let s = "echo xxx && ;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_pipe_operator()
{
    let s = "echo xxx | !";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(12, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_brace_group()
{
    let s = "
{
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_brace_group_and_eof()
{
    let s = "
{
    echo abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(2, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_subshell()
{
    let s = "
(
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_subshell_and_eof()
{
    let s = "
(
    echo abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(2, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_for_clause()
{
    let s = "
for i in 1 2 3; do
    echo abc
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_for_clause_and_eof()
{
    let s = "
for i in 1 2 3; do
    echo abc
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_for_clause_without_in_keyword()
{
    let s = "
for i 1 2 3
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(7, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_for_clause_without_in_keyword_and_eof()
{
    let s = "
for i";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(6, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_case_clause()
{
    let s = "
case abc in
    abc) echo xxx;;
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_case_clause_and_eof()
{
    let s = "
case abc in
    abc) echo xxx;;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_case_clause_without_in_keyword()
{
    let s = "
case abc do";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(10, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_case_clause_without_in_keyword_and_eof()
{
    let s = "
case abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_if_clause()
{
    let s = "
if true; then
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_if_clause_and_eof()
{
    let s = "
if true; then
    echo abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(2, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_while_clause()
{
    let s = "
while true; do
    echo abc
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_while_clause_and_eof()
{
    let s = "
while true; do
    echo abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(2, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_until_clause()
{
    let s = "
until true; do
    echo abc
fi
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_until_clause_and_eof()
{
    let s = "
until true; do
    echo abc";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(2, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_function_definition()
{
    let s = "
fun() do
    echo abc
done
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(7, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_logical_commands_parses_complains_on_unexpected_token_for_function_definition_and_eof()
{
    let s = "
fun()";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_logical_commands(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(6, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(true, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_alias_command_parses_command()
{
    let s = "echo abc def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_alias_command(&mut lexer, &settings) {
        Ok(alias_command) => {
            assert_eq!(String::from("test.sh"), alias_command.path);
            assert_eq!(1, alias_command.pos.line);
            assert_eq!(1, alias_command.pos.column);
            assert_eq!(3, alias_command.command.words.len());
            assert_eq!(String::from("test.sh"), alias_command.command.words[0].path);
            assert_eq!(1, alias_command.command.words[0].pos.line);
            assert_eq!(1, alias_command.command.words[0].pos.column);
            assert_eq!(1, alias_command.command.words[0].word_elems.len());
            match &alias_command.command.words[0].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("echo"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), alias_command.command.words[1].path);
            assert_eq!(1, alias_command.command.words[1].pos.line);
            assert_eq!(6, alias_command.command.words[1].pos.column);
            assert_eq!(1, alias_command.command.words[1].word_elems.len());
            match &alias_command.command.words[1].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), alias_command.command.words[2].path);
            assert_eq!(1, alias_command.command.words[2].pos.line);
            assert_eq!(10, alias_command.command.words[2].pos.column);
            assert_eq!(1, alias_command.command.words[2].word_elems.len());
            match &alias_command.command.words[2].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("def"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(true, alias_command.command.redirects.is_empty());
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_alias_command_parses_command_with_redictions()
{
    let s = "echo abc > xxx 2>> yyy";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_alias_command(&mut lexer, &settings) {
        Ok(alias_command) => {
            assert_eq!(String::from("test.sh"), alias_command.path);
            assert_eq!(1, alias_command.pos.line);
            assert_eq!(1, alias_command.pos.column);
            assert_eq!(2, alias_command.command.words.len());
            assert_eq!(String::from("test.sh"), alias_command.command.words[0].path);
            assert_eq!(1, alias_command.command.words[0].pos.line);
            assert_eq!(1, alias_command.command.words[0].pos.column);
            assert_eq!(1, alias_command.command.words[0].word_elems.len());
            match &alias_command.command.words[0].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("echo"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("test.sh"), alias_command.command.words[1].path);
            assert_eq!(1, alias_command.command.words[1].pos.line);
            assert_eq!(6, alias_command.command.words[1].pos.column);
            assert_eq!(1, alias_command.command.words[1].word_elems.len());
            match &alias_command.command.words[1].word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(2, alias_command.command.redirects.len());
            match &(*alias_command.command.redirects[0]) {
                Redirection::Output(path, pos, None, word, is_bar) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(false, *is_bar);
                    assert_eq!(String::from("test.sh"), word.path);
                    assert_eq!(1, word.pos.line);
                    assert_eq!(12, word.pos.column);
                    assert_eq!(1, word.word_elems.len());
                    match &word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("xxx"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*alias_command.command.redirects[1]) {
                Redirection::Appending(path, pos, Some(n), word) => {
                    assert_eq!(&String::from("test.sh"), path);
                    assert_eq!(1, pos.line);
                    assert_eq!(16, pos.column);
                    assert_eq!(2, *n);
                    assert_eq!(String::from("test.sh"), word.path);
                    assert_eq!(1, word.pos.line);
                    assert_eq!(20, word.pos.column);
                    assert_eq!(1, word.word_elems.len());
                    match &word.word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("yyy"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_alias_command_parses_complains_on_unexpected_token_for_command()
{
    let s = "echo xxx;;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_alias_command(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_arith_expr_parses_expression()
{
    let s = "1 + 2))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Add, op);
            match &(*expr1) {
                ArithmeticExpression::Number(path1, pos1, n1) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(1, *n1);
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(5, pos2.column);
                    assert_eq!(2, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression_with_nested_expressions()
{
    let s = "1 * 2 + 4 / 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Add, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::Multiply, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Binary(path2, pos2, expr5, op2, expr6) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(BinaryOperator::Divide, *op2);
                    match &(**expr5) {
                        ArithmeticExpression::Number(path5, pos5, n5) => {
                            assert_eq!(&String::from("test.sh"), path5);
                            assert_eq!(1, pos5.line);
                            assert_eq!(9, pos5.column);
                            assert_eq!(4, *n5);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr6) {
                        ArithmeticExpression::Number(path6, pos6, n6) => {
                            assert_eq!(&String::from("test.sh"), path6);
                            assert_eq!(1, pos6.line);
                            assert_eq!(13, pos6.column);
                            assert_eq!(3, *n6);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression_with_nested_expressions_which_are_parentheses()
{
    let s = "(1 + 2) * (4 - 3)))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Multiply, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(2, pos1.column);
                    assert_eq!(BinaryOperator::Add, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(2, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(6, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Binary(path2, pos2, expr5, op2, expr6) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(12, pos2.column);
                    assert_eq!(BinaryOperator::Substract, *op2);
                    match &(**expr5) {
                        ArithmeticExpression::Number(path5, pos5, n5) => {
                            assert_eq!(&String::from("test.sh"), path5);
                            assert_eq!(1, pos5.line);
                            assert_eq!(12, pos5.column);
                            assert_eq!(4, *n5);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr6) {
                        ArithmeticExpression::Number(path6, pos6, n6) => {
                            assert_eq!(&String::from("test.sh"), path6);
                            assert_eq!(1, pos6.line);
                            assert_eq!(16, pos6.column);
                            assert_eq!(3, *n6);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_number()
{
    let s = "123))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Number(path, pos, n)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(123, n);
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_variable()
{
    let s = "x))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Parameter(path, pos, ParameterName::Variable(var_name))) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("x"), var_name);
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression12()
{
    let s = "~-1))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Unary(path, pos, op, expr1)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(UnaryOperator::Not, op);
            match &(*expr1) {
                ArithmeticExpression::Unary(path1, pos1, op1, expr2) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(2, pos1.column);
                    assert_eq!(UnaryOperator::Negate, *op1);
                    match &(**expr2) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(3, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression12_with_plus()
{
    let s = "+1))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Number(path, pos, n)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
            assert_eq!(1, n);
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression11()
{
    let s = "1 * 2 / 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Divide, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::Multiply, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression10()
{
    let s = "1 + 2 - 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Substract, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::Add, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression9()
{
    let s = "1 << 2 >> 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::ShiftRight, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::ShiftLeft, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(6, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(11, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression8()
{
    let s = "1 < 2 > 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::GreaterThan, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::LessThan, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression7()
{
    let s = "1 == 2 != 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::NotEqual, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::Equal, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(6, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(11, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression6()
{
    let s = "1 & 2 & 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::And, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::And, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression5()
{
    let s = "1 ^ 2 ^ 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::ExclusiveOr, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::ExclusiveOr, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression4()
{
    let s = "1 | 2 | 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Or, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::Or, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(9, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression3()
{
    let s = "1 && 2 && 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::LogicalAnd, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::LogicalAnd, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(6, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(11, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression2()
{
    let s = "1 || 2 || 3))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::LogicalOr, op);
            match &(*expr1) {
                ArithmeticExpression::Binary(path1, pos1, expr3, op1, expr4) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(BinaryOperator::LogicalOr, *op1);
                    match &(**expr3) {
                        ArithmeticExpression::Number(path3, pos3, n3) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                            assert_eq!(1, *n3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(6, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Number(path2, pos2, n2) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(11, pos2.column);
                    assert_eq!(3, *n2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression1()
{
    let s = "x = y *= 2))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Binary(path, pos, expr1, op, expr2)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(BinaryOperator::Assign, op);
            match &(*expr1) {
                ArithmeticExpression::Parameter(path1, pos1, ParameterName::Variable(var_name1)) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(&String::from("x"), var_name1);
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Binary(path2, pos2, expr3, op2, expr4) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(5, pos2.column);
                    assert_eq!(BinaryOperator::MultiplyAssign, *op2);
                    match &(**expr3) {
                        ArithmeticExpression::Parameter(path3, pos3, ParameterName::Variable(var_name3)) => {
                            assert_eq!(&String::from("test.sh"), path3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(5, pos3.column);
                            assert_eq!(&String::from("y"), var_name3);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(10, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_parses_expression1_with_conditionals()
{
    let s = "1 ? 2 ? 3 : 4 : 5 ? 6 : 7))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Ok(ArithmeticExpression::Conditional(path, pos, expr1, expr2, expr3)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            match &(*expr1) {
                ArithmeticExpression::Number(path1, pos1, n1) => {
                    assert_eq!(&String::from("test.sh"), path1);
                    assert_eq!(1, pos1.line);
                    assert_eq!(1, pos1.column);
                    assert_eq!(1, *n1);
                },
                _ => assert!(false),
            }
            match &(*expr2) {
                ArithmeticExpression::Conditional(path2, pos2, expr4, expr5, expr6) => {
                    assert_eq!(&String::from("test.sh"), path2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(5, pos2.column);
                    match &(**expr4) {
                        ArithmeticExpression::Number(path4, pos4, n4) => {
                            assert_eq!(&String::from("test.sh"), path4);
                            assert_eq!(1, pos4.line);
                            assert_eq!(5, pos4.column);
                            assert_eq!(2, *n4);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr5) {
                        ArithmeticExpression::Number(path5, pos5, n5) => {
                            assert_eq!(&String::from("test.sh"), path5);
                            assert_eq!(1, pos5.line);
                            assert_eq!(9, pos5.column);
                            assert_eq!(3, *n5);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr6) {
                        ArithmeticExpression::Number(path6, pos6, n6) => {
                            assert_eq!(&String::from("test.sh"), path6);
                            assert_eq!(1, pos6.line);
                            assert_eq!(13, pos6.column);
                            assert_eq!(4, *n6);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &(*expr3) {
                ArithmeticExpression::Conditional(path3, pos3, expr7, expr8, expr9) => {
                    assert_eq!(&String::from("test.sh"), path3);
                    assert_eq!(1, pos3.line);
                    assert_eq!(17, pos3.column);
                    match &(**expr7) {
                        ArithmeticExpression::Number(path7, pos7, n7) => {
                            assert_eq!(&String::from("test.sh"), path7);
                            assert_eq!(1, pos7.line);
                            assert_eq!(17, pos7.column);
                            assert_eq!(5, *n7);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr8) {
                        ArithmeticExpression::Number(path8, pos8, n8) => {
                            assert_eq!(&String::from("test.sh"), path8);
                            assert_eq!(1, pos8.line);
                            assert_eq!(21, pos8.column);
                            assert_eq!(6, *n8);
                        },
                        _ => assert!(false),
                    }
                    match &(**expr9) {
                        ArithmeticExpression::Number(path9, pos9, n9) => {
                            assert_eq!(&String::from("test.sh"), path9);
                            assert_eq!(1, pos9.line);
                            assert_eq!(25, pos9.column);
                            assert_eq!(7, *n9);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(true, parser.here_docs.is_empty());
}

#[test]
fn test_parser_parse_arith_expr_complains_on_syntax_error()
{
    let s = "))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("syntax error"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_arith_expr_complains_on_unexpected_token()
{
    let s = "()))";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_in_arith_expr();
    let mut parser = Parser::new();
    let settings = Settings::new();
    match parser.parse_arith_expr(&mut lexer, &settings) {
        Err(ParserError::Syntax(path, pos, msg, is_cont)) => {
            assert_eq!(String::from("test.sh"), path);
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
            assert_eq!(String::from("syntax error"), msg);
            assert_eq!(false, is_cont);
        },
        _ => assert!(false),
    }
}
