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
fn test_lexer_get_char_reads_characters()
{
    let s = "abc";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('a', c);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('b', c);
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('c', c);
            assert_eq!(1, pos.line);
            assert_eq!(3, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((None, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(4, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_get_char_reads_characters_for_verbose()
{
    let s = "abc";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut settings = Settings::new();
    settings.verbose_flag = true;
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('a', c);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('b', c);
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('c', c);
            assert_eq!(1, pos.line);
            assert_eq!(3, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((None, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(4, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::from("abc"), lexer.content_for_verbose);
}

#[test]
fn test_lexer_get_char_unreads_characters()
{
    let s = "ab";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('a', c);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            match lexer.get_char(&settings) {
                Ok((Some(c2), pos2)) => {
                    assert_eq!('b', c2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(2, pos2.column);
                    lexer.unget_char(c2, &pos2, &settings);
                    lexer.unget_char(c, &pos, &settings);
                    match lexer.get_char(&settings) {
                        Ok((Some(c3), pos3)) => {
                            assert_eq!('a', c3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                        },
                        _ => assert!(false),
                    }
                    match lexer.get_char(&settings) {
                        Ok((Some(c3), pos3)) => {
                            assert_eq!('b', c3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(2, pos3.column);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::new(), lexer.content_for_verbose);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_get_char_unreads_characters_for_verbose()
{
    let s = "ab";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let mut settings = Settings::new();
    settings.verbose_flag = true;
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('a', c);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            match lexer.get_char(&settings) {
                Ok((Some(c2), pos2)) => {
                    assert_eq!('b', c2);
                    assert_eq!(1, pos2.line);
                    assert_eq!(2, pos2.column);
                    lexer.unget_char(c2, &pos2, &settings);
                    lexer.unget_char(c, &pos, &settings);
                    match lexer.get_char(&settings) {
                        Ok((Some(c3), pos3)) => {
                            assert_eq!('a', c3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(1, pos3.column);
                        },
                        _ => assert!(false),
                    }
                    match lexer.get_char(&settings) {
                        Ok((Some(c3), pos3)) => {
                            assert_eq!('b', c3);
                            assert_eq!(1, pos3.line);
                            assert_eq!(2, pos3.column);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("ab"), lexer.content_for_verbose);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_get_char_reads_newline()
{
    let s = "a\nb";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('a', c);
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('\n', c);
            assert_eq!(1, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((Some(c), pos)) => {
            assert_eq!('b', c);
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    match lexer.get_char(&settings) {
        Ok((None, pos)) => {
            assert_eq!(2, pos.line);
            assert_eq!(2, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_token()
{
    let s = "abc";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_two_tokens()
{
    let s = "abc def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(5, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("def"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_eof()
{
    let s = "abc";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),            
    }
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(4, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_undo_token_undoes_tokens()
{
    let s = "abc def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                    match lexer.next_token(&settings) {
                        Ok((Token::Word(word_elems2), pos2)) => {
                            assert_eq!(1, pos2.line);
                            assert_eq!(5, pos2.column);
                            assert_eq!(1, word_elems2.len());
                            match &word_elems2[0] {
                                WordElement::Simple(SimpleWordElement::String(s2)) => {
                                    assert_eq!(&String::from("def"), s2);
                                    lexer.undo_token(&Token::Word(word_elems2), &pos2);
                                    lexer.undo_token(&Token::Word(word_elems), &pos);
                                    match lexer.next_token(&settings) {
                                        Ok((Token::Word(word_elems3), pos3)) => {
                                            assert_eq!(1, pos3.line);
                                            assert_eq!(1, pos3.column);
                                            assert_eq!(1, word_elems3.len());
                                            match &word_elems3[0] {
                                                WordElement::Simple(SimpleWordElement::String(s3)) => {
                                                    assert_eq!(&String::from("abc"), s3);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match lexer.next_token(&settings) {
                                        Ok((Token::Word(word_elems3), pos3)) => {
                                            assert_eq!(1, pos3.line);
                                            assert_eq!(5, pos3.column);
                                            assert_eq!(1, word_elems3.len());
                                            match &word_elems3[0] {
                                                WordElement::Simple(SimpleWordElement::String(s3)) => {
                                                    assert_eq!(&String::from("def"), s3);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_newline()
{
    let s = "\n";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Newline, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_skips_comment()
{
    let s = "# comment\n";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Newline, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(10, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_skips_comment_for_eof()
{
    let s = "# comment";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(10, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_semi()
{
    let s = ";";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Semi, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_amp()
{
    let s = "&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Amp, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_bar()
{
    let s = "|";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Bar, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_semi_semi()
{
    let s = ";;";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::SemiSemi, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_amp_amp()
{
    let s = "&&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::AmpAmp, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_bar_bar()
{
    let s = "||";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::BarBar, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less()
{
    let s = "<";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Less(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_greater()
{
    let s = ">";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Greater(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_less()
{
    let s = "<<";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessLess(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_less_minus()
{
    let s = "<<-";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessLessMinus(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_greater()
{
    let s = "<>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessGreater(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_amp()
{
    let s = "<&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessAmp(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_greater()
{
    let s = ">>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterGreater(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_amp()
{
    let s = ">&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterAmp(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_bar()
{
    let s = ">|";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterBar(None), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_with_number()
{
    let s = "2<";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Less(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_greater_with_number()
{
    let s = "2>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Greater(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_less_with_number()
{
    let s = "2<<";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessLess(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_less_minus_with_number()
{
    let s = "2<<-";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessLessMinus(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_greater_with_number()
{
    let s = "2<>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessGreater(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_less_amp_with_number()
{
    let s = "2<&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LessAmp(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_greater_with_number()
{
    let s = "2>>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterGreater(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_amp_with_number()
{
    let s = "2>&";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterAmp(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_greater_bar_with_number()
{
    let s = "2>|";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::GreaterBar(Some(n)), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, n);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_lparen()
{
    let s = "(";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LParen, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_rparen()
{
    let s = ")";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::RParen, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_excl_for_first_word()
{
    let s = "!";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::FirstWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Excl, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_lbrace_for_first_word()
{
    let s = "{";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::FirstWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::LBrace, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_rbrace_for_first_word()
{
    let s = "}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::FirstWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::RBrace, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_if_keyword_for_first_word()
{
    let s = "if";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::FirstWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::If, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_in_keyword_for_third_word()
{
    let s = "in";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::ThirdWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::In, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_does_not_return_if_keyword()
{
    let s = "if";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("if"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_string()
{
    let s = "abc";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_space()
{
    let s = "abc\\ def";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc def"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_first_space()
{
    let s = "\\ abcdef";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from(" abcdef"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_splitted_word()
{
    let s = "abc\\\ndef";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abcdef"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_word_with_escapes()
{
    let s = "\\a\\b\\c\\?\\*\\[\\]\\:\\!\\^\\~";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc\\?\\*\\[\\]\\:\\!\\^\\~"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_word_with_variable()
{
    let s = "$var";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_argument()
{
    let s = "$12";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Argument(n), None)) => {
                    assert_eq!(12, *n);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_at_special_parameter()
{
    let s = "$@";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::At, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}   

#[test]
fn test_lexer_next_token_returns_word_with_star_special_parameter()
{
    let s = "$*";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Star, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_hash_special_parameter()
{
    let s = "$#";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Hash, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_word_with_ques_special_parameter()
{
    let s = "$?";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Ques, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_minus_special_parameter()
{
    let s = "$-";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Minus, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_dolar_special_parameter()
{
    let s = "$$";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Dolar, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_excl_special_parameter()
{
    let s = "$!";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Special(spec_name), None)) => {
                    assert_eq!(SpecialParameterName::Excl, *spec_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_word_with_parameter_in_brace()
{
    let s = "${var}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_length()
{
    let s = "${#var}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::ParameterLength(ParameterName::Variable(var_name))) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_singly_quoted_string()
{
    let s = "'abc def'";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::SinglyQuoted(s) => {
                    assert_eq!(&String::from("abc def"), s);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_word_with_doubly_quoted_string()
{
    let s = "\"abc $var\"";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::DoublyQuoted(simple_word_elems) => {
                    assert_eq!(2, simple_word_elems.len());
                    match &simple_word_elems[0] {
                        SimpleWordElement::String(s) => {
                            assert_eq!(&String::from("abc "), s);
                        },
                        _ => assert!(false),
                    }
                    match &simple_word_elems[1] {
                        SimpleWordElement::Parameter(ParameterName::Variable(var_name), None) => {
                            assert_eq!(&String::from("var"), var_name);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_elements()
{
    let s = "abc$var";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(2, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::String(s)) => {
                    assert_eq!(&String::from("abc"), s);
                },
                _ => assert!(false),
            }
            match &word_elems[1] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), None)) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_here_document_word()
{
    let s = "abc$var";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::HereDocumentWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::HereDocWord(s), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("abc$var"), s);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_here_document_without_minus()
{
    let s = "
abc def
ghi$var
jkl mno
EOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InHereDocument(String::from("EOT"), false));
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::HereDoc(simple_word_elems, is_minus), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(5, simple_word_elems.len());
            match &simple_word_elems[0] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("abc def\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[1] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("ghi"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[2] {
                SimpleWordElement::Parameter(ParameterName::Variable(var_name), None) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[3] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[4] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("jkl mno\n"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(false, is_minus);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_here_document_without_minus_for_delim_with_eof()
{
    let s = "
abc def
ghi$var
jkl mno
EOT";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InHereDocument(String::from("EOT"), false));
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::HereDoc(simple_word_elems, is_minus), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(5, simple_word_elems.len());
            match &simple_word_elems[0] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("abc def\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[1] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("ghi"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[2] {
                SimpleWordElement::Parameter(ParameterName::Variable(var_name), None) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[3] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[4] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("jkl mno\n"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(false, is_minus);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_here_document_with_minus()
{
    let s = "
\t\tabc def
\tghi$var
\tjkl mno
\t\tEOT
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InHereDocument(String::from("EOT"), true));
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::HereDoc(simple_word_elems, is_minus), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(5, simple_word_elems.len());
            match &simple_word_elems[0] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("abc def\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[1] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("ghi"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[2] {
                SimpleWordElement::Parameter(ParameterName::Variable(var_name), None) => {
                    assert_eq!(&String::from("var"), var_name);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[3] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("\n"), s);
                },
                _ => assert!(false),
            }
            match &simple_word_elems[4] {
                SimpleWordElement::String(s) => {
                    assert_eq!(&String::from("jkl mno\n"), s);
                },
                _ => assert!(false),
            }
            assert_eq!(true, is_minus);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_eof_for_in_command_substitution()
{
    let s = ")xxx";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InCommandSubstitution);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}    

#[test]
fn test_lexer_next_token_returns_eof_for_in_command_substitution_with_first_word()
{
    let s = ")xxx";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InCommandSubstitution);
    lexer.push_state(State::FirstWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_eof_for_in_command_substitution_with_third_word()
{
    let s = ")xxx";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InCommandSubstitution);
    lexer.push_state(State::ThirdWord);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_eof_for_in_parameter_expansion()
{
    let s = "}xxx";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    lexer.push_state(State::InParameterExpansion);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::EOF, pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_colon_minus_modifier()
{
    let s = "${var:-abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::ColonMinus, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_minus_modifier()
{
    let s = "${var-abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Minus, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_colon_equal_modifier()
{
    let s = "${var:=abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::ColonEqual, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_equal_modifier()
{
    let s = "${var=abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Equal, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_colon_ques_modifier()
{
    let s = "${var:?abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::ColonQues, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_ques_modifier()
{
    let s = "${var?abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Ques, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_colon_plus_modifier()
{
    let s = "${var:+abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::ColonPlus, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_plus_modifier()
{
    let s = "${var+abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Plus, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_perc_modifier()
{
    let s = "${var%abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Perc, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_perc_perc_modifier()
{
    let s = "${var%%abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::PercPerc, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_hash_modifier()
{
    let s = "${var#abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::Hash, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(7, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(11, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_parameter_with_hash_hash_modifier()
{
    let s = "${var##abc def}";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Parameter(ParameterName::Variable(var_name), Some((ParameterModifier::HashHash, words)))) => {
                    assert_eq!(&String::from("var"), var_name);
                    assert_eq!(2, words.len());
                    assert_eq!(String::from("test.sh"), words[0].path);
                    assert_eq!(1, words[0].pos.line);
                    assert_eq!(8, words[0].pos.column);
                    assert_eq!(1, words[0].word_elems.len());
                    match &words[0].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("abc"), s);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(String::from("test.sh"), words[1].path);
                    assert_eq!(1, words[1].pos.line);
                    assert_eq!(12, words[1].pos.column);
                    assert_eq!(1, words[1].word_elems.len());
                    match &words[1].word_elems[0] {
                        WordElement::Simple(SimpleWordElement::String(s)) => {
                            assert_eq!(&String::from("def"), s);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_command()
{
    let s = "$(echo abc def)";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Command(logical_commands)) => {
                    assert_eq!(1, logical_commands.len());
                    assert_eq!(String::from("test.sh"), logical_commands[0].path);
                    assert_eq!(1, logical_commands[0].pos.line);
                    assert_eq!(3, logical_commands[0].pos.column);
                    assert_eq!(false, logical_commands[0].is_in_background);
                    assert_eq!(true, logical_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                    assert_eq!(1, logical_commands[0].first_command.pos.line);
                    assert_eq!(3, logical_commands[0].first_command.pos.column);
                    assert_eq!(false, logical_commands[0].first_command.is_negative);
                    assert_eq!(1, logical_commands[0].first_command.commands.len());
                    match &(*logical_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(3, pos.column);
                            assert_eq!(3, simple_command.words.len());
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
                            assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                            assert_eq!(1, simple_command.words[2].pos.line);
                            assert_eq!(12, simple_command.words[2].pos.column);
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
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_nested_command()
{
    let s = "$(echo abc $(echo def) )";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Command(logical_commands)) => {
                    assert_eq!(1, logical_commands.len());
                    assert_eq!(String::from("test.sh"), logical_commands[0].path);
                    assert_eq!(1, logical_commands[0].pos.line);
                    assert_eq!(3, logical_commands[0].pos.column);
                    assert_eq!(false, logical_commands[0].is_in_background);
                    assert_eq!(true, logical_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                    assert_eq!(1, logical_commands[0].first_command.pos.line);
                    assert_eq!(3, logical_commands[0].first_command.pos.column);
                    assert_eq!(false, logical_commands[0].first_command.is_negative);
                    assert_eq!(1, logical_commands[0].first_command.commands.len());
                    match &(*logical_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(3, pos.column);
                            assert_eq!(3, simple_command.words.len());
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
                            assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                            assert_eq!(1, simple_command.words[2].pos.line);
                            assert_eq!(12, simple_command.words[2].pos.column);
                            assert_eq!(1, simple_command.words[2].word_elems.len());
                            match &simple_command.words[2].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Command(logical_commands2)) => {
                                    assert_eq!(1, logical_commands2.len());
                                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                                    assert_eq!(1, logical_commands2[0].pos.line);
                                    assert_eq!(14, logical_commands2[0].pos.column);
                                    assert_eq!(false, logical_commands2[0].is_in_background);
                                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                                    assert_eq!(1, logical_commands2[0].first_command.pos.line);
                                    assert_eq!(14, logical_commands2[0].first_command.pos.column);
                                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                                    match &(*logical_commands2[0].first_command.commands[0]) {
                                        Command::Simple(path, pos, simple_command2) => {
                                            assert_eq!(&String::from("test.sh"), path);
                                            assert_eq!(1, pos.line);
                                            assert_eq!(14, pos.column);
                                            assert_eq!(2, simple_command2.words.len());
                                            assert_eq!(String::from("test.sh"), simple_command2.words[0].path);
                                            assert_eq!(1, simple_command2.words[0].pos.line);
                                            assert_eq!(14, simple_command2.words[0].pos.column);
                                            assert_eq!(1, simple_command2.words[0].word_elems.len());
                                            match &simple_command2.words[0].word_elems[0] {
                                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                                    assert_eq!(&String::from("echo"), s);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(String::from("test.sh"), simple_command2.words[1].path);
                                            assert_eq!(1, simple_command2.words[1].pos.line);
                                            assert_eq!(19, simple_command2.words[1].pos.column);
                                            assert_eq!(1, simple_command2.words[1].word_elems.len());
                                            match &simple_command2.words[1].word_elems[0] {
                                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                                    assert_eq!(&String::from("def"), s);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(true, simple_command2.redirects.is_empty());
                                        },
                                        _ => assert!(false),
                                    }
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
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_command_for_backquote()
{
    let s = "`echo abc def`";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Command(logical_commands)) => {
                    assert_eq!(1, logical_commands.len());
                    assert_eq!(String::from("test.sh"), logical_commands[0].path);
                    assert_eq!(1, logical_commands[0].pos.line);
                    assert_eq!(2, logical_commands[0].pos.column);
                    assert_eq!(false, logical_commands[0].is_in_background);
                    assert_eq!(true, logical_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                    assert_eq!(1, logical_commands[0].first_command.pos.line);
                    assert_eq!(2, logical_commands[0].first_command.pos.column);
                    assert_eq!(1, logical_commands[0].first_command.commands.len());
                    match &(*logical_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(2, pos.column);
                            assert_eq!(3, simple_command.words.len());
                            assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                            assert_eq!(1, simple_command.words[0].pos.line);
                            assert_eq!(2, simple_command.words[0].pos.column);
                            assert_eq!(false, logical_commands[0].first_command.is_negative);
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
                            assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                            assert_eq!(1, simple_command.words[2].pos.line);
                            assert_eq!(11, simple_command.words[2].pos.column);
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
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_nested_command_for_backquote()
{
    let s = "`echo abc \\`echo def\\``";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::Simple(SimpleWordElement::Command(logical_commands)) => {
                    assert_eq!(1, logical_commands.len());
                    assert_eq!(String::from("test.sh"), logical_commands[0].path);
                    assert_eq!(1, logical_commands[0].pos.line);
                    assert_eq!(2, logical_commands[0].pos.column);
                    assert_eq!(false, logical_commands[0].is_in_background);
                    assert_eq!(true, logical_commands[0].pairs.is_empty());
                    assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                    assert_eq!(1, logical_commands[0].first_command.pos.line);
                    assert_eq!(2, logical_commands[0].first_command.pos.column);
                    assert_eq!(false, logical_commands[0].first_command.is_negative);
                    assert_eq!(1, logical_commands[0].first_command.commands.len());
                    match &(*logical_commands[0].first_command.commands[0]) {
                        Command::Simple(path, pos, simple_command) => {
                            assert_eq!(&String::from("test.sh"), path);
                            assert_eq!(1, pos.line);
                            assert_eq!(2, pos.column);
                            assert_eq!(3, simple_command.words.len());
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
                            assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                            assert_eq!(1, simple_command.words[2].pos.line);
                            assert_eq!(11, simple_command.words[2].pos.column);
                            assert_eq!(1, simple_command.words[2].word_elems.len());
                            match &simple_command.words[2].word_elems[0] {
                                WordElement::Simple(SimpleWordElement::Command(logical_commands2)) => {
                                    assert_eq!(1, logical_commands2.len());
                                    assert_eq!(String::from("test.sh"), logical_commands2[0].path);
                                    assert_eq!(1, logical_commands2[0].pos.line);
                                    assert_eq!(13, logical_commands2[0].pos.column);
                                    assert_eq!(false, logical_commands2[0].is_in_background);
                                    assert_eq!(true, logical_commands2[0].pairs.is_empty());
                                    assert_eq!(String::from("test.sh"), logical_commands2[0].first_command.path);
                                    assert_eq!(1, logical_commands2[0].first_command.pos.line);
                                    assert_eq!(13, logical_commands2[0].first_command.pos.column);
                                    assert_eq!(false, logical_commands2[0].first_command.is_negative);
                                    assert_eq!(1, logical_commands2[0].first_command.commands.len());
                                    match &(*logical_commands2[0].first_command.commands[0]) {
                                        Command::Simple(path, pos, simple_command2) => {
                                            assert_eq!(&String::from("test.sh"), path);
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(2, simple_command2.words.len());
                                            assert_eq!(String::from("test.sh"), simple_command2.words[0].path);
                                            assert_eq!(1, simple_command2.words[0].pos.line);
                                            assert_eq!(13, simple_command2.words[0].pos.column);
                                            assert_eq!(1, simple_command2.words[0].word_elems.len());
                                            match &simple_command2.words[0].word_elems[0] {
                                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                                    assert_eq!(&String::from("echo"), s);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(String::from("test.sh"), simple_command2.words[1].path);
                                            assert_eq!(1, simple_command2.words[1].pos.line);
                                            assert_eq!(18, simple_command2.words[1].pos.column);
                                            assert_eq!(1, simple_command2.words[1].word_elems.len());
                                            match &simple_command2.words[1].word_elems[0] {
                                                WordElement::Simple(SimpleWordElement::String(s)) => {
                                                    assert_eq!(&String::from("def"), s);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(true, simple_command2.redirects.is_empty());
                                        },
                                        _ => assert!(false),
                                    }
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
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_command_in_doubly_quoted_string()
{
    let s = "\"$(echo abc def)\"";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::DoublyQuoted(simple_word_elems) => {
                    assert_eq!(1, simple_word_elems.len());
                    match &simple_word_elems[0] {
                        SimpleWordElement::Command(logical_commands) => {
                            assert_eq!(1, logical_commands.len());
                            assert_eq!(String::from("test.sh"), logical_commands[0].path);
                            assert_eq!(1, logical_commands[0].pos.line);
                            assert_eq!(4, logical_commands[0].pos.column);
                            assert_eq!(false, logical_commands[0].is_in_background);
                            assert_eq!(true, logical_commands[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                            assert_eq!(1, logical_commands[0].first_command.pos.line);
                            assert_eq!(4, logical_commands[0].first_command.pos.column);
                            assert_eq!(false, logical_commands[0].first_command.is_negative);
                            assert_eq!(1, logical_commands[0].first_command.commands.len());
                            match &(*logical_commands[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(1, pos.line);
                                    assert_eq!(4, pos.column);
                                    assert_eq!(3, simple_command.words.len());
                                    assert_eq!(String::from("test.sh"), simple_command.words[0].path);
                                    assert_eq!(1, simple_command.words[0].pos.line);
                                    assert_eq!(4, simple_command.words[0].pos.column);
                                    assert_eq!(1, simple_command.words[0].word_elems.len());
                                    match &simple_command.words[0].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("echo"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[1].path);
                                    assert_eq!(1, simple_command.words[1].pos.line);
                                    assert_eq!(9, simple_command.words[1].pos.column);
                                    assert_eq!(1, simple_command.words[1].word_elems.len());
                                    match &simple_command.words[1].word_elems[0] {
                                        WordElement::Simple(SimpleWordElement::String(s)) => {
                                            assert_eq!(&String::from("abc"), s);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                                    assert_eq!(1, simple_command.words[2].pos.line);
                                    assert_eq!(13, simple_command.words[2].pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}

#[test]
fn test_lexer_next_token_returns_word_with_command_in_doubly_quoted_string_for_backquoted()
{
    let s = "\"`echo abc def`\"";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut cr = CharReader::new(&mut cursor);
    let mut lexer = Lexer::new("test.sh", &Position::new(1, 1), &mut cr, 0, false);
    let settings = Settings::new();
    match lexer.next_token(&settings) {
        Ok((Token::Word(word_elems), pos)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(1, word_elems.len());
            match &word_elems[0] {
                WordElement::DoublyQuoted(simple_word_elems) => {
                    assert_eq!(1, simple_word_elems.len());
                    match &simple_word_elems[0] {
                        SimpleWordElement::Command(logical_commands) => {
                            assert_eq!(1, logical_commands.len());
                            assert_eq!(String::from("test.sh"), logical_commands[0].path);
                            assert_eq!(1, logical_commands[0].pos.line);
                            assert_eq!(3, logical_commands[0].pos.column);
                            assert_eq!(false, logical_commands[0].is_in_background);
                            assert_eq!(true, logical_commands[0].pairs.is_empty());
                            assert_eq!(String::from("test.sh"), logical_commands[0].first_command.path);
                            assert_eq!(1, logical_commands[0].first_command.pos.line);
                            assert_eq!(3, logical_commands[0].first_command.pos.column);
                            assert_eq!(false, logical_commands[0].first_command.is_negative);
                            assert_eq!(1, logical_commands[0].first_command.commands.len());
                            match &(*logical_commands[0].first_command.commands[0]) {
                                Command::Simple(path, pos, simple_command) => {
                                    assert_eq!(&String::from("test.sh"), path);
                                    assert_eq!(1, pos.line);
                                    assert_eq!(3, pos.column);
                                    assert_eq!(3, simple_command.words.len());
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
                                    assert_eq!(String::from("test.sh"), simple_command.words[2].path);
                                    assert_eq!(1, simple_command.words[2].pos.line);
                                    assert_eq!(12, simple_command.words[2].pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    assert_eq!(String::new(), lexer.content_for_verbose);
}
