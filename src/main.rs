use std::io::{self,Write};

mod vec3;
mod color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n {} {}\n255\n",image_width,image_height);

    let mut out = io::stdout();

    for j in (0..image_height).rev(){
        let indicator = format!("\rScan lines remaining: {} ",j);
        io::stderr().write(indicator.as_bytes()); out.flush();
        for i in 0..image_width {
            let r = i as f32 / image_width as f32;
            let g = j as f32 / image_height as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let pixel_color = vec3::vec3::Color::new(ir as f32,ig as f32,ib as f32);

            color::write_color(&mut out,pixel_color);
        }
        io::stderr().write(b"\nDone.\n");
    }
}
