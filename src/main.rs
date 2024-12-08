use std::io;
use raytracer::utils;

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    
    // Create a 3D array to store the image
    let mut image = [[[0u8; 3]; IMAGE_WIDTH]; IMAGE_HEIGHT];
    
    // Generate the gradient
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;
            
            image[j][i] = [
                (255.999 * r) as u8,
                (255.999 * g) as u8,
                (255.999 * b) as u8,
            ];
        }
    }
    
    utils::write_to_ppm("output.ppm", &image)?;
    Ok(())
}
