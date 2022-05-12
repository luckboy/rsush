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
use std::fmt;
use std::io::*;
use std::rc::*;
use std::result;
use crate::lexer::*;
use crate::settings::*;

#[derive(Clone)]
pub struct Word
{
    pub path: String,
    pub pos: Position,
    pub word_elems: Vec<WordElement>,
}

#[derive(Clone)]
pub struct Command
{
    pub path: String,
    pub pos: Position,
}

#[derive(Clone)]
pub struct ArithmeticExpression
{
    pub path: String,
    pub pos: Position,
}

pub struct Parser
{}

impl Parser
{
    pub fn new() -> Parser
    { Parser {} }
    
    pub fn parse_words<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Word>>>
    { Ok(Vec::new()) }

    pub fn parse_commands<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Command>>>
    { Ok(Vec::new()) }

    pub fn parse_arith_expr<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    { Ok(ArithmeticExpression { path: lexer.path().clone(), pos: lexer.pos(), }) }
}

#[derive(Copy, Clone)]
pub struct Position
{
    pub line: u64,
    pub column: u64,
}

impl Position
{
    pub fn new(line: u64, column: u64) -> Position
    { Position { line, column, } }
}

impl fmt::Display for Position
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}.{}", self.line, self.column) }
}

pub type ParserResult<T> = result::Result<T, ParserError>;

pub enum ParserError
{
    IO(String, Error),
    Syntax(String, Position, String, bool),
}

impl ParserError
{
    pub fn has_cont(&self) -> bool
    {
         match self {
             ParserError::IO(path, _) => false,
             ParserError::Syntax(_, _, _, is_cont) => *is_cont,
         }
    }
}

impl fmt::Display for ParserError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
         match self {
             ParserError::IO(path, err) => write!(f, "{}: {}", path, err),
             ParserError::Syntax(path, pos, msg, _) => write!(f, "{}: {}: {}", path, pos, msg),
         }
    }
}
