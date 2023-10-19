use std::env;
use std::env::Args;

pub enum Preset {
    Fast { samples_per_pixel: i32, depth: u32 },
    Slow { samples_per_pixel: i32, depth: u32 },
}

pub fn parse_preset(args: Args) -> Option<Preset> {
    let mut preset = None;

    for arg in args.into_iter() {
        match arg.as_str() {
            "-fast" => {
                preset = Some(Preset::Fast {
                    samples_per_pixel: 10,
                    depth: 10,
                })
            }
            "-slow" => {
                preset = Some(Preset::Slow {
                    samples_per_pixel: 100,
                    depth: 50,
                })
            }
            _ => {
                preset = Some(Preset::Fast {
                    samples_per_pixel: 10,
                    depth: 10,
                })
            }
        };
    }

    preset
}
