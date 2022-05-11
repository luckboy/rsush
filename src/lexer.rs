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
use std::io::*;
use std::rc::*;
use crate::io::*;
use crate::parser::*;

#[derive(Clone, PartialEq)]
enum State
{
    Initial,
    InParameterExpansion,
    InCommandSubstitution,
    InHereDocumentWord,
    InHereDocument(String),
    InFirstWord,
    InThirdWord,
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
    EOF,
}

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
    pushed_chars: Vec<char>,
    pushed_tokens: Vec<Token>,
    pushed_arith_tokens: Vec<ArithmeticToken>,
    state_stack: Vec<State>,
    current_state: State,
    position: Position,
}

impl<'a> Lexer<'a>
{
    pub fn new(path: &str, reader: &'a mut dyn CharRead) -> Lexer<'a>
    {
        Lexer {
            reader,
            pushed_chars: Vec::new(),
            pushed_tokens: Vec::new(),
            pushed_arith_tokens: Vec::new(),
            state_stack: Vec::new(),
            current_state: State::Initial,
            position: Position::new(path, 1, 1),
        }
    }

    pub fn next_token(&mut self) -> ParserResult<(Token, Position)>
    { Err(ParserError::Syntax(self.position.clone(), String::from("not implemented"), false)) }

    pub fn next_arith_token(&mut self) -> ParserResult<(ArithmeticToken, Position)>
    { Err(ParserError::Syntax(self.position.clone(), String::from("not implemented"), false)) }
}
