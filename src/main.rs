use clap::Parser;
use image::Rgb;
use log::info;
use std::{path::PathBuf, process::Command};

enum Cell {
    Wall,
    Water,
    Air,
}
impl Cell {
    pub fn from_pixel(pixel: &Rgb<u8>) -> Option<Self> {
        match pixel {
            Rgb([0, 0, 0]) => Some(Self::Wall),
            Rgb([0, 0, 255]) => Some(Self::Water),
            Rgb([255, 255, 255]) => Some(Self::Air),
            _ => None,
        }
    }

    pub fn from_pixel_approximate(pixel: &Rgb<u8>) -> Option<Self> {
        match pixel {
            Rgb([0, 0, 0]) => Some(Self::Wall),
            Rgb([0, 0, 255]) => Some(Self::Water),
            Rgb([r, g, b]) => {
                if *r < 150 && *g < 150 && *b < 150 {
                    Some(Self::Wall)
                } else {
                    None
                }
            }
        }
    }
}

/// Simulates the flow of water
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the image
    image: PathBuf,

    /// Framerate of the output video
    #[clap(short, long, default_value = "30")]
    framerate: u32,

    /// Name of the output video
    #[clap(short = 'n', long, default_value = "out.mov")]
    filename: String,

    /// Whether to use the approximate algorithm (gray will be treated as wall)
    #[clap(short, long)]
    approximate: bool,

    /// Whether to convert image to simplest form
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut args = Args::parse();
    let image = image::open(args.image).expect("Failed to open image");
    let mut rgb = image.to_rgb8();

    if PathBuf::from("images").exists() {
        std::fs::remove_dir_all("images").expect("Failed to remove images directory");
    }
    std::fs::create_dir_all("images").expect("Failed to create images directory");

    if args.debug {
        for x in 0..rgb.width() {
            for y in 0..rgb.height() {
                let pixel = rgb.get_pixel_mut(x, y);

                match match args.approximate {
                    true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                    false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                } {
                    Cell::Wall => *pixel = Rgb([0, 0, 0]),
                    Cell::Water => *pixel = Rgb([0, 0, 255]),
                    Cell::Air => *pixel = Rgb([255, 255, 255]),
                }
            }
        }

        args.approximate = false;
    }

    std::thread::scope(|thread_scope| {
        let mut cached_max_y = rgb.height();

        'frames: for frame in 1.. {
            'checks: {
                info!("Frame {}", frame);
                for y in (0..cached_max_y).rev() {
                    let mut water_coords = vec![];

                    for x in 0..rgb.width() {
                        let pixel = rgb.get_pixel(x, y);
                        let cell = match args.approximate {
                            true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                            false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                        };

                        if let Cell::Water = cell {
                            water_coords.push((x, y));
                        }
                    }

                    let mut found_space = false;

                    // Check space below first
                    for water_coord in &water_coords {
                        let below = (water_coord.0, water_coord.1 + 1);
                        if below.1 < rgb.height() {
                            let pixel = rgb.get_pixel(below.0, below.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                rgb.put_pixel(below.0, below.1, Rgb([0, 0, 255]));
                                found_space = true;
                            }
                        }
                    }

                    if found_space {
                        cached_max_y = y + 2;
                        break 'checks;
                    }

                    // Check space to the side second
                    for water_coord in &water_coords {
                        'check: {
                            if water_coord.0 == 0 {
                                break 'check;
                            }
                            let left = (water_coord.0 - 1, water_coord.1);
                            if left.0 < rgb.width() {
                                let pixel = rgb.get_pixel(left.0, left.1);
                                let cell = match args.approximate {
                                    true => {
                                        Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air)
                                    }
                                    false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                                };

                                if let Cell::Air = cell {
                                    rgb.put_pixel(left.0, left.1, Rgb([0, 0, 255]));
                                    found_space = true;
                                }
                            }
                        }

                        let right = (water_coord.0 + 1, water_coord.1);
                        if right.0 < rgb.width() {
                            let pixel = rgb.get_pixel(right.0, right.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                rgb.put_pixel(right.0, right.1, Rgb([0, 0, 255]));
                                found_space = true;
                            }
                        }
                    }

                    if found_space {
                        cached_max_y = y + 1;
                        break 'checks;
                    }

                    // Check space above last
                    for water_coord in &water_coords {
                        if water_coord.1 == 0 {
                            continue;
                        }

                        let above = (water_coord.0, water_coord.1 - 1);
                        if above.1 < rgb.height() {
                            let pixel = rgb.get_pixel(above.0, above.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                rgb.put_pixel(above.0, above.1, Rgb([0, 0, 255]));
                                found_space = true;
                            }
                        }
                    }

                    if found_space {
                        cached_max_y = y;
                        break 'checks;
                    }
                }

                if true {
                    // Added because of clippy::never_loop
                    // Save image
                    let rgb = rgb.clone();
                    thread_scope.spawn(move || {
                        info!("Saving image #{:04}", frame);
                        rgb.save(format!("images/{:04}.png", frame))
                            .expect("Failed to save image");
                    });

                    break 'frames;
                }
            }
            // Save image
            let rgb = rgb.clone();
            thread_scope.spawn(move || {
                info!("Saving image #{:04}", frame);
                rgb.save(format!("images/{:04}.png", frame))
                    .expect("Failed to save image");
            });
        }
    });

    if PathBuf::from("output.mp4").exists() {
        std::fs::remove_file("output.mp4").expect("Failed to remove output.mp4");
    }

    // ffmpeg -framerate 30 -i images/%04d.png -vf scale=1000x1000:flags=neighbor output.mp4
    Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate",
            args.framerate.to_string().as_str(),
            "-i",
            "images/%04d.png",
            "-vf",
            "scale=1000x1000:flags=neighbor",
            args.filename.as_str(),
        ])
        .output()
        .expect("Failed to run ffmpeg");

    std::fs::remove_dir_all("images").expect("Failed to remove images directory");
}
