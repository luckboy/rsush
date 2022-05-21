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
pub struct HereDocument
{
    pub delim: String,
    pub has_minus: bool,
    pub simple_word_elems: Vec<SimpleWordElement>,
    
}

#[derive(Clone)]
pub enum Redirection
{
    Input(String, Position, Option<i32>, Rc<Word>),
    Output(String, Position, Option<i32>, Rc<Word>, bool),
    InputAndOutput(String, Position, Option<i32>, Rc<Word>),
    Appending(String, Position, Option<i32>, Rc<Word>),
    InputDuplicating(String, Position, Option<i32>, Rc<Word>),
    OutputDuplicating(String, Position, Option<i32>, Rc<Word>),
    HereDocument(String, Position, Option<i32>, Rc<RefCell<HereDocument>>),
}

impl Redirection
{
    pub fn path(&self) -> String
    {
        match self {
            Redirection::Input(path, _, _, _) => path.clone(),
            Redirection::Output(path, _, _, _, _) => path.clone(),
            Redirection::InputAndOutput(path, _, _, _) => path.clone(),
            Redirection::Appending(path, _, _, _) => path.clone(),
            Redirection::InputDuplicating(path, _, _, _) => path.clone(),
            Redirection::OutputDuplicating(path, _, _, _) => path.clone(),
            Redirection::HereDocument(path, _, _, _) => path.clone(),
        }
    }

    pub fn pos(&self) -> Position
    {
        match self {
            Redirection::Input(_, pos, _, _) => *pos,
            Redirection::Output(_, pos, _, _, _) => *pos,
            Redirection::InputAndOutput(_, pos, _, _) => *pos,
            Redirection::Appending(_, pos, _, _) => *pos,
            Redirection::InputDuplicating(_, pos, _, _) => *pos,
            Redirection::OutputDuplicating(_, pos, _, _) => *pos,
            Redirection::HereDocument(_, pos, _, _) => *pos,
        }
    }
}

#[derive(Clone)]
pub struct SimpleCommand
{
    words: Vec<Rc<Word>>,
    redirects: Vec<Rc<Redirection>>,
}

#[derive(Clone)]
pub struct CasePair
{
    pub pattern_words: Vec<Rc<Word>>,
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
    Case(Rc<Word>, Vec<CasePair>),
    If(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>, Vec<ElifPair>, Option<Vec<Rc<LogicalCommand>>>),
    While(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>),
    Until(Vec<Rc<LogicalCommand>>, Vec<Rc<LogicalCommand>>),
}

#[derive(Clone)]
pub struct FunctionBody
{
    path: String,
    pos: Position,
    command: CompoundCommand,
    redirects: Vec<Rc<Redirection>>,
}

#[derive(Clone)]
pub enum Command
{
    Simple(String, Position, SimpleCommand),
    Compound(String, Position, CompoundCommand, Vec<Rc<Redirection>>),
    FunctionDefinition(String, Position, Rc<Word>, Rc<FunctionBody>),
}

impl Command
{
    pub fn path(&self) -> String
    {
        match self {
            Command::Simple(path, _, _) => path.clone(),
            Command::Compound(path, _, _, _) => path.clone(),
            Command::FunctionDefinition(path, _, _, _) => path.clone(),
        }
    }

    pub fn pos(&self) -> Position
    {
        match self {
            Command::Simple(_, pos, _) => *pos,
            Command::Compound(_, pos, _, _) => *pos,
            Command::FunctionDefinition(_, pos, _, _) => *pos,
        }
    }
}

#[derive(Clone)]
pub struct PipeCommand
{
    pub path: String,
    pub pos: Position,
    pub is_negative: bool,
    pub commands: Vec<Rc<Command>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    pub first_command: Rc<PipeCommand>,
    pub pairs: Vec<LogicalPair>,
    pub is_in_background: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator
{
    Negate,
    Not,
    LogicalNot,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    ExclusiveOr,
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
    ExclusiveOrAssign,
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

impl ArithmeticExpression
{
    pub fn path(&self) -> String
    {
        match self {
            ArithmeticExpression::Number(path, _, _) => path.clone(),
            ArithmeticExpression::Parameter(path, _, _) => path.clone(),
            ArithmeticExpression::Unary(path, _, _, _) => path.clone(),
            ArithmeticExpression::Binary(path, _, _, _, _) => path.clone(),
            ArithmeticExpression::Conditional(path, _, _, _, _) => path.clone(),
        }
    }

    pub fn pos(&self) -> Position
    {
        match self {
            ArithmeticExpression::Number(_, pos, _) => *pos,
            ArithmeticExpression::Parameter(_, pos, _) => *pos,
            ArithmeticExpression::Unary(_, pos, _, _) => *pos,
            ArithmeticExpression::Binary(_, pos, _, _, _) => *pos,
            ArithmeticExpression::Conditional(_, pos, _, _, _) => *pos,
        }
    }
}

pub struct Parser
{
    here_docs: Vec<Rc<RefCell<HereDocument>>>,
    has_first_word_or_third_word: bool,
}

impl Parser
{
    pub fn new() -> Parser
    { Parser { here_docs: Vec::new(), has_first_word_or_third_word: false } }

    fn parse_words_without_last_token<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Word>>>
    {
        let mut words: Vec<Rc<Word>> = Vec::new();
        loop {
            match lexer.next_token(settings)? {
                (Token::Word(word_elems), pos) => {
                    let word = Word {
                        path: lexer.path().clone(),
                        pos,
                        word_elems,
                    };
                    words.push(Rc::new(word));
                },
                (token, pos) => {
                    lexer.undo_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(words)
    }    

    pub fn parse_words<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Word>>>
    {
        let words = self.parse_words_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::EOF, _) => (),
            (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
        Ok(words)
    }
    
    fn parse_here_docs<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<()>
    {
        for here_doc in &self.here_docs {
            let mut here_doc = here_doc.borrow_mut();
            lexer.push_in_here_doc(here_doc.delim.as_str(), here_doc.has_minus);
            match lexer.next_token(settings)? {
                (Token::HereDoc(simple_word_elems, _), _) => here_doc.simple_word_elems = simple_word_elems,
                (_, _) => panic!("token isn't here document"), 
            }
            lexer.pop_state();
        }
        self.here_docs.clear();
        Ok(())
    }

    fn skip_newlines<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<()>
    {
        loop {
            match lexer.next_token(settings)? {
                (Token::Newline, _) => self.parse_here_docs(lexer, settings)?,
                (token, pos) => {
                    lexer.undo_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(())
    }

    fn parse_redirect_word<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Word>
    {
        match lexer.next_token(settings)? {
            (Token::Word(word_elems), pos) => {
                let word = Word {
                    path: lexer.path().clone(),
                    pos,
                    word_elems: word_elems.clone(),
                };
                Ok(word)
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_here_doc_word<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<String>
    {
        lexer.push_here_doc_word();
        match lexer.next_token(settings)? {
            (Token::HereDocWord(s), _) => {
                lexer.pop_state();
                Ok(s)
            },
            (Token::EOF, pos) => {
                lexer.pop_state();
                Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true))
            },
            (_, pos) => {
                lexer.pop_state();
                Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false))
            },
        }
    }
    
    fn parse_redirect<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<Redirection>>
    {
        match lexer.next_token(settings)? {
            (Token::Less(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::Input(lexer.path().clone(), pos, n, Rc::new(word))))
            },
            (Token::Greater(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::Output(lexer.path().clone(), pos, n, Rc::new(word), false)))
            },
            (Token::LessLess(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let s = self.parse_here_doc_word(lexer, settings)?;
                let here_doc = HereDocument {
                    delim: s,
                    has_minus: false,
                    simple_word_elems: Vec::new(),
                };
                let here_doc = Rc::new(RefCell::new(here_doc));
                self.here_docs.push(here_doc.clone());
                Ok(Some(Redirection::HereDocument(lexer.path().clone(), pos, n, here_doc.clone())))
            },
            (Token::LessLessMinus(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let s = self.parse_here_doc_word(lexer, settings)?;
                let here_doc = HereDocument {
                    delim: s,
                    has_minus: true,
                    simple_word_elems: Vec::new(),
                };
                let here_doc = Rc::new(RefCell::new(here_doc));
                self.here_docs.push(here_doc.clone());
                Ok(Some(Redirection::HereDocument(lexer.path().clone(), pos, n, here_doc.clone())))
            },
            (Token::LessGreater(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::InputAndOutput(lexer.path().clone(), pos, n, Rc::new(word))))
            },
            (Token::LessAmp(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::InputDuplicating(lexer.path().clone(), pos, n, Rc::new(word))))
            },
            (Token::GreaterGreater(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::Appending(lexer.path().clone(), pos, n, Rc::new(word))))
            },
            (Token::GreaterAmp(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::OutputDuplicating(lexer.path().clone(), pos, n, Rc::new(word))))
            },
            (Token::GreaterBar(n), pos) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                let word = self.parse_redirect_word(lexer, settings)?;
                Ok(Some(Redirection::Output(lexer.path().clone(), pos, n, Rc::new(word), true)))
            },
            (token, pos) => {
                lexer.undo_token(&token, &pos);
                Ok(None)
            },
        }
    }

    fn parse_redirects<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Redirection>>>
    {
        let mut redirects: Vec<Rc<Redirection>> = Vec::new();
        loop {
            match self.parse_redirect(lexer, settings)? {
                Some(redirect) => redirects.push(Rc::new(redirect.clone())),
                None => break,
            }
        }
        Ok(redirects)
    }
    
    fn parse_do_clause<'a>(&mut self, lexer: &mut Lexer<'a>, is_do_word: bool, settings: &Settings) -> ParserResult<Vec<Rc<LogicalCommand>>>
    {
        if !is_do_word {
            match lexer.next_token(settings)? {
                (Token::Do, _) => (),
                (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
            }
        }
        lexer.pop_state();
        self.has_first_word_or_third_word = false;
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::Done, _) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                Ok(commands)
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_pattern_words<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<Word>>>
    {
        let mut words: Vec<Rc<Word>> = Vec::new();
        match lexer.next_token(settings)? {
            (Token::Word(word_elems), pos) => {
                let first_word = Word {
                    path: lexer.path().clone(),
                    pos,
                    word_elems: word_elems.clone(),
                };
                words.push(Rc::new(first_word));
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                loop {
                    match lexer.next_token(settings)? {
                        (Token::Bar, _) => (),
                        (token, pos) => {
                            lexer.undo_token(&token, &pos);
                            break;
                        },
                    }
                    match lexer.next_token(settings)? {
                        (Token::Word(word_elems), pos) => {
                            let word = Word {
                                path: lexer.path().clone(),
                                pos,
                                word_elems: word_elems.clone(),
                            };
                            words.push(Rc::new(word));
                        },
                        (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                        (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                    }
                }
                Ok(words)
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }
    
    fn parse_brace_group<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        lexer.push_initial();
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::RBrace, _) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                lexer.pop_state();
                Ok(CompoundCommand::BraceGroup(commands))
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_subshell<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    { 
        lexer.push_initial();
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::RParen, _) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                lexer.pop_state();
                Ok(CompoundCommand::Subshell(commands))
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_for_clause<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        match lexer.next_token(settings)? {
            (Token::Word(word_elems), pos) => {
                let word = Word {
                    path: lexer.path().clone(),
                    pos,
                    word_elems,
                };
                lexer.push_third_word();
                self.has_first_word_or_third_word = true;
                match lexer.next_token(settings)? {
                    (Token::In, _) => {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                        let words = self.parse_words_without_last_token(lexer, settings)?;
                        match lexer.next_token(settings)? {
                            (token @ (Token::Newline | Token::Semi), _) => {
                                match token {
                                    Token::Newline => self.parse_here_docs(lexer, settings)?,
                                    _ => (),
                                }
                                lexer.push_first_word();
                                self.has_first_word_or_third_word = true;
                                self.skip_newlines(lexer, settings)?;
                                let commands = self.parse_do_clause(lexer, false, settings)?;
                                Ok(CompoundCommand::For(Rc::new(word), words, commands))
                            },
                            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                        }
                    },
                    (Token::Do, _) => {
                        let commands = self.parse_do_clause(lexer, true, settings)?;
                        Ok(CompoundCommand::For(Rc::new(word), Vec::new(), commands))
                    },
                    (token @ (Token::Newline | Token::Semi), _) => {
                        match token {
                            Token::Newline => self.parse_here_docs(lexer, settings)?,
                            _ => (),
                        }
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                        lexer.push_first_word();
                        self.has_first_word_or_third_word = true;
                        self.skip_newlines(lexer, settings)?;
                        let commands = self.parse_do_clause(lexer, false, settings)?;
                        Ok(CompoundCommand::For(Rc::new(word), Vec::new(), commands))
                    },
                    (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                    (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                }
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_case_clause<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        match lexer.next_token(settings)? {
            (Token::Word(word_elems), pos) => {
                let word = Word {
                    path: lexer.path().clone(),
                    pos,
                    word_elems,
                };
                lexer.push_third_word();
                self.has_first_word_or_third_word = true;
                match lexer.next_token(settings)? {
                    (Token::In, _) => {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                        lexer.push_initial();
                        let mut pairs: Vec<CasePair> = Vec::new();
                        loop {
                            self.skip_newlines(lexer, settings)?;
                            lexer.push_first_word();
                            self.has_first_word_or_third_word = true;
                            match lexer.next_token(settings)? {
                                (Token::LParen, _) => {
                                    lexer.pop_state();
                                    self.has_first_word_or_third_word = false;
                                },
                                (Token::Esac, _) => {
                                    lexer.pop_state();
                                    self.has_first_word_or_third_word = false;
                                    break;
                                },
                                (token, pos) => {
                                    lexer.undo_token(&token, &pos);
                                },
                            }
                            let pattern_words = self.parse_pattern_words(lexer, settings)?;
                            match lexer.next_token(settings)? {
                                (Token::RParen, _) => (),
                                (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                                (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                            }
                            lexer.push_first_word();
                            self.has_first_word_or_third_word = true;
                            let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
                            match lexer.next_token(settings)? {
                                (Token::SemiSemi, _) => {
                                    if self.has_first_word_or_third_word {
                                        lexer.pop_state();
                                        self.has_first_word_or_third_word = false;
                                    }
                                },
                                (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                                (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                            }
                            let pair = CasePair {
                                pattern_words,
                                commands,
                            };
                            pairs.push(pair);
                        }
                        lexer.pop_state();
                        Ok(CompoundCommand::Case(Rc::new(word), pairs))
                    },
                    (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                    (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                }
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_if_clause<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let cond_commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::Then, _) => (),
            (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
        let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        let mut pairs: Vec<ElifPair> = Vec::new();
        loop {
            match lexer.next_token(settings)? {
                (Token::Elif, _) => (),
                (token, pos) => {
                    lexer.undo_token(&token, &pos);
                    break;
                },
            }
            let cond_commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
            match lexer.next_token(settings)? {
                (Token::Then, _) => (),
                (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
            }
            let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
            let pair = ElifPair {
                cond_commands,
                commands,
            };
            pairs.push(pair);
        }
        match lexer.next_token(settings)? {
            (Token::Fi, _) => {
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                Ok(CompoundCommand::If(cond_commands, commands, pairs, None))
            },
            (Token::Else, _) => {
                let commands2 = self.parse_logical_commands_without_last_token(lexer, settings)?;
                match lexer.next_token(settings)? {
                    (Token::Fi, _) => (),
                    (Token::EOF, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
                    (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                }
                if self.has_first_word_or_third_word {
                    lexer.pop_state();
                    self.has_first_word_or_third_word = false;
                }
                Ok(CompoundCommand::If(cond_commands, commands, pairs, Some(commands2)))
            },
            (Token::EOF, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), true)),
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
    }

    fn parse_while_clause<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let cond_commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        let commands = self.parse_do_clause(lexer, false, settings)?;
        Ok(CompoundCommand::While(cond_commands, commands))
    }

    fn parse_until_clause<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<CompoundCommand>
    {
        lexer.push_first_word();
        self.has_first_word_or_third_word = true;
        let cond_commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        let commands = self.parse_do_clause(lexer, false, settings)?;
        Ok(CompoundCommand::Until(cond_commands, commands))
    }

    fn parse_compound_command<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<(CompoundCommand, Position)>>
    {
        match lexer.next_token(settings)? {
            (Token::LBrace, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_brace_group(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::LParen, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_subshell(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::For, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_for_clause(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::Case, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_case_clause(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::If, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_if_clause(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::While, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_while_clause(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (Token::Until, pos) => {
                lexer.pop_state();
                self.has_first_word_or_third_word = false;
                let compound_command = self.parse_until_clause(lexer, settings)?;
                Ok(Some((compound_command, pos)))
            },
            (token, pos) => {
                lexer.undo_token(&token, &pos);
                Ok(None)
            },
        }
    }
    
    fn parse_compound_command_and_redirects<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<(CompoundCommand, Vec<Rc<Redirection>>, Position)>>
    {
        match self.parse_compound_command(lexer, settings)? {
            Some((compound_command, pos)) => {
                let redirects = self.parse_redirects(lexer, settings)?;
                Ok(Some((compound_command, redirects, pos)))
            },
            None => Ok(None),
        }
    }

    fn parse_simple_command<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<(SimpleCommand, Position)>>
    {
        let mut words: Vec<Rc<Word>> = Vec::new();
        let mut redirects: Vec<Rc<Redirection>> = Vec::new();
        let mut is_first = true;
        let mut first_pos = lexer.pos();
        loop {
            match lexer.next_token(settings)? {
                (Token::Word(word_elems), pos) => {
                    if self.has_first_word_or_third_word {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                    }
                    let word = Word {
                        path: lexer.path().clone(),
                        pos,
                        word_elems: word_elems.clone(),
                    };
                    words.push(Rc::new(word.clone()));
                    is_first = false;
                    first_pos = word.pos;
                }
                (token, pos) => {
                    lexer.undo_token(&token, &pos);
                    match self.parse_redirect(lexer, settings)? {
                        Some(redirect) => {
                            redirects.push(Rc::new(redirect.clone()));
                            is_first = false;
                            first_pos = redirect.pos();
                        },
                        None => break,
                    }
                },
            }
        }
        if !is_first {
            let simple_command = SimpleCommand {
                words,
                redirects,
            };
            Ok(Some((simple_command, first_pos)))
        } else {
            Ok(None)
        }
    }

    fn parse_fun_body<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<FunctionBody>>
    {
        match self.parse_compound_command_and_redirects(lexer, settings)? {
            Some((command, redirects, pos)) => {
                let fun_body = FunctionBody {
                    path: lexer.path().clone(),
                    pos,
                    command,
                    redirects,
                };
                Ok(Some(fun_body))
            },
            None => Ok(None),
        }
    }
    
    fn parse_command<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<Command>>
    {
        match self.parse_compound_command_and_redirects(lexer, settings)? {
            Some((compound_command, redirects, pos)) => Ok(Some(Command::Compound(lexer.path().clone(), pos, compound_command, redirects))),
            None => {
                match lexer.next_token(settings)? {
                    (Token::Word(word_elems), pos) => {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                        let word = Word {
                            path: lexer.path().clone(),
                            pos,
                            word_elems: word_elems.clone(),
                        };
                        match lexer.next_token(settings)? {
                            (Token::LParen, pos2) => {
                                match lexer.next_token(settings)? {
                                    (Token::RParen, _) => {
                                        lexer.push_first_word();
                                        self.has_first_word_or_third_word = true;
                                        self.skip_newlines(lexer, settings)?;
                                        match self.parse_fun_body(lexer, settings)? {
                                            Some(fun_body) => Ok(Some(Command::FunctionDefinition(lexer.path().clone(), pos, Rc::new(word), Rc::new(fun_body)))),
                                            None => {
                                                let (token, pos) = lexer.next_token(settings)?;
                                                let is_cont = match token {
                                                    Token::EOF => true,
                                                    _ => false,
                                                };
                                                return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), is_cont));
                                            },
                                        }
                                    },
                                    (token3, pos3) => {
                                        lexer.undo_token(&token3, &pos3);
                                        lexer.undo_token(&Token::LParen, &pos2);
                                        lexer.undo_token(&Token::Word(word_elems), &pos);
                                        lexer.push_first_word();
                                        self.has_first_word_or_third_word = true;
                                        match self.parse_simple_command(lexer, settings)? {
                                            Some((simple_command, first_pos)) => Ok(Some(Command::Simple(lexer.path().clone(), first_pos, simple_command))),
                                            None => Ok(None),
                                        }
                                    },
                                }
                            },
                            (token2, pos2) => {
                                lexer.undo_token(&token2, &pos2);
                                lexer.undo_token(&Token::Word(word_elems.clone()), &pos);
                                lexer.push_first_word();
                                self.has_first_word_or_third_word = true;
                                match self.parse_simple_command(lexer, settings)? {
                                    Some((simple_command, first_pos)) => Ok(Some(Command::Simple(lexer.path().clone(), first_pos, simple_command))),
                                    None => Ok(None),
                                }
                            },
                        }
                    },
                    (token, pos) => {
                        lexer.undo_token(&token, &pos);
                         match self.parse_simple_command(lexer, settings)? {
                             Some((simple_command, first_pos)) => Ok(Some(Command::Simple(lexer.path().clone(), first_pos, simple_command))),
                             None => Ok(None),
                         }
                    },
                }
            },
        }
    }
    
    fn parse_pipe_command<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<PipeCommand>>
    {
        let is_negative = match lexer.next_token(settings)? {
            (Token::Excl, _) => true,
            (token, pos) => {
                lexer.undo_token(&token, &pos);
                false
            },
        };
        match self.parse_command(lexer, settings)? {
            Some(first_command) => {
                let mut commands: Vec<Rc<Command>> = Vec::new();
                commands.push(Rc::new(first_command.clone()));
                loop {
                    match lexer.next_token(settings)? {
                        (Token::Bar, _) => (),
                        (token, pos) => {
                            lexer.undo_token(&token, &pos);
                            break;
                        },
                    }
                    if self.has_first_word_or_third_word {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                    }
                    lexer.push_first_word();
                    self.has_first_word_or_third_word = true;
                    self.skip_newlines(lexer, settings)?;
                    match self.parse_command(lexer, settings)? {
                        Some(command) => commands.push(Rc::new(command)),
                        None => {
                            let (token, pos) = lexer.next_token(settings)?;
                            let is_cont = match token {
                                Token::EOF => true,
                                _ => false,
                            };
                            return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), is_cont));
                        },
                    }
                }
                let pipe_command = PipeCommand {
                    path: lexer.path().clone(),
                    pos: first_command.pos(),
                    is_negative,
                    commands,
                };
                Ok(Some(pipe_command))
            },
            None => {
                if is_negative {
                    let (token, pos) = lexer.next_token(settings)?;
                    let is_cont = match token {
                        Token::EOF => true,
                        _ => false,
                    };
                    Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), is_cont))
                } else {
                    Ok(None)
                }
            },
        }
    }
    
    fn parse_logical_command<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<LogicalCommand>>
    {
        match self.parse_pipe_command(lexer, settings)? {
            Some(first_command) => {
                let mut pairs: Vec<LogicalPair> = Vec::new();
                loop {
                    let op = match lexer.next_token(settings)? {
                        (Token::AmpAmp, _) => LogicalOperator::And,
                        (Token::BarBar, _) => LogicalOperator::Or,
                        (token, pos) => {
                            lexer.undo_token(&token, &pos);
                            break;
                        },
                    };
                    if self.has_first_word_or_third_word {
                        lexer.pop_state();
                        self.has_first_word_or_third_word = false;
                    }
                    lexer.push_first_word();
                    self.has_first_word_or_third_word = true;
                    self.skip_newlines(lexer, settings)?;
                    match self.parse_pipe_command(lexer, settings)? {
                        Some(command) => {
                            let pair = LogicalPair {
                                op,
                                command: Rc::new(command),
                            };
                            pairs.push(pair);
                        },
                        None => {
                            let (token, pos) = lexer.next_token(settings)?;
                            let is_cont = match token {
                                Token::EOF => true,
                                _ => false,
                            };
                            return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), is_cont));
                        },
                    }
                }
                let logical_command = LogicalCommand {
                    path: lexer.path().clone(),
                    pos: first_command.pos,
                    first_command: Rc::new(first_command),
                    pairs,
                    is_in_background: false,
                };
                Ok(Some(logical_command))
            },
            None => Ok(None),
        }
    }
    
    fn parse_logical_commands_without_last_token<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<LogicalCommand>>>
    {
        let mut commands: Vec<Rc<LogicalCommand>> = Vec::new();
        loop {
            if !self.has_first_word_or_third_word {
                lexer.push_first_word();
                self.has_first_word_or_third_word = true;
            }
            self.skip_newlines(lexer, settings)?;
            match lexer.next_token(settings)? {
                (token @ Token::EOF, pos) => {
                    lexer.undo_token(&token, &pos);
                    break;
                },
                _ => (),
            }
            match self.parse_logical_command(lexer, settings)? {
                Some(mut command) => {
                    match lexer.next_token(settings)? {
                        (Token::Newline, _) => {
                            self.parse_here_docs(lexer, settings)?;
                            commands.push(Rc::new(command));
                        },
                        (Token::Semi, _) => {
                            commands.push(Rc::new(command));
                        },
                        (Token::Amp, _) => {
                            command.is_in_background = true;
                            commands.push(Rc::new(command));
                        },
                        (token, pos) => {
                            lexer.undo_token(&token, &pos);
                            commands.push(Rc::new(command));
                            break;
                        },
                    }
                },
                None => break,
            }
        }
        Ok(commands)
    }
    
    pub fn parse_logical_commands<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Vec<Rc<LogicalCommand>>>
    {
        let commands = self.parse_logical_commands_without_last_token(lexer, settings)?;
        match lexer.next_token(settings)? {
            (Token::EOF, pos) => {
                if !self.here_docs.is_empty() {
                    return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected end of file"), true))
                }
            },
            (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
        if self.has_first_word_or_third_word {
            lexer.pop_state();
            self.has_first_word_or_third_word = false;
        }
        Ok(commands)
    }

    pub fn parse_logical_commands_for_line<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<Option<Vec<Rc<LogicalCommand>>>>
    {
        let mut commands: Vec<Rc<LogicalCommand>> = Vec::new();
        let mut is_eof = false;
        loop {
            if !self.has_first_word_or_third_word {
                lexer.push_first_word();
                self.has_first_word_or_third_word = true;
            }
            match lexer.next_token(settings)? {
                (Token::Newline, _) => {
                    self.parse_here_docs(lexer, settings)?;
                    break;
                },
                (Token::EOF, pos) => {
                    if !self.here_docs.is_empty() {
                        return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected end of file"), true))
                    }
                    is_eof = true;
                    break;
                },
                (token, pos) => lexer.undo_token(&token, &pos),
            }
            match self.parse_logical_command(lexer, settings)? {
                Some(mut command) => {
                    match lexer.next_token(settings)? {
                        (Token::Newline, _) => {
                            self.parse_here_docs(lexer, settings)?;
                            commands.push(Rc::new(command));
                            break;
                        },
                        (Token::Semi, _) => {
                            commands.push(Rc::new(command));
                        },
                        (Token::Amp, _) => {
                            command.is_in_background = true;
                            commands.push(Rc::new(command));
                        },
                        (Token::EOF, pos) => {
                            if !self.here_docs.is_empty() {
                                return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected end of file"), true))
                            }
                            commands.push(Rc::new(command));
                            is_eof = true;
                            break;
                        },
                        (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                    }
                },
                None => {
                    match lexer.next_token(settings)? {
                        (Token::EOF, pos) => {
                            if !self.here_docs.is_empty() {
                                return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected end of file"), true))
                            }
                            is_eof = true;
                            break;
                        },
                        (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                    }
                },
            }
        }
        if self.has_first_word_or_third_word {
            lexer.pop_state();
            self.has_first_word_or_third_word = false;
        }
        if !commands.is_empty() || !is_eof {
            Ok(Some(commands))
        } else {
            Ok(None)
        }
    }

    fn parse_arith_expr12<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        match lexer.next_arith_token(settings)? {
            (ArithmeticToken::Number(n), pos) => Ok(ArithmeticExpression::Number(lexer.path().clone(), pos, n)),
            (ArithmeticToken::Parameter(param_name), pos) => Ok(ArithmeticExpression::Parameter(lexer.path().clone(), pos, param_name)),
            (ArithmeticToken::Tylda, pos) => {
                let expr = self.parse_arith_expr12(lexer, settings)?;
                Ok(ArithmeticExpression::Unary(lexer.path().clone(), pos, UnaryOperator::Not, Rc::new(expr)))
            },
            (ArithmeticToken::Excl, pos) => {
                let expr = self.parse_arith_expr12(lexer, settings)?;
                Ok(ArithmeticExpression::Unary(lexer.path().clone(), pos, UnaryOperator::LogicalNot, Rc::new(expr)))
            },
            (ArithmeticToken::Plus, _) => {
                let expr = self.parse_arith_expr12(lexer, settings)?;
                Ok(expr)
            },
            (ArithmeticToken::Minus, pos) => {
                let expr = self.parse_arith_expr12(lexer, settings)?;
                Ok(ArithmeticExpression::Unary(lexer.path().clone(), pos, UnaryOperator::Negate, Rc::new(expr)))
            },
            (ArithmeticToken::LParen, _) => {
                lexer.push_in_arith_expr_and_paren();
                let expr = self.parse_arith_expr1(lexer, settings)?;
                match lexer.next_arith_token(settings)? {
                    (ArithmeticToken::RParen, _) => {
                        lexer.pop_state();
                        Ok(expr)
                    },
                    (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                }
            },
            (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("syntax error"), false)),
        }
    }

    fn parse_arith_expr11<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr12(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Star, _) => {
                    let expr2 = self.parse_arith_expr12(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Multiply, Rc::new(expr2))
                },
                (ArithmeticToken::Slash, _) => {
                    let expr2 = self.parse_arith_expr12(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Divide, Rc::new(expr2))
                },
                (ArithmeticToken::Perc, _) => {
                    let expr2 = self.parse_arith_expr12(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Module, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_arith_expr10<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr11(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Plus, _) => {
                    let expr2 = self.parse_arith_expr11(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Add, Rc::new(expr2))
                },
                (ArithmeticToken::Minus, _) => {
                    let expr2 = self.parse_arith_expr11(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Substract, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr9<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr10(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::LessLess, _) => {
                    let expr2 = self.parse_arith_expr10(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::ShiftLeft, Rc::new(expr2))
                },
                (ArithmeticToken::GreaterGreater, _) => {
                    let expr2 = self.parse_arith_expr10(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::ShiftRight, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_arith_expr8<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr9(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Less, _) => {
                    let expr2 = self.parse_arith_expr9(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::LessThan, Rc::new(expr2))
                },
                (ArithmeticToken::GreaterEqual, _) => {
                    let expr2 = self.parse_arith_expr9(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::GreaterEqual, Rc::new(expr2))
                },
                (ArithmeticToken::Greater, _) => {
                    let expr2 = self.parse_arith_expr9(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::GreaterThan, Rc::new(expr2))
                },
                (ArithmeticToken::LessEqual, _) => {
                    let expr2 = self.parse_arith_expr9(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::LessEqual, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr7<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr8(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::EqualEqual, _) => {
                    let expr2 = self.parse_arith_expr8(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Equal, Rc::new(expr2))
                },
                (ArithmeticToken::ExclEqual, _) => {
                    let expr2 = self.parse_arith_expr8(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::NotEqual, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr6<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr7(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Amp, _) => {
                    let expr2 = self.parse_arith_expr7(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::And, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr5<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr6(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Caret, _) => {
                    let expr2 = self.parse_arith_expr6(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::ExclusiveOr, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr4<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr5(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::Bar, _) => {
                    let expr2 = self.parse_arith_expr5(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::Or, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr3<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr4(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::AmpAmp, _) => {
                    let expr2 = self.parse_arith_expr4(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::LogicalAnd, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr2<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let mut expr1 = self.parse_arith_expr3(lexer, settings)?;
        let first_pos = expr1.pos();
        loop {
            match lexer.next_arith_token(settings)? {
                (ArithmeticToken::BarBar, _) => {
                    let expr2 = self.parse_arith_expr3(lexer, settings)?;
                    expr1 = ArithmeticExpression::Binary(lexer.path().clone(), first_pos, Rc::new(expr1), BinaryOperator::LogicalOr, Rc::new(expr2))
                },
                (token, pos) => {
                    lexer.undo_arith_token(&token, &pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_arith_expr1<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let expr1 = self.parse_arith_expr2(lexer, settings)?;
        match lexer.next_arith_token(settings)? {
            (ArithmeticToken::Equal, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::Assign, Rc::new(expr2)))
            },
            (ArithmeticToken::StarEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::MultiplyAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::SlashEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::DivideAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::PercEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::ModuleAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::PlusEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::AddAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::MinusEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::SubstractAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::LessLessEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::ShiftLeftAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::GreaterGreaterEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::ShiftRightAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::AmpEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::AndAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::CaretEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::ExclusiveOrAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::BarEqual, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                Ok(ArithmeticExpression::Binary(lexer.path().clone(), expr1.pos(), Rc::new(expr1), BinaryOperator::OrAssign, Rc::new(expr2)))
            },
            (ArithmeticToken::Ques, _) => {
                let expr2 = self.parse_arith_expr1(lexer, settings)?;
                match lexer.next_arith_token(settings)? {
                    (ArithmeticToken::Colon, _) => {
                        let expr3 = self.parse_arith_expr1(lexer, settings)?;
                        Ok(ArithmeticExpression::Conditional(lexer.path().clone(), expr1.pos(), Rc::new(expr1), Rc::new(expr2), Rc::new(expr3)))
                    },
                    (_, pos) => Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
                }
            },
            (token, pos) => {
                lexer.undo_arith_token(&token, &pos);
                Ok(expr1)
            },
        }
    }

    pub fn parse_arith_expr<'a>(&mut self, lexer: &mut Lexer<'a>, settings: &Settings) -> ParserResult<ArithmeticExpression>
    {
        let expr = self.parse_arith_expr1(lexer, settings)?;
        match lexer.next_arith_token(settings)? {
            (ArithmeticToken::EOF, _) => (),
            (_, pos) => return Err(ParserError::Syntax(lexer.path().clone(), pos, String::from("unexpected token"), false)),
        }
        Ok(expr)
    }
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
