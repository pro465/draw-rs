pub use termion::*;

use color::Color;
use std::io::{self, Write};

pub trait Draw {
    type Args;
    type Ret;

    fn draw(&self, writer: &mut impl Write) -> io::Result<Self::Ret>;
    fn update(&mut self, _args: Self::Args) {}
}

pub struct Rect<T: Color + Clone> {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub color: T,
}

impl<T: Color + Clone> Draw for Rect<T> {
    type Args = ();
    type Ret = ();

    fn draw(&self, writer: &mut impl Write) -> io::Result<()> {
        write!(
            writer,
            "{}{}",
            cursor::Goto(self.x, self.y),
            color::Bg(self.color.clone())
        )?;

        for y in 0..self.h {
            for _ in 0..self.w {
                writer.write(b" ")?;
            }
            write!(writer, "{}", cursor::Goto(self.x, self.y + y))?;
        }

        write!(writer, "{}", color::Bg(color::Reset))?;

        Ok(())
    }
}
