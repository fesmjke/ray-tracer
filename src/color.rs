use std::io::{Stdout, Write};

use crate::vec3::vec3::{Color};

pub fn write_color(out : &mut Stdout,pixel_color : Color) {

    let buf = format!("{} {} {}\n",pixel_color.r(),pixel_color.g(),pixel_color.b());

    out.write(buf.as_bytes());
}