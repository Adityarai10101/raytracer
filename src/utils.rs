use std::fs::File;
use std::io::{self, Write};
use crate::vec3::Vec3;
use std::time::{SystemTime, UNIX_EPOCH};

// convert from linear space colors to gamma space colors
fn linear_to_gamma(
    linear_value: f64
) -> f64 {
    if linear_value > 0.0 { // this should ALWAYS be true
        return linear_value.sqrt();
    }
    return 0.0;
}

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

            let r = (linear_to_gamma(pixel.x()) * 255.999) as u8;
            let g = (linear_to_gamma(pixel.y()) * 255.999) as u8;
            let b = (linear_to_gamma(pixel.z()) * 255.999) as u8;
            file.write_all(&[r, g, b])?;
        }
    }

    Ok(())
}

