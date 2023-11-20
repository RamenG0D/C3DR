
# 3D Renderer

## Overview

This is a 3D renderer written in Rust. It's designed to be fast, efficient, and easy to use in other projects.

it uses raylib for rendering but, it won't use the built in 3d support from raylib.

Instead math will be performed on 3d points that make triangles to produce points in 2d space that look like their 3d counter parts on a screen.

test will be used to produce a program demonstrating a feature of the current working system, but it should be plug and chug into another project, kinda like how the built in raylib camera system works.

### Prerequisites for buildig from source

- Rust `1.73.0` (has been tested on 1.73.0 but might work on other versions)

### Installation

1. Clone the repository
```sh
    git clone https://github.com/RamenG0D/C3DR.git
```

2. Build the project
```sh
    cargo build --release
```
`OR` Run the project
```sh
    cargo run --release
```

## Goals

✅ = Done
❌ = Not Done
🚧 = In Progress

| Feature                                 | Status |
|-----------------------------------------|--------|
| Vertex processing                       |✅|
| Transformations                         |✅|
| Projection                              |✅|
| Rotation                                |✅|
| Scaling                                 |✅|
| Triangle rendering                      |✅|
| Wireframe rendering                     |✅|
| Clipping                                |❌|
| Rasterization                           |❌|
| Shading                                 |✅|
| Texturing                               |❌|
| Lighting                                |✅|
| Animation                               |❌|
| Physics                                 |❌|
| Collision detection                     |❌|
| Object file support                     |❌|
| More complex scene rendering            |❌|
| Making a minecraft clone(Ultimate goal) |❌|

## Inspiration

- [OneLoneCoder/ConsoleGameEngine](https://github.com/OneLoneCoder/Javidx9/tree/master/ConsoleGameEngine/BiggerProjects/Engine3D)

## If you like this project, please consider giving it a ⭐!
