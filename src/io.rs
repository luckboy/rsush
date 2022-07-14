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

pub trait CharRead: BufRead
{
    fn get_char(&mut self) -> Result<Option<char>>
    {
        let mut char_buf: Vec<u8> = Vec::new();
        for i in 0..6 {
            let mut buf: [u8; 1] = [0; 1];
            let mut is_eof = false;
            loop {
                match self.read(&mut buf) {
                    Ok(0) => {
                        is_eof = true;
                        break;
                    },
                    Ok(_) => {
                        char_buf.push(buf[0]);
                        break;
                    },
                    Err(err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(err),
                }
            }
            if !is_eof {
                match String::from_utf8(char_buf.clone()) {
                    Ok(s) => return Ok(Some(s.chars().next().unwrap())),
                    Err(_) => (),
                }
            } else {
                if i == 0 {
                    return Ok(None);
                } else {
                    return Err(Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8"));
                }
            }
        }
        Err(Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8"))
    }
}

pub struct CharReader<R: BufRead>
{
    reader: R,
}

impl<R: BufRead> CharReader<R>
{
    pub fn new(reader: R) -> CharReader<R>
    { CharReader { reader, } }
}

impl<R: BufRead> Read for CharReader<R>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    { self.reader.read(buf) }
}

impl<R: BufRead> BufRead for CharReader<R>
{
    fn fill_buf(&mut self) -> Result<&[u8]>
    { self.reader.fill_buf() }

    fn consume(&mut self, amt: usize)
    { self.reader.consume(amt); }
}

impl<R: BufRead> CharRead for CharReader<R>
{}

pub trait LineRead: Read
{
    fn read_line(&mut self, buf: &mut String) -> Result<usize>
    {
        let mut line_buf: Vec<u8> = Vec::new();
        loop {
            let mut byte_buf: [u8; 1] = [0; 1];
            match self.read(&mut byte_buf) {
                Ok(0) => break,
                Ok(_) => {
                    line_buf.push(byte_buf[0]);
                    if byte_buf[0] == b'\n' {
                        break;
                    }
                },
                Err(err) if err.kind() == ErrorKind::Interrupted => (),
                Err(err) => return Err(err),
            }
        }
        let size = line_buf.len();
        match String::from_utf8(line_buf) {
            Ok(s) => {
                buf.push_str(s.as_str());
                Ok(size)
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8")),
        }
    }
}

pub struct LineReader<R: Read>
{
    reader: R,
}

impl<R: Read> LineReader<R>
{
    pub fn new(reader: R) -> LineReader<R>
    { LineReader { reader, } }
}

impl<R: Read> Read for LineReader<R>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    { self.reader.read(buf) }
}

impl<R: Read> LineRead for LineReader<R>
{}
