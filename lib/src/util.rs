use color_eyre::eyre::Result;

use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_color(buffer: &mut BufWriter<File>, x: u16, y: u16, z: u16) -> Result<()> {
    write!(buffer, "{} {} {}\n", x, y, z)?;
    Ok(())
}
