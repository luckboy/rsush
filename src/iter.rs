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
use std::iter::Iterator;

pub trait PushbackIterator: Iterator
{
    fn undo(&mut self, item: Self::Item);
}

#[derive(Clone)]
pub struct PushbackIter<I: Iterator>
{
    iter: I,
    pushed_items: Vec<I::Item>,
}

impl<I: Iterator> PushbackIter<I>
{
    pub fn new(iter: I) -> PushbackIter<I>
    { PushbackIter { iter, pushed_items: Vec::new(), } }
}

impl<I: Iterator> Iterator for PushbackIter<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item>
    {
        match self.pushed_items.pop() {
            Some(item) => Some(item),
            None       => self.iter.next(),
        }
    }
}

impl<I: Iterator> PushbackIterator for PushbackIter<I>
{
    fn undo(&mut self, item: Self::Item)
    { self.pushed_items.push(item); }
}
