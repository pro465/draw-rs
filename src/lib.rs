use std::io::{self, Write};

pub trait Draw {
    type Args;
    type Ret;

    fn draw(&self, writer: &mut impl Write) -> io::Result<Self::Ret>;
    fn update(&mut self, _args: Self::Args) {}
}
