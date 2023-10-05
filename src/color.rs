use crate::vec3::Color;
use std::io::{Stdout, Write};

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
    let buf = format!(
        "{} {} {}\n",
        pixel_color.r(),
        pixel_color.g(),
        pixel_color.b()
    );

    out.write(buf.as_bytes())
        .expect("Unable to write color data!");
}
