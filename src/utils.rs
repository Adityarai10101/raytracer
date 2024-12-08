use std::fs::File;
use std::io::{self, Write};

pub fn write_to_ppm<const W: usize, const H: usize>(
    filename: &str,
    image: &[[[u8; 3]; W]; H],
) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "P6")?;
    writeln!(file, "{} {}", image[0].len(), image.len())?;
    writeln!(file, "255")?;

    for y in 0..image.len() {
        for x in 0..image[0].len() {
            file.write_all(&image[y][x])?;
        }
    }

    Ok(())
}
