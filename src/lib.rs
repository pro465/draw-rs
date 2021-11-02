use std::io::{self, Write};

pub trait Draw {
    type Args1;
    type Args2;
    type Ret;

    fn draw(&self, writer: &mut impl Write, _args: Self::Args1) -> io::Result<Self::Ret>;
    fn update(&mut self, _args: Self::Args2) {}
}
