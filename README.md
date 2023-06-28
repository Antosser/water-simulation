# Water Simulation
Simulates how virtual water would flow in a 2d environment

## Examples
![maze](https://raw.githubusercontent.com/Antosser/water-simulation/master/examples/maze.png)

https://github.com/Antosser/water-simulation/assets/71790328/1c561ca6-190d-495b-947e-5f4957e772e0

![snail](https://raw.githubusercontent.com/Antosser/water-simulation/master/examples/snail.png)

https://github.com/Antosser/water-simulation/assets/71790328/432adb90-d00a-421f-8ed7-dd6bf3d2538f

## Usage
```
A water simulation written in Rust

Usage: water-simulation.exe [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Location of the image

Options:
  -f, --framerate <FRAMERATE>  Framerate of the output video [default: 30]
  -n, --filename <FILENAME>    Name of the output video [default: out.mov]
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
