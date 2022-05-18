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
use std::cell::*;
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
pub enum Redirect
{
    Input(String, Position, Rc<Word>),
    Output(String, Position, Rc<Word>, bool),
    InputAndOuput(String, Position, Rc<Word>),
    Appending(String, Position, Rc<Word>),
    InputDuplicating(String, Position, Rc<Word>),
    OutputDuplicating(String, Position, Rc<Word>),
    HereDocument(String, Position, Rc<RefCell<Vec<SimpleWordElement>>>),
}

#[derive(Clone)]
pub struct SimpleCommand
{
    words: Vec<Rc<Word>>,
    redirects: Vec<Rc<Redirect>>,
}

#[derive(Clone)]
pub struct Case
{
    pub pattern: Vec<Rc<Word>>,
    pub commands: Vec<Rc<LogicalCommand>>,
}

#[derive(Clone)]
pub struct ElifPair
{
    cond_commands: Vec<Rc<LogicalCommand>>,
    commands: Vec<Rc<LogicalCommand>>,
}

#[derive(Clone)]
pub enum CompoundCommand
{
    BraceGroup(Vec<Rc<LogicalCommand>>),
    Subshell(Vec<Rc<LogicalCommand>>),
    For(Rc<Word>, Vec<Rc<Word>>, Vec<Rc<LogicalCommand>>),
    Case(Rc<Word>, Vec<Case>),
    If(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>, Vec<ElifPair>, Option<Vec<Rc<LogicalCommand>>>),
    While(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>),
    Until(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>),
}

#[derive(Clone)]
pub enum Command
{
    Simple(String, Position, SimpleCommand),
    Compound(String, Position, CompoundCommand, Option<Vec<Rc<Redirect>>>),
    FunctionDefinition(String, Position, Rc<Word>, CompoundCommand, Option<Vec<Rc<Redirect>>>),
}

#[derive(Clone)]
pub struct PipeCommand
{
    pub path: String,
    pub pos: Position,
    pub is_negative: bool,
    pub commands: Vec<Rc<Command>>,
}

#[derive(Copy, Clone)]
pub enum LogicalOperator
{
    And,
    Or,
}

#[derive(Clone)]
pub struct LogicalPair
{
    pub op: LogicalOperator,
    pub command: Rc<PipeCommand>,
}

#[derive(Clone)]
pub struct LogicalCommand
{
    pub path: String,
    pub pos: Position,
    pub first: Rc<PipeCommand>,
    pub pairs: Vec<LogicalPair>,
    pub is_in_background: bool,
}

#[derive(Copy, Clone)]
pub enum UnaryOperator
{
    Negate,
    Not,
    LogicalNot,
}

#[derive(Copy, Clone)]
pub enum BinaryOperator
{
    Multiply,
    Divide,
    Module,
    Add,
    Substract,
    ShiftLeft,
    ShiftRight,
    LessThan,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    Equal,
    NotEqual,
    And,
    ExlusiveOr,
    Or,
    LogicalAnd,
    LogicalOr,
    Assign,
    MultiplyAssign,
    DivideAssign,
    ModuleAssign,
    AddAssign,
    SubstractAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    AndAssign,
    ExlusiveOrAssign,
    OrAssign,
}

#[derive(Clone)]
pub enum ArithmeticExpression
{
    Number(String, Position, i64),
    Parameter(String, Position, ParameterName),
    Unary(String, Position, UnaryOperator, Rc<ArithmeticExpression>),
    Binary(String, Position, Rc<ArithmeticExpression>, BinaryOperator, Rc<ArithmeticExpression>),
    Conditional(String, Position, Rc<ArithmeticExpression>, Rc<ArithmeticExpression>, Rc<ArithmeticExpression>),
}

pub struct Parser
{}

impl Parser
{
    pub fn new() -> Parser
    { Parser {} }
    
    pub fn parse_words<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Word>>>
    { Ok(Vec::new()) }

    pub fn parse_logical_commands<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<LogicalCommand>>>
    { Ok(Vec::new()) }

    pub fn parse_arith_expr<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    { Ok(ArithmeticExpression::Number(lexer.path().clone(), lexer.pos(), 0)) }
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
             ParserError::IO(_, _) => false,
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
