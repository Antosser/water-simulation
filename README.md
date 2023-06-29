# Water Simulation
Simulates how virtual water would flow in a 2d environment

## Examples
![maze](https://raw.githubusercontent.com/Antosser/water-simulation/master/examples/maze.png)

https://github.com/Antosser/water-simulation/assets/71790328/3446ae80-bf5c-4c0b-8323-9d22f46580c5

![snail](https://raw.githubusercontent.com/Antosser/water-simulation/master/examples/snail.png)

https://github.com/Antosser/water-simulation/assets/71790328/e9278dd7-364a-4835-bd53-4e15381329b4

## Usage
```
A water simulation written in Rust

Usage: water-simulation.exe [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Location of the image

Options:
  -f, --framerate <FRAMERATE>  Framerate of the output video [default: 30]
  -n, --filename <FILENAME>    Name of the output video [default: out.mp4]
  -a, --approximate            Whether to use the approximate algorithm (gray will be treated as wall)
  -d, --debug                  Whether to convert image to simplest form
  -h, --help                   Print help
  -V, --version                Print version
```

## Image
* Black (0, 0, 0) = wall
* Blue (0, 0, 255) = water
* Anything else = air

## Requirements
1. ffmpeg in path
2. cargo if you want to build yourself

## Installation using GitHub Releases
1. Download the executable
2. Run it with the image as an argument

## Installation using Cargo
1. Install cargo (Rust)
2. Run `cargo install water-simulation`
3. Run `water-simulation your_file.png`
