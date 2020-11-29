const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::path::Path;

fn main() -> Result<(), Error> {
    let path = Path::new("image.ppm");
    // Create file and use buffered write
    let file = File::create(&path).expect("Unable to create file");
    let mut f = BufWriter::new(file);

    // Write the image header.
    write!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    print!("Scanlines remaining: ");
    for j in 0..IMAGE_HEIGHT {
        let j = IMAGE_HEIGHT - j - 1;
        print!("{} ", j);
        for i in 0..IMAGE_WIDTH {
            write!(f, "{} {} {}\n", i, j, 63)?;
        }
    }

    Ok(())
}
