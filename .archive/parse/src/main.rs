extern crate combine;

use combine::{Parser, satisfy, Stream, ParseResult, parser, chainl1, StreamOnce, many1, eof};
use combine::char::{string, letter, alpha_num, space};


#[derive(Debug, Clone)]
pub struct CRStream<I> {
    inner: I,
    rownum: usize,
    rowpos: usize,
}

impl<I> CRStream<I> where I: Stream<Item=char, Position=usize> {
    pub fn new(inner: I) -> Self {
        let rowpos = inner.position();
        CRStream {
            inner: inner,
            rownum: 0,
            rowpos: rowpos,
        }
    }
}

impl<I> StreamOnce for CRStream<I> where I: Stream<Item=char, Position=usize> {
    type Item = I::Item;
    type Range = I::Range;
    type Position = (usize, usize);
    fn uncons(&mut self) -> Result<Self::Item, combine::primitives::Error<Self::Item, Self::Range>> {
        let c = self.inner.uncons()?;
        if c == '\n' {
            self.rownum += 1;
            self.rowpos = self.inner.position();
        }
        Ok(c)
    }
    fn position(&self) -> Self::Position {
        (self.rownum, self.inner.position() - self.rowpos)
    }
}


fn main() {
//    let mut parser = many1::<Vec<_>, _>(digit());
//    println!("{:?}", parser.parse("123"));
//    println!("{:?}", parser.parse("123ABC"));
//    println!("{:?}", parser.parse("ABC123"));
//
//    // 英数字と空白のみからなる文字列を受理する
//    let mut parser = many1::<Vec<_>, _>(alpha_num().or(space())).skip(eof());
//    println!("{:?}", parser.parse("hoge"));
//    println!("{:?}", parser.parse("hoge+fuga"));
//    println!("{:?}", parser.parse("h\nge+fuga"));


    // 英数字と空白のみからなる文字列を受理する
    let mut parser = many1::<Vec<_>, _>(alpha_num().or(space())).skip(eof());
    println!("{:?}", parser.parse(CRStream::new("hoge")));
    println!("{:?}", parser.parse(CRStream::new("hoge+fuga")));
    println!("{:?}", parser.parse(CRStream::new("h\nge+fuga")));
}
