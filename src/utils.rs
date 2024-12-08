use std::fs::File;
use std::io::{self, Write};
use crate::vec3::Vec3;

pub fn write_to_ppm(
    filename: &str,
    image: &Vec<Vec<Vec3>>,
) -> io::Result<()> {
    let height = image.len();
    if height == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty image"));
    }
    
    let width = image[0].len();
    if image.iter().any(|row| row.len() != width) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Inconsistent row lengths"));
    }

    let mut file = File::create(filename)?;
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    for row in image {
        for pixel in row {
            let r = (pixel.x() * 255.999) as u8;
            let g = (pixel.y() * 255.999) as u8;
            let b = (pixel.z() * 255.999) as u8;
            file.write_all(&[r, g, b])?;
        }
    }

    Ok(())
}
