use anyhow::anyhow;
use clap::Parser;
use image::{Rgb, RgbImage};
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path::PathBuf, process::Command};

const AIR: [u8; 3] = [255, 255, 255];
const WALL: [u8; 3] = [0, 0, 0];
const WATER: [u8; 3] = [0, 0, 255];
const APPROXIMATE_WATER_THRESHOLD: u8 = 150;

enum Cell {
    Wall,
    Water,
    Air,
}
impl Cell {
    pub fn from_pixel(pixel: &Rgb<u8>) -> Option<Self> {
        match pixel {
            Rgb(WALL) => Some(Self::Wall),
            Rgb(WATER) => Some(Self::Water),
            Rgb(AIR) => Some(Self::Air),
            _ => None,
        }
    }

    pub fn from_pixel_approximate(pixel: &Rgb<u8>) -> Option<Self> {
        match pixel {
            Rgb(WALL) => Some(Self::Wall),
            Rgb(WATER) => Some(Self::Water),
            Rgb([r, g, b]) => {
                if *r < APPROXIMATE_WATER_THRESHOLD
                    && *g < APPROXIMATE_WATER_THRESHOLD
                    && *b < APPROXIMATE_WATER_THRESHOLD
                {
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
    #[clap(short = 'n', long, default_value = "out.mp4")]
    filename: String,

    /// Whether to use the approximate algorithm (gray will be treated as wall)
    #[clap(short, long)]
    approximate: bool,

    /// Whether to convert image to simplest form
    #[clap(short, long)]
    debug: bool,
}

fn simulate_into_images(image: &mut RgbImage, args: &Args) -> anyhow::Result<()> {
    std::thread::scope(|thread_scope| -> anyhow::Result<()> {
        let mut threads = Vec::new();

        let mut cached_max_y = image.height();
        'frames: for frame in 1.. {
            'checks: {
                info!("Frame {}", frame);
                for y in (0..cached_max_y).rev() {
                    let water_coords = (0..image.width())
                        .into_par_iter()
                        .filter_map(|x| {
                            let pixel = image.get_pixel(x, y);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Water = cell {
                                Some((x, y))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    let mut found_space = false;

                    // Check space below first
                    for water_coord in &water_coords {
                        let below = (water_coord.0, water_coord.1 + 1);
                        if below.1 < image.height() {
                            let pixel = image.get_pixel(below.0, below.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                image.put_pixel(below.0, below.1, Rgb([0, 0, 255]));
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
                            if left.0 < image.width() {
                                let pixel = image.get_pixel(left.0, left.1);
                                let cell = match args.approximate {
                                    true => {
                                        Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air)
                                    }
                                    false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                                };

                                if let Cell::Air = cell {
                                    image.put_pixel(left.0, left.1, Rgb([0, 0, 255]));
                                    found_space = true;
                                }
                            }
                        }

                        let right = (water_coord.0 + 1, water_coord.1);
                        if right.0 < image.width() {
                            let pixel = image.get_pixel(right.0, right.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                image.put_pixel(right.0, right.1, Rgb([0, 0, 255]));
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
                        if above.1 < image.height() {
                            let pixel = image.get_pixel(above.0, above.1);
                            let cell = match args.approximate {
                                true => Cell::from_pixel_approximate(pixel).unwrap_or(Cell::Air),
                                false => Cell::from_pixel(pixel).unwrap_or(Cell::Air),
                            };

                            if let Cell::Air = cell {
                                image.put_pixel(above.0, above.1, Rgb([0, 0, 255]));
                                found_space = true;
                            }
                        }
                    }

                    if found_space {
                        cached_max_y = y;
                        break 'checks;
                    }
                }

                let rgb = image.clone();
                threads.push(thread_scope.spawn(move || -> anyhow::Result<()> {
                    info!("Saving image #{:06}", frame);
                    rgb.save(format!("images/{:06}.png", frame))?;

                    Ok(())
                }));

                break 'frames;
            }
            // Save image
            let rgb = image.clone();
            threads.push(thread_scope.spawn(move || -> anyhow::Result<()> {
                info!("Saving image #{:06}", frame);
                rgb.save(format!("images/{:06}.png", frame))?;

                Ok(())
            }));
        }

        for thread in threads {
            thread
                .join()
                .map_err(|_| anyhow!("Error joining thread"))??
        }

        Ok(())
    })?;

    Ok(())
}

fn render_video(args: &Args) -> anyhow::Result<()> {
    if PathBuf::from("output.mp4").exists() {
        std::fs::remove_file("output.mp4")?;
    }

    // ffmpeg -framerate 30 -i images/%06d.png -vf scale=1000x1000:flags=neighbor output.mp4
    Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate",
            args.framerate.to_string().as_str(),
            "-i",
            "images/%06d.png",
            "-vf",
            "scale=1000x1000:flags=neighbor",
            args.filename.as_str(),
        ])
        .spawn()?
        .wait()?;

    std::fs::remove_dir_all("images")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut args = Args::parse();
    let mut image = image::open(args.image.clone())?.to_rgb8();

    if PathBuf::from("images").exists() {
        std::fs::remove_dir_all("images")?;
    }
    std::fs::create_dir_all("images")?;

    if args.debug {
        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = image.get_pixel_mut(x, y);

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

    simulate_into_images(&mut image, &args)?;

    render_video(&args)?;

    Ok(())
}
