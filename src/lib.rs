use std::io::{Result, Write};

pub fn broadcast<A: Write, B: Write>(primary: A, secondary: B) -> Broadcast<A, B> {
    Broadcast { primary: primary, secondary: secondary }
}

pub struct Broadcast<A, B> {
    primary: A,
    secondary: B,
}

impl<A: Write, B: Write> Write for Broadcast<A, B> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        let n = try!(self.primary.write(data));
        try!(self.secondary.write_all(&data[..n]));
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.primary.flush().and(self.secondary.flush())
    }
}

#[test]
fn it_works() {
}
