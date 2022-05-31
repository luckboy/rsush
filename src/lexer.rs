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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
    Command(Vec<Rc<LogicalCommand>>),
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
    LessLess(Option<i32>),
    LessLessMinus(Option<i32>),
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
    HereDoc(Vec<SimpleWordElement>, bool),
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
    backquote_column_inc: u64,
    content_for_verbose: String,
    has_ignored_eof: bool,
    first_keywords: HashMap<String, Token>,
    second_keywords: HashMap<String, Token>,
}

impl<'a> Lexer<'a>
{
    pub fn new(path: &str, pos: &Position, reader: &'a mut dyn CharRead, backquote_column_inc: u64, is_ignored_eof: bool) -> Lexer<'a>
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
        second_keywords.insert(String::from("do"), Token::Do);
        second_keywords.insert(String::from("in"), Token::In);
        Lexer {
            reader,
            pushed_chars: Vec::new(),
            pushed_tokens: Vec::new(),
            pushed_arith_tokens: Vec::new(),
            state_stack: Vec::new(),
            current_state: State::Initial,
            path: String::from(path),
            pos: *pos,
            backquote_column_inc,
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
    
    pub fn content_for_verbose(&self) -> String
    { self.content_for_verbose.clone() }

    pub fn clear_content_for_verbose(&mut self)
    { self.content_for_verbose.clear(); }

    pub fn push_initial(&mut self)
    { self.push_state(State::Initial); }

    pub fn push_here_doc_word(&mut self)
    { self.push_state(State::HereDocumentWord); }

    pub fn push_in_here_doc(&mut self, s: &str, is_minus: bool)
    { self.push_state(State::InHereDocument(String::from(s), is_minus)); }

    pub fn push_first_word(&mut self)
    { self.push_state(State::FirstWord); }    

    pub fn push_third_word(&mut self)
    { self.push_state(State::ThirdWord); }    

    pub fn push_in_arith_expr_and_paren(&mut self)
    { self.push_state(State::InArithmeticExpressionAndParentheses); }
    
    fn push_state(&mut self, state: State)
    {
        self.state_stack.push(self.current_state.clone());
        self.current_state = state.clone();
    }

    pub fn pop_state(&mut self)
    {
        match self.state_stack.pop() {
            Some(state) => self.current_state = state,
            None        => (),
        }
    }
    
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
                    if c == '`' {
                        self.pos.column += self.backquote_column_inc;
                    }
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
    
    fn skip_comment(&mut self, is_arith_expr: bool, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some(c @ '\n'), pos) => {
                    if !is_arith_expr {
                        self.unget_char(c, &pos, settings);
                    }
                    break;
                },
                (Some(_), _) => (),
            }
        }
        Ok(())
    }
    
    fn skip_spaces(&mut self, is_arith_expr: bool, settings: &Settings) -> ParserResult<()>
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
                (Some('#'), _) => self.skip_comment(is_arith_expr, settings)?,
                (Some(c @ '\n'), pos) if !is_arith_expr => {
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
                    (None, _) => Ok((Token::LessLess(n), *token_pos)),
                    (Some('-'), _) => Ok((Token::LessLessMinus(n), *token_pos)),
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        Ok((Token::LessLess(n), *token_pos))
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
    
    fn read_string_word(&mut self, s: &mut String, is_simple_word: bool, settings: &Settings) -> ParserResult<bool>
    {
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
                        (Some(c2), _) => {
                            if is_simple_word {
                                match c2 {
                                    '?' | '*' | '[' | ']' | ':' | '!' | '^' | '~' => s.push('\\'),
                                    _ => (),
                                }
                            }
                            s.push(c2);
                        },
                    }
                },
                (Some(c @ (';' | '<' | '>' | '&' | '|' | '(' | ')' | '\'' | '"' | '#')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) if c.is_whitespace() => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) => {
                    if is_simple_word {
                        if c == '$' || c == '`' || (c == '}' && self.current_state == State::InParameterExpansion) {
                            self.unget_char(c, &pos, settings);
                            break;
                        }
                    }
                    s.push(c)
                },
            }
        }
        Ok(can_be_keyword)
    }
    
    fn read_singly_quoted_word(&mut self, s: &mut String, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)), 
                (Some('\''), _) => break,
                (Some(c), _) => s.push(c),
            }
        }
        Ok(())
    }
    
    fn read_doubly_quoted_word(&mut self, s: &mut String, is_simple_word: bool, settings: &Settings) -> ParserResult<()>
    {
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
                (Some(c @ '"'), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) => {
                    if is_simple_word {
                        if c == '$' || c == '`' {
                            self.unget_char(c, &pos, settings);
                            break;
                        }
                    }
                    s.push(c);
                },
            }
        }
        Ok(())
    }
    
    fn get_here_doc_word(&mut self, token_pos: &Position, settings: &Settings) -> ParserResult<(Token, Position)>
    {
        let mut s = String::new();
        loop {
            match self.get_char(settings)? {
                (None, _) => break,
                (Some('\''), _) => self.read_singly_quoted_word(&mut s, settings)?,
                (Some('"'), _) => self.read_doubly_quoted_word(&mut s, false, settings)?, 
                (Some(c @ (';' | '<' | '>' | '&' | '|' | '(' | ')' | '#')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) if c.is_whitespace() => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) => {
                    self.unget_char(c, &pos, settings);
                    self.read_string_word(&mut s, false, settings)?;
                },
            }
        }
        Ok((Token::HereDocWord(s), *token_pos))
    }
    
    fn get_var_name(&mut self, c: char, settings: &Settings) -> ParserResult<ParameterName>
    {
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
        Ok(ParameterName::Variable(s))
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
                let param_name = self.get_var_name(c, settings)?;
                Ok(Some(param_name))
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
                    (Some('#'), _) => Ok(Some(ParameterModifier::HashHash)),
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        Ok(Some(ParameterModifier::Hash))
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
                                        self.push_state(State::InParameterExpansion);
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
                        self.push_state(State::InArithmeticExpression);
                        let arith_expr = parser.parse_arith_expr(self, settings)?;
                        self.pop_state();
                        Ok(SimpleWordElement::ArithmeticExpression(arith_expr))
                    },
                    (Some(c2), pos2) => {
                        self.unget_char(c2, &pos2, settings);
                        let mut parser = Parser::new();
                        self.push_state(State::InCommandSubstitution);
                        let commands = parser.parse_logical_commands(self, settings)?;
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
        let simple_word_elem_pos = self.pos;
        loop {
            match self.get_char(settings)? {
                (None, pos) => return Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                (Some(c @ '\\'), _) => {
                    match self.get_char(settings)? {
                        (None, pos2) => return Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                        (Some('`'), _) => s.push('`'),
                        (Some(c2), pos2) => {
                            self.unget_char(c2, &pos2, settings);
                            s.push(c);
                        },
                    }
                },
                (Some('`'), _) => break,
                (Some(c), _) => s.push(c),
            }
        }
        let mut cursor = Cursor::new(s.as_bytes());
        let mut cr = CharReader::new(&mut cursor);
        let mut lexer = Lexer::new(self.path.as_str(), &simple_word_elem_pos, &mut cr, self.backquote_column_inc + 1, false);
        let mut parser = Parser::new();
        let commands = parser.parse_logical_commands(&mut lexer, settings)?;
        Ok(SimpleWordElement::Command(commands))
    }
    
    fn get_string_simple_word_elem_for_word_elem(&mut self, settings: &Settings) -> ParserResult<(SimpleWordElement, bool)>
    {
        let mut s = String::new();
        let can_be_keyword = self.read_string_word(&mut s, true, settings)?;
        Ok((SimpleWordElement::String(s), can_be_keyword))
    }
    
    fn get_string_simple_word_elem_for_doubly_quoted(&mut self, settings: &Settings) -> ParserResult<SimpleWordElement>
    {
        let mut s = String::new();
        self.read_doubly_quoted_word(&mut s, true, settings)?;
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
        self.read_singly_quoted_word(&mut s, settings)?;
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
                (Some(c @ (';' | '<' | '>' | '&' | '|' | '(' | ')' | '#')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), pos) if c.is_whitespace() => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c @ '}'), pos) if self.current_state == State::InParameterExpansion => {
                    self.unget_char(c, &pos, settings);
                    break;
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
    
    fn get_string_simple_word_elem_for_here_doc(&mut self, settings: &Settings) -> ParserResult<(SimpleWordElement, bool)>
    {
        let mut s = String::new();
        let mut is_newline = false;
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
                (Some('\n'), _) => {
                    s.push('\n');
                    is_newline = true;
                    break;
                },
                (Some(c @ ('$' | '`')), pos) => {
                    self.unget_char(c, &pos, settings);
                    break;
                },
                (Some(c), _) => s.push(c),
            }
        }
        Ok((SimpleWordElement::String(s), is_newline))
    }
    
    fn read_simple_word_elems_for_here_doc(&mut self, simple_word_elems: &mut Vec<SimpleWordElement>, settings: &Settings) -> ParserResult<()>
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
                (Some(c), pos) => {
                    self.unget_char(c, &pos, settings);
                    let (simple_word_elem, is_newline) = self.get_string_simple_word_elem_for_here_doc(settings)?;
                    simple_word_elems.push(simple_word_elem);
                    if is_newline {
                        break;
                    }
                },
            }
        }
        Ok(())
    }
    
    fn get_here_doc(&mut self, delim: &str, is_minus: bool, token_pos: &Position, settings: &Settings) -> ParserResult<(Token, Position)>
    {
        let mut simple_word_elems: Vec<SimpleWordElement> = Vec::new();
        loop {
            if is_minus {
                loop {
                    match self.get_char(settings)? {
                        (None, _) => break,
                        (Some('\t'), _) => (),
                        (Some(c), pos) => {
                            self.unget_char(c, &pos, settings);
                            break;
                        },
                    }
                }
            }
            let mut line = String::new();
            let mut chars_with_poses: Vec<(char, Position)> = Vec::new();
            let mut is_eof = false;
            let mut eof_pos = self.pos;
            loop {
                match self.get_char(settings)? {
                    (None, pos) => {
                        is_eof = true;
                        eof_pos = pos;
                        break;
                    },
                    (Some('\n'), pos) => {
                        chars_with_poses.push(('\n', pos));
                        break;
                    },
                    (Some(c), pos) => {
                        line.push(c);
                        chars_with_poses.push((c, pos));
                    },
                }
            }
            if line == String::from(delim) {
                break;
            }
            if is_eof {
                return Err(ParserError::Syntax(self.path.clone(), eof_pos, String::from("unexpected end of file"), true));
            }
            chars_with_poses.reverse();
            for (c, pos) in &chars_with_poses {
                self.unget_char(*c, pos, settings);
            }
            self.read_simple_word_elems_for_here_doc(&mut simple_word_elems, settings)?;
        }
        Ok((Token::HereDoc(simple_word_elems, is_minus), *token_pos))
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
                        self.skip_spaces(false, settings)?;
                        let token_pos = self.pos;
                        match self.get_char(settings)? {
                            (None, pos) => Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                            (Some(';' | '<' | '>' | '&' | '|' | '(' | ')' | '#'), pos) => {
                                Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected character"), false))
                            },
                            (Some(c), pos) => {
                                self.unget_char(c, &pos, settings);
                                self.get_here_doc_word(&token_pos, settings)
                            },
                        }
                    },
                    State::InHereDocument(delim_r, is_minus_r) => {
                        let token_pos = self.pos;
                        let delim = delim_r.clone();
                        let is_minus = *is_minus_r;
                        self.get_here_doc(delim.as_str(), is_minus, &token_pos, settings)
                    },
                    _ => {
                        self.skip_spaces(false, settings)?;
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
                            (Some(')'), _) if self.current_state == State::InCommandSubstitution || ((self.current_state == State::FirstWord || self.current_state == State::ThirdWord) && self.state_stack.last().map(|s| s == &State::InCommandSubstitution).unwrap_or(false)) => {
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
    {
        match self.pushed_arith_tokens.pop() {
            Some((arith_token, pos)) => Ok((arith_token, pos)),
            None => {
                match &self.current_state {
                    State::InArithmeticExpression | State::InArithmeticExpressionAndParentheses => {
                        self.skip_spaces(true, settings)?;
                        let arith_token_pos = self.pos;
                        match self.get_char(settings)? {
                            (None, pos) => Err(ParserError::Syntax(self.path.clone(), pos, String::from("unexpected end of file"), true)),
                            (Some('('), _) => Ok((ArithmeticToken::LParen, arith_token_pos)),
                            (Some(')'), _) => {
                                if self.current_state == State::InArithmeticExpression {
                                    match self.get_char(settings)? {
                                        (None, pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected end of file"), true)),
                                        (Some(')'), _) => Ok((ArithmeticToken::RParen, arith_token_pos)),
                                        (Some(_), pos2) => Err(ParserError::Syntax(self.path.clone(), pos2, String::from("unexpected character"), false)),
                                    }
                                } else {
                                    Ok((ArithmeticToken::RParen, arith_token_pos))
                                }
                            },
                            (Some('~'), _) => Ok((ArithmeticToken::Tylda, arith_token_pos)),
                            (Some('!'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Excl, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::ExclEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Excl, arith_token_pos))
                                    },
                                }
                            },
                            (Some('*'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Star, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::StarEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Star, arith_token_pos))
                                    },
                                }
                            },
                            (Some('/'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Slash, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::SlashEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Slash, arith_token_pos))
                                    },
                                }
                            },
                            (Some('%'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Perc, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::PercEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Perc, arith_token_pos))
                                    },
                                }
                            },
                            (Some('+'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Plus, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::PlusEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Plus, arith_token_pos))
                                    },
                                }
                            },
                            (Some('-'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Minus, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::MinusEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Minus, arith_token_pos))
                                    },
                                }
                            },
                            (Some('<'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Less, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::LessEqual, arith_token_pos)),
                                    (Some('<'), _) => {
                                        match self.get_char(settings)? {
                                            (None, _) => Ok((ArithmeticToken::LessLess, arith_token_pos)),
                                            (Some('='), _) => Ok((ArithmeticToken::LessLessEqual, arith_token_pos)),
                                            (Some(c3), pos3) => {
                                                self.unget_char(c3, &pos3, settings);
                                                Ok((ArithmeticToken::LessLess, arith_token_pos))
                                            },
                                        }
                                    },
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Less, arith_token_pos))
                                    },
                                }
                            },
                            (Some('>'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Greater, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::GreaterEqual, arith_token_pos)),
                                    (Some('>'), _) => {
                                        match self.get_char(settings)? {
                                            (None, _) => Ok((ArithmeticToken::GreaterGreater, arith_token_pos)),
                                            (Some('='), _) => Ok((ArithmeticToken::GreaterGreaterEqual, arith_token_pos)),
                                            (Some(c3), pos3) => {
                                                self.unget_char(c3, &pos3, settings);
                                                Ok((ArithmeticToken::GreaterGreater, arith_token_pos))
                                            },
                                        }
                                    },
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Greater, arith_token_pos))
                                    },
                                }
                            },
                            (Some('='), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Equal, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::EqualEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Equal, arith_token_pos))
                                    },
                                }
                            },
                            (Some('&'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Amp, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::AmpEqual, arith_token_pos)),
                                    (Some('&'), _) => Ok((ArithmeticToken::AmpAmp, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Amp, arith_token_pos))
                                    },
                                }
                            },
                            (Some('^'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Caret, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::CaretEqual, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Caret, arith_token_pos))
                                    },
                                }
                            },
                            (Some('|'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Bar, arith_token_pos)),
                                    (Some('='), _) => Ok((ArithmeticToken::BarEqual, arith_token_pos)),
                                    (Some('|'), _) => Ok((ArithmeticToken::BarBar, arith_token_pos)),
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Bar, arith_token_pos))
                                    },
                                }
                            },
                            (Some('?'), _) => Ok((ArithmeticToken::Ques, arith_token_pos)),
                            (Some(':'), _) => Ok((ArithmeticToken::Colon, arith_token_pos)),
                            (Some('0'), _) => {
                                match self.get_char(settings)? {
                                    (None, _) => Ok((ArithmeticToken::Number(0), arith_token_pos)),
                                    (Some('X' | 'x'), _) => {
                                        let mut s = String::new();
                                        loop {
                                            match self.get_char(settings)? {
                                                (None, _) => break,
                                                (Some(c3 @ (('0'..='9') | ('A'..='F') | ('a'..='f'))), _) => s.push(c3),
                                                (Some(c3), pos3) => {
                                                    self.unget_char(c3, &pos3, settings);
                                                    break;
                                                },
                                            }
                                        }
                                        if !s.is_empty() {
                                            match i64::from_str_radix(s.as_str(), 16) {
                                                Ok(n) => Ok((ArithmeticToken::Number(n), arith_token_pos)),
                                                Err(_) => Err(ParserError::Syntax(self.path.clone(), arith_token_pos, String::from("too large number"), false)),
                                            }
                                        } else {
                                            Err(ParserError::Syntax(self.path.clone(), arith_token_pos, String::from("no hexadecimal digits"), false))
                                        }
                                    },
                                    (Some(c2 @ ('0'..='7')), _) => {
                                        let mut s = String::new();
                                        s.push(c2);
                                        loop {
                                            match self.get_char(settings)? {
                                                (None, _) => break,
                                                (Some(c3 @ ('0'..='7')), _) => s.push(c3),
                                                (Some(c3), pos3) => {
                                                    self.unget_char(c3, &pos3, settings);
                                                    break;
                                                },
                                            }
                                        }
                                        match i64::from_str_radix(s.as_str(), 8) {
                                            Ok(n) => Ok((ArithmeticToken::Number(n), arith_token_pos)),
                                            Err(_) => Err(ParserError::Syntax(self.path.clone(), arith_token_pos, String::from("too large number"), false)),
                                        }
                                    },
                                    (Some(c2), pos2) => {
                                        self.unget_char(c2, &pos2, settings);
                                        Ok((ArithmeticToken::Number(0), arith_token_pos))
                                    },
                                }
                            },
                            (Some(c @ ('1'..='9')), _) => {
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
                                match s.parse::<i64>() {
                                    Ok(n) => Ok((ArithmeticToken::Number(n), arith_token_pos)),
                                    Err(_) => Err(ParserError::Syntax(self.path.clone(), arith_token_pos, String::from("too large number"), false)),
                                }
                            },
                            (Some('$'), _) => {
                                match self.get_param_name(settings)? {
                                    Some(param_name) => Ok((ArithmeticToken::Parameter(param_name), arith_token_pos)),
                                    None => Err(ParserError::Syntax(self.path.clone(), arith_token_pos, String::from("no parameter name"), false)),
                                }
                            },
                            (Some(c), _) if c.is_alphabetic() || c == '_' => {
                                let param_name = self.get_var_name(c, settings)?;
                                Ok((ArithmeticToken::Parameter(param_name), arith_token_pos))
                            },
                            (Some(_), pos) => Err(ParserError::Syntax(self.path.clone(), pos, String::from("invalid character"), false)),
                        }
                    },
                    _ => {
                        panic!("current state isn't in arithmetic expression or in arithmetic expression and parentheses");
                    },
                }
            },
        }
    }

    pub fn undo_arith_token(&mut self, arith_token: &ArithmeticToken, pos: &Position)
    { self.pushed_arith_tokens.push((arith_token.clone(), *pos)); }
}

fn is_number_str(s: &str) -> bool
{ s.chars().all(|c| c >= '0' && c <= '9') }

#[cfg(test)]
mod tests
{
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
}
