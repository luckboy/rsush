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
use std::collections::HashMap;
use std::io::*;
use std::rc::*;
use crate::io::*;
use crate::parser::*;
use crate::settings::*;

#[derive(Clone, PartialEq)]
enum State
{
    Initial,
    InParameterExpansion,
    InCommandSubstitution,
    HereDocumentWord,
    InHereDocument(String, bool),
    FirstWord,
    ThirdWord,
    InArithmeticExpression,
    InArithmeticExpressionAndParentheses,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SpecialParameterName
{
    At,
    Star,
    Hash,
    Ques,
    Minus,
    Dolar,
    Excl,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ParameterModifier
{
    ColonMinus,
    Minus,
    ColonEqual,
    Equal,
    ColonQues,
    Ques,
    ColonPlus,
    Plus,
    Perc,
    PercPerc,
    Hash,
    HashHash,
}

#[derive(Clone, PartialEq)]
pub enum ParameterName
{
    Variable(String),
    Argument(usize),
    Special(SpecialParameterName),
}

#[derive(Clone)]
pub enum SimpleWordElement
{
    String(String),
    Parameter(ParameterName, Option<(ParameterModifier, Vec<Rc<Word>>)>),
    ParameterLength(ParameterName),
    Command(Vec<Rc<Command>>),
    ArithmeticExpression(ArithmeticExpression),
}

#[derive(Clone)]
pub enum WordElement
{
    Simple(SimpleWordElement),
    SinglyQuoted(String),
    DoublyQuoted(Vec<SimpleWordElement>),
}

#[derive(Clone)]
pub enum Token
{
    Newline,
    Semi,
    Less(Option<i32>),
    Greater(Option<i32>),
    Amp,
    Bar,
    SemiSemi,
    LessLess,
    LessLessMinus,
    LessGreater(Option<i32>),
    LessAmp(Option<i32>),
    GreaterGreater(Option<i32>),
    GreaterAmp(Option<i32>),
    GreaterBar(Option<i32>),
    AmpAmp,
    BarBar,
    LParen,
    RParen,
    Excl,
    LBrace,
    RBrace,
    Case,
    Do,
    Done,
    Elif,
    Else,
    Esac,
    Fi,
    For,
    If,
    In,
    Then,
    Until,
    While,
    Word(Vec<WordElement>),
    HereDocWord(String),
    HereDoc(Vec<WordElement>, bool),
    EOF,
}

#[derive(Clone)]
pub enum ArithmeticToken
{
    LParen,
    RParen,
    Tylda,
    Excl,
    Star,
    Slash,
    Perc,
    Plus,
    Minus,
    LessLess,
    GreaterGreater,
    Less,
    GreaterEqual,
    Greater,
    LessEqual,
    EqualEqual,
    ExclEqual,
    Amp,
    Caret,
    Bar,
    AmpAmp,
    BarBar,
    Ques,
    Colon,
    Equal,
    StarEqual,
    SlashEqual,
    PercEqual,
    PlusEqual,
    MinusEqual,
    LessLessEqual,
    GreaterGreaterEqual,
    AmpEqual,
    CaretEqual,
    BarEqual,
    Number(i64),
    Parameter(ParameterName),
    EOF,
}

pub struct Lexer<'a>
{
    reader: &'a mut dyn CharRead,
    pushed_chars: Vec<(char, Position)>,
    pushed_tokens: Vec<(Token, Position)>,
    pushed_arith_tokens: Vec<(ArithmeticToken, Position)>,
    state_stack: Vec<State>,
    current_state: State,
    path: String,
    pos: Position,
    content_for_verbose: String,
    has_ignored_eof: bool,
    first_keywords: HashMap<String, Token>,
    second_keywords: HashMap<String, Token>,
}

impl<'a> Lexer<'a>
{
    pub fn new(path: &str, reader: &'a mut dyn CharRead, is_ignored_eof: bool) -> Lexer<'a>
    {
        let mut first_keywords: HashMap<String, Token> = HashMap::new();
        first_keywords.insert(String::from("!"), Token::Excl);
        first_keywords.insert(String::from("{"), Token::LBrace);
        first_keywords.insert(String::from("}"), Token::RBrace);
        first_keywords.insert(String::from("case"), Token::Case);
        first_keywords.insert(String::from("do"), Token::Do);
        first_keywords.insert(String::from("done"), Token::Done);
        first_keywords.insert(String::from("elif"), Token::Elif);
        first_keywords.insert(String::from("else"), Token::Else);
        first_keywords.insert(String::from("esac"), Token::Esac);
        first_keywords.insert(String::from("fi"), Token::Fi);
        first_keywords.insert(String::from("for"), Token::For);
        first_keywords.insert(String::from("if"), Token::If);
        first_keywords.insert(String::from("then"), Token::Then);
        first_keywords.insert(String::from("until"), Token::Until);
        first_keywords.insert(String::from("while"), Token::While);
        let mut second_keywords: HashMap<String, Token> = HashMap::new();
        second_keywords.insert(String::from("in"), Token::In);
        Lexer {
            reader,
            pushed_chars: Vec::new(),
            pushed_tokens: Vec::new(),
            pushed_arith_tokens: Vec::new(),
            state_stack: Vec::new(),
            current_state: State::Initial,
            path: String::from(path),
            pos: Position::new(1, 1),
            content_for_verbose: String::new(),
            has_ignored_eof: is_ignored_eof,
            first_keywords,
            second_keywords,
        }
    }

    pub fn path(&self) -> String
    { self.path.clone() }

    pub fn pos(&self) -> Position
    { self.pos }

    pub fn push_here_doc_word(&mut self)
    { self.push_state(&State::HereDocumentWord); }

    pub fn push_in_here_doc(&mut self, s: &str, is_minus: bool)
    { self.push_state(&State::InHereDocument(String::from(s), is_minus)); }

    pub fn push_first_word(&mut self)
    { self.push_state(&State::FirstWord); }    

    pub fn push_third_word(&mut self)
    { self.push_state(&State::ThirdWord); }    

    fn push_state(&mut self, state: &State)
    {
        self.state_stack.push(self.current_state.clone());
        self.current_state = state.clone();
    }

    pub fn pop_state(&mut self)
    { self.current_state = self.state_stack.pop().unwrap(); }
    
    fn get_char(&mut self, settings: &Settings) -> ParserResult<(Option<char>, Position)>
    {
        let res = match self.pushed_chars.pop() {
            Some((c, pos)) => {
                self.pos = pos;
                Ok((Some(c), pos))
            },
            None => {
                loop {
                    match self.reader.get_char() {
                        Ok(None)    => {
                            if self.has_ignored_eof && settings.ignoreeof_flag {
                                continue;
                            } else {
                                break Ok((None, self.pos));
                            }
                        },
                        Ok(Some(c)) => break Ok((Some(c), self.pos)),
                        Err(err)    => break Err(ParserError::IO(self.path.clone(), err)),
                    }
                }
            },
        };
        match res {
            Ok((Some(c), pos)) => {
                if c == '\n' {
                    self.pos.line += 1;
                    self.pos.column = 1;
                } else {
                    self.pos.column += 1;
                }
                if settings.verbose_flag {
                    self.content_for_verbose.push(c);
                }
                Ok((Some(c), pos))
            },
            res => res,
        }
    }
    
    fn unget_char(&mut self, c: char, pos: &Position, settings: &Settings)
    {
        self.pushed_chars.push((c, *pos));
        self.pos = *pos;
        if settings.verbose_flag {
            let _c2 = self.content_for_verbose.pop();
        }
    }
    
    fn skip_comment(&mut self, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some(c @ '\n'), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(_), _) => (),
            }
        }
        Ok(())
    }
    
    fn skip_spaces(&mut self, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some(c @ '\\'), pos) => {
                    match self.get_char(settings)? {
                        (None, _) => (),
                        (Some('\n'), _) => (),
                        (Some(c2), pos2) => {
                            self.unget_char(c2, &pos2, settings);
                            self.unget_char(c, &pos, settings);
                            break;
                        },
                    }
                },
                (Some('#'), _) => self.skip_comment(settings)?,
                (Some(c @ '\n'), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), _) if c.is_whitespace() => (),
                (Some(c), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
            }
        }
        Ok(())
    }

    fn get_less_token(&mut self, n: Option<i32>, token_pos: &Position, settings: &Settings) -> ParserResult<(Token, Position)>
    {
        match self.get_char(settings)? {
            (None, _) => Ok((Token::Less(n), *token_pos)),
            (Some('<'), _) => {
                match self.get_char(settings)? {
                    (None, _) => Ok((Token::LessLess, *token_pos)),
                    (Some('-'), _) => Ok((Token::LessLessMinus, *token_pos)),
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        Ok((Token::LessLess, *token_pos))
                    },
                }
            },
            (Some('>'), _) => Ok((Token::LessGreater(n), *token_pos)),
            (Some('&'), _) => Ok((Token::LessAmp(n), *token_pos)),
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                Ok((Token::Less(n), *token_pos))
            },
        }
    }

    fn get_greater_token(&mut self, n: Option<i32>, token_pos: &Position, settings: &Settings) -> ParserResult<(Token, Position)>
    {
        match self.get_char(settings)? {
            (None, _) => Ok((Token::Greater(n), *token_pos)),
            (Some('>'), _) => Ok((Token::GreaterGreater(n), *token_pos)),
            (Some('&'), _) => Ok((Token::GreaterAmp(n), *token_pos)),
            (Some('|'), _) => Ok((Token::GreaterBar(n), *token_pos)),
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                Ok((Token::Greater(n), *token_pos))
            },
        }
    }
    
    fn get_less_or_greater_token(&mut self, n: Option<i32>, token_pos: &Position, settings: &Settings) -> ParserResult<Option<(Token, Position)>>
    {
        match self.get_char(settings)? {
            (None, _) => Ok(None),
            (Some('<'), _) => Ok(Some(self.get_less_token(n, token_pos, settings)?)),
            (Some('>'), _) => Ok(Some(self.get_greater_token(n, token_pos, settings)?)),
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                Ok(None)
            },
        }
    }
    
    fn get_param_name(&mut self, settings: &Settings) -> ParserResult<Option<ParameterName>>
    {
        let param_name_pos = self.pos;
        match self.get_char(settings)? {
            (None, _) => Ok(None),
            (Some('@'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::At))),
            (Some('*'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Star))),
            (Some('#'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Hash))),
            (Some('?'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Ques))),
            (Some('-'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Minus))),
            (Some('$'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Dolar))),
            (Some('!'), _) => Ok(Some(ParameterName::Special(SpecialParameterName::Excl))),
            (Some(c @ ('0'..='9')), _) => {
                let mut s = String::new();
                s.push(c);
                loop {
                    match self.get_char(settings)? {
                        (None, _) => break,
                        (Some(c2 @ ('0'..='9')), _) => s.push(c2),
                        (Some(c2), pos2) => {
                            self.unget_char(c2, &pos2, settings);
                            break;
                        },
                    }
                }
                match s.parse::<usize>() {
                    Ok(n) => Ok(Some(ParameterName::Argument(n))),
                    Err(_) => Err(ParserError::Syntax(self.path.clone(), param_name_pos, String::from("too large argument number"), false)),
                }
            },
            (Some(c), _) if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                loop {
                    match self.get_char(settings)? {
                        (None, _) => break,
                        (Some(c2), _) if c2.is_alphanumeric() || c == '_' => s.push(c2),
                        (Some(c2), pos2) => {
                            self.unget_char(c2, &pos2, settings);
                            break;
                        },
                    }
                }
                Ok(Some(ParameterName::Variable(s)))
            },
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                Ok(None)
            },
        }
    }
    
    fn get_param_modifier(&mut self, settings: &Settings) -> ParserResult<Option<ParameterModifier>>
    {
        match self.get_char(settings)? {
            (None, _) => Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("unexpected end of file"), true)),
            (Some(':'), _) => {
                match self.get_char(settings)? {
                    (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                    (Some('-'), _) => Ok(Some(ParameterModifier::ColonMinus)),
                    (Some('='), _) => Ok(Some(ParameterModifier::ColonEqual)),
                    (Some('?'), _) => Ok(Some(ParameterModifier::ColonQues)),
                    (Some('+'), _) => Ok(Some(ParameterModifier::ColonPlus)),
                    (Some(_), pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected character"), false)),
                }
            },
            (Some('-'), _) => Ok(Some(ParameterModifier::Minus)),
            (Some('='), _) => Ok(Some(ParameterModifier::Equal)),
            (Some('?'), _) => Ok(Some(ParameterModifier::Ques)),
            (Some('+'), _) => Ok(Some(ParameterModifier::Plus)),
            (Some('%'), _) => {
                match self.get_char(settings)? {
                    (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                    (Some('%'), _) => Ok(Some(ParameterModifier::PercPerc)),
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        Ok(Some(ParameterModifier::Perc))
                    },
                }
            },
            (Some('#'), _) => {
                match self.get_char(settings)? {
                    (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                    (Some('#'), _) => Ok(Some(ParameterModifier::PercPerc)),
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        Ok(Some(ParameterModifier::Perc))
                    },
                }
            },
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                Ok(None)
            },
        }
    }
    
    fn get_dolar_simple_word_elem(&mut self, settings: &Settings) -> ParserResult<SimpleWordElement>
    {
        match self.get_char(settings)? {
            (None, _) => Ok(SimpleWordElement::String(String::from("$"))),
            (Some('{'), _) => {
                match self.get_char(settings)? {
                    (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                    (Some('#'), _) => {
                        let param_name = self.get_param_name(settings)?;
                        match param_name {
                            Some(param_name) => {
                                match self.get_char(settings)? {
                                    (None, pos3) => Err(ParserError::Syntax(self.path.clone(), pos3, String::from("unexpected end of file"), true)),
                                    (Some('}'), _) => Ok(SimpleWordElement::ParameterLength(param_name)),
                                    (Some(_), pos3) => Err(ParserError::Syntax(self.path.clone(), pos3, String::from("unexpected character"), false)),
                                }
                            },
                            None => Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("no parameter name"), false)),
                        }
                    },
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        let param_name = self.get_param_name(settings)?;
                        match param_name {
                            Some(param_name) => {
                                match self.get_param_modifier(settings)? {
                                    Some(modifier) => {
                                        let mut parser = Parser::new();
                                        self.push_state(&State::InParameterExpansion);
                                        let words = parser.parse_words(self, settings)?;
                                        self.pop_state();
                                        Ok(SimpleWordElement::Parameter(param_name, Some((modifier, words))))
                                    },
                                    None => {
                                        match self.get_char(settings)? {
                                            (None, pos3) => Err(ParserError::Syntax(self.path.clone(), pos3, String::from("unexpected end of file"), true)),
                                            (Some('}'), _) => Ok(SimpleWordElement::Parameter(param_name, None)),
                                            (Some(_), pos3) => Err(ParserError::Syntax(self.path.clone(), pos3, String::from("unexpected character"), false)),
                                        }
                                    },
                                }
                            },
                            None => Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("no parameter name"), false)),
                        }
                    },
                }
            },
            (Some('('), _) => {
                match self.get_char(settings)? {
                    (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                    (Some('('), _) => {
                        let mut parser = Parser::new();
                        self.push_state(&State::InParameterExpansion);
                        let arith_expr = parser.parse_arith_expr(self, settings)?;
                        self.pop_state();
                        Ok(SimpleWordElement::ArithmeticExpression(arith_expr))
                    },
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        let mut parser = Parser::new();
                        self.push_state(&State::InParameterExpansion);
                        let commands = parser.parse_commands(self, settings)?;
                        self.pop_state();
                        Ok(SimpleWordElement::Command(commands))
                    },
                }
            },
            (Some(c), pos) => {
                self.unget_char(c, &pos, settings);
                let param_name = self.get_param_name(settings)?;
                match param_name {
                    Some(param_name) => Ok(SimpleWordElement::Parameter(param_name, None)),
                    None => Ok(SimpleWordElement::String(String::from("$"))),
                }
            },
        }
    }
    
    fn get_backquote_simple_word_elem(&mut self, settings: &Settings) -> ParserResult<SimpleWordElement>
    {
        let mut s = String::new();
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                (Some(c @ '\\'), _) => {
                    match self.get_char(settings)? {
                        (None, pos2) => return Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                        (Some('`'), _) => s.push('`'),
                        (Some(c2), _) => {
                            s.push(c);
                            s.push(c2);
                        },
                    }
                },
                (Some('`'), _) => break,
                (Some(c), _) => s.push(c),
            }
        }
        let mut cursor = Cursor::new(s.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new(&format!("{}: {}", self.path, self.pos), &mut cr, false);
        let mut parser = Parser::new();
        let commands = parser.parse_commands(&mut lexer, settings)?;
        Ok(SimpleWordElement::Command(commands))
    }
    
    fn get_string_simple_word_elem_for_word_elem(&mut self, settings: &Settings) -> ParserResult<(SimpleWordElement, bool)>
    {
        let mut s = String::new();
        let mut can_be_keyword = true;
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some('\\'), _) => {
                    can_be_keyword = false;
                    match self.get_char(settings)? {
                        (None, _) => {
                            s.push('\\');
                            break;
                        },
                        (Some('\n'), _) => (),
                        (Some(c2 @ ('?' | '*' | '[' | ']' | ':' | '!' | '^' | '~')), _) => {
                            s.push('\\');
                            s.push(c2);
                        },
                        (Some(c2), _) => s.push(c2),
                    }
                },
                (Some(c @ ('<' | '>' | '&' | '|' | '(' | ')' | '$' | '`' | '\'' | '"' | '#')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) if c.is_whitespace() => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), _) => s.push(c),
            }
        }
        Ok((SimpleWordElement::String(s), can_be_keyword))
    }
    
    fn get_string_simple_word_elem_for_doubly_quoted(&mut self, settings: &Settings) -> ParserResult<SimpleWordElement>
    {
        let mut s = String::new();
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                (Some('\\'), _) => {
                    match self.get_char(settings)? {
                        (None, _) => {
                            s.push('\\');
                            break;
                        },
                        (Some('\n'), _) => (),
                        (Some(c2), _) => s.push(c2),
                    }
                },
                (Some(c @ ('$' | '`' | '"')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), _) => s.push(c),
            }
        }
        Ok(SimpleWordElement::String(s))
    }
    
    fn read_simple_word_elems_for_doubly_quoted(&mut self, simple_word_elems: &mut Vec<SimpleWordElement>, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                (Some('$'), _) => {
                    let simple_word_elem = self.get_dolar_simple_word_elem(settings)?;
                    simple_word_elems.push(simple_word_elem);
                }
                (Some('`'), _) => {
                    let simple_word_elem = self.get_backquote_simple_word_elem(settings)?;
                    simple_word_elems.push(simple_word_elem);
                }
                (Some('"'), _) => break,
                (Some(c), pos) => {
                    self.unget_char(c, &pos, settings);
                    let simple_word_elem = self.get_string_simple_word_elem_for_doubly_quoted(settings)?;
                    simple_word_elems.push(simple_word_elem);
                },
            }
        }
        Ok(())
    }
    
    fn get_dolar_word_elem(&mut self, settings: &Settings) -> ParserResult<WordElement>
    {
        let simple_word_elem = self.get_dolar_simple_word_elem(settings)?;
        Ok(WordElement::Simple(simple_word_elem))
    }

    fn get_backquote_word_elem(&mut self, settings: &Settings) -> ParserResult<WordElement>
    {
        let simple_word_elem = self.get_backquote_simple_word_elem(settings)?;
        Ok(WordElement::Simple(simple_word_elem))
    }

    fn get_string_word_elem(&mut self, settings: &Settings) -> ParserResult<(WordElement, bool)>
    {
        let (simple_word_elem, can_be_reserved_word) = self.get_string_simple_word_elem_for_word_elem(settings)?;
        Ok((WordElement::Simple(simple_word_elem), can_be_reserved_word))
    }
    
    fn get_singly_quoted_word_elem(&mut self, settings: &Settings) -> ParserResult<WordElement>
    {
        let mut s = String::new();
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)), 
                (Some('\''), _) => break,
                (Some(c), _) => s.push(c),
            }
        }
        Ok(WordElement::SinglyQuoted(s))
    }

    fn get_doubly_quoted_word_elem(&mut self, settings: &Settings) -> ParserResult<WordElement>
    {
        let mut simple_word_elems: Vec<SimpleWordElement> = Vec::new();
        self.read_simple_word_elems_for_doubly_quoted(&mut simple_word_elems, settings)?;
        Ok(WordElement::DoublyQuoted(simple_word_elems))
    }
    
    fn read_word_elems(&mut self, word_elems: &mut Vec<WordElement>, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some('$'), _) => {
                    let word_elem = self.get_dolar_word_elem(settings)?;
                    word_elems.push(word_elem);
                },
                (Some('`'), _) => {
                    let word_elem = self.get_backquote_word_elem(settings)?;
                    word_elems.push(word_elem);
                },
                (Some('\''), _) => {
                    let word_elem = self.get_singly_quoted_word_elem(settings)?;
                    word_elems.push(word_elem);
                },
                (Some('"'), _) => {
                    let word_elem = self.get_doubly_quoted_word_elem(settings)?;
                    word_elems.push(word_elem);
                },
                (Some(c), pos) => {
                    self.unget_char(c, &pos, settings);
                    let (word_elem, _) = self.get_string_word_elem(settings)?;
                    word_elems.push(word_elem);
                }
            }
        }
        Ok(())
    }
    
    pub fn next_token(&mut self, settings: &Settings) -> ParserResult<(Token, Position)>
    {
        match self.pushed_tokens.pop() {
            Some((token, pos)) => Ok((token, pos)),
            None => {
                match &self.current_state {
                    State::InArithmeticExpression => {
                        panic!("current state is in arithmetic expression");
                    },
                    State::InArithmeticExpressionAndParentheses => {
                        panic!("current state is in arithmetic expression and parentheses");
                    },
                    State::HereDocumentWord => {
                        Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("not implemented"), false))                        
                    },
                    State::InHereDocument(_, _) => {
                        Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("not implemented"), false))
                    },
                    _ => {
                        self.skip_spaces(settings)?;
                        let token_pos = self.pos;
                        match self.get_char(settings)? {
                            (None, pos) => {
                                if self.current_state == State::InParameterExpansion || self.current_state == State::InCommandSubstitution {
                                    Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true))
                                } else {
                                    Ok((Token::EOF, token_pos))
                                }
                            },
                            (Some('}'), _) if self.current_state == State::InParameterExpansion => {
                                Ok((Token::EOF, token_pos))
                            },
                            (Some(')'), _) if self.current_state == State::InCommandSubstitution => {
                                Ok((Token::EOF, token_pos))
                            },
                            (Some('\n'), _) => Ok((Token::Newline, token_pos)),
                            (Some(';'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((Token::Semi, token_pos)),
                                    (Some(';'), _) => Ok((Token::SemiSemi, token_pos)),
                                    (Some(c), pos) => {
                                        self.unget_char(c, &pos, settings);
                                        Ok((Token::Semi, token_pos))
                                    },
                                }
                            },
                            (Some('<'), _) => self.get_less_token(None, &token_pos, settings),
                            (Some('>'), _) => self.get_greater_token(None, &token_pos, settings),
                            (Some('&'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((Token::Amp, token_pos)),
                                    (Some('&'), _) => Ok((Token::AmpAmp, token_pos)),
                                    (Some(c), pos) => {
                                        self.unget_char(c, &pos, settings);
                                        Ok((Token::Amp, token_pos))
                                    },
                                }
                            },
                            (Some('|'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((Token::Bar, token_pos)),
                                    (Some('|'), _) => Ok((Token::BarBar, token_pos)),
                                    (Some(c), pos) => {
                                        self.unget_char(c, &pos, settings);
                                        Ok((Token::Bar, token_pos))
                                    },
                                }
                            },
                            (Some('('), _) => Ok((Token::LParen, token_pos)),
                            (Some(')'), _) => Ok((Token::RParen, token_pos)),
                            (Some('$'), _) => {
                                let word_elem = self.get_dolar_word_elem(settings)?;
                                let mut word_elems = vec![word_elem];
                                self.read_word_elems(&mut word_elems, settings)?;
                                Ok((Token::Word(word_elems), token_pos))
                            },
                            (Some('`'), _) => {
                                let word_elem = self.get_backquote_word_elem(settings)?;
                                let mut word_elems = vec![word_elem];
                                self.read_word_elems(&mut word_elems, settings)?;
                                Ok((Token::Word(word_elems), token_pos))
                            },
                            (Some('\''), _) => {
                                let word_elem = self.get_singly_quoted_word_elem(settings)?;
                                let mut word_elems = vec![word_elem];
                                self.read_word_elems(&mut word_elems, settings)?;
                                Ok((Token::Word(word_elems), token_pos))
                            },
                            (Some('"'), _) => {
                                let word_elem = self.get_doubly_quoted_word_elem(settings)?;
                                let mut word_elems = vec![word_elem];
                                self.read_word_elems(&mut word_elems, settings)?;
                                Ok((Token::Word(word_elems), token_pos))
                            },
                            (Some(c), pos) => {
                                self.unget_char(c, &pos, settings);
                                let (word_elem, can_be_keyword) = self.get_string_word_elem(settings)?;
                                match (word_elem, can_be_keyword) {
                                    (WordElement::Simple(SimpleWordElement::String(s)), true) if is_number_str(s.as_str()) => {
                                        match s.parse::<i32>() {
                                            Ok(n) => {
                                                match self.get_less_or_greater_token(Some(n), &token_pos, settings)? {
                                                    Some((tmp_token @ (Token::LessLess | Token::LessLessMinus), tmp_pos)) => {
                                                        self.undo_token(&tmp_token, &tmp_pos);
                                                        let word_elems = vec![WordElement::Simple(SimpleWordElement::String(s))];
                                                        Ok((Token::Word(word_elems), token_pos))
                                                    },
                                                    Some((tmp_token, _)) => {
                                                        Ok((tmp_token, token_pos))
                                                    },
                                                    None => {
                                                        let mut word_elems = vec![WordElement::Simple(SimpleWordElement::String(s))];
                                                        self.read_word_elems(&mut word_elems, settings)?;
                                                        Ok((Token::Word(word_elems), token_pos))
                                                    },
                                                }
                                            },
                                            Err(_) => {
                                                match self.get_less_or_greater_token(None, &token_pos, settings)? {
                                                    Some((tmp_token @ (Token::LessLess | Token::LessLessMinus), tmp_pos)) => {
                                                        self.undo_token(&tmp_token, &tmp_pos);
                                                        let word_elems = vec![WordElement::Simple(SimpleWordElement::String(s))];
                                                        Ok((Token::Word(word_elems), token_pos))
                                                    },
                                                    Some((_, _)) => {
                                                        Err(ParserError::Syntax(self.path.clone(), token_pos, String::from("too large I/O number"), false))
                                                    },
                                                    None => {
                                                        let mut word_elems = vec![WordElement::Simple(SimpleWordElement::String(s))];
                                                        self.read_word_elems(&mut word_elems, settings)?;
                                                        Ok((Token::Word(word_elems), token_pos))
                                                    },
                                                }
                                            },
                                        }
                                    },
                                    (WordElement::Simple(SimpleWordElement::String(s)), can_be_keyword) => {
                                        let mut word_elems = vec![WordElement::Simple(SimpleWordElement::String(s.clone()))];
                                        self.read_word_elems(&mut word_elems, settings)?;
                                        match (&self.current_state, can_be_keyword, word_elems.len() == 1) {
                                            (State::FirstWord, true, true) => {
                                                match self.first_keywords.get(&s) {
                                                    Some(tmp_token) => Ok((tmp_token.clone(), token_pos)),
                                                    None => Ok((Token::Word(word_elems), token_pos)),
                                                }
                                            },
                                            (State::ThirdWord, true, true) => {
                                                match self.second_keywords.get(&s) {
                                                    Some(tmp_token) => Ok((tmp_token.clone(), token_pos)),
                                                    None => Ok((Token::Word(word_elems), token_pos)),
                                                }
                                            },
                                            _ => Ok((Token::Word(word_elems), token_pos)),
                                        }
                                    },
                                    (word_elem, _) => {
                                        let mut word_elems = vec![word_elem];
                                        self.read_word_elems(&mut word_elems, settings)?;
                                        Ok((Token::Word(word_elems), token_pos))
                                    }
                                }
                            },
                        }
                    },
                }
            },
        }
    }

    pub fn undo_token(&mut self, token: &Token, pos: &Position)
    { self.pushed_tokens.push((token.clone(), *pos)); }
    
    pub fn next_arith_token(&mut self, settings: &Settings) -> ParserResult<(ArithmeticToken, Position)>
    { Err(ParserError::Syntax(self.path.clone(), self.pos, String::from("not implemented"), false)) }

    pub fn undo_arith_token(&mut self, arith_token: &ArithmeticToken, pos: &Position)
    { self.pushed_arith_tokens.push((arith_token.clone(), *pos)); }
}

fn is_number_str(s: &str) -> bool
{ s.chars().all(|c| c >= '0' && c <= '9') }
