use std::io::Write;

mod vec3;
pub use crate::vec3::Vec3;
pub use crate::vec3::write_color;

use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        std::io::stderr().flush().expect("Unable to flush stderr");

        for i in 0..image_width {
            let pixel_color = Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_width - 1) as f64),
                0.25
            );
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}

