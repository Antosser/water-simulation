# Water Simulation
Simulates how virtual water would flow in a 2d environment

## Examples
![maze](https://github.com/Antosser/water-simulation/assets/71790328/7cf2877c-263c-4296-bf68-d158bcc322d6)

https://github.com/Antosser/water-simulation/assets/71790328/913ea621-cbc0-4e39-af57-3209f7b3f61d

![snail](https://github.com/Antosser/water-simulation/assets/71790328/eeea46e8-d109-4ebe-936f-62ae60223a36)

https://github.com/Antosser/water-simulation/assets/71790328/ba4ef345-304a-4b1a-a986-d03725ac542f

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
