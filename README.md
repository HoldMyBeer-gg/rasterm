# rasterm

[![CI](https://github.com/jabberwock/rasterm/actions/workflows/rust.yml/badge.svg)](https://github.com/jabberwock/rasterm/actions "GitHub Actions")
[![License](https://img.shields.io/badge/License-AGPL--3.0%20%2B%20Commons%20Clause-30363D?style=flat&labelColor=1e3a5f)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![YouTube — demo](https://img.shields.io/badge/YouTube-Watch%20demo-FF0000?logo=youtube&logoColor=white)](https://www.youtube.com/watch?v=QrUqEvD_na8)


A 3D software renderer written in Rust, rendered entirely in the terminal. No GPU required.

A software rasterizer built from scratch in Rust. No GPU, no OpenGL — pure CPU rasterization rendered to the terminal via ANSI escape codes.

[<img src="https://img.youtube.com/vi/QrUqEvD_na8/hqdefault.jpg" width="640" alt="rasterm demo video thumbnail">](https://www.youtube.com/watch?v=QrUqEvD_na8)

## Features

- Software rasterization with programmable vertex/fragment shaders
- Sutherland-Hodgman near-plane frustum clipping
- Per-pixel depth buffering with bounding-box edge-function rasterizer
- Perspective-correct texture mapping with bilinear sampling
- Directional lighting with two-sided Lambertian diffuse + Blinn-Phong specular
- Keyframe animation system with linear, smooth, and step interpolation
- Morph target vertex animation (Suzanne smile demo)
- OBJ model loading with automatic triangle reduction
- ASCII and 24-bit true color rendering modes
- Parallel terminal output via rayon
- Adaptive resolution on terminal resize
- 5 built-in demo scenes with choreographed camera paths
- Interactive camera controls (orbit, zoom, pause)

## Usage

```bash
# ASCII mode (default) - launches demo reel
cargo run --release

# 24-bit true color mode (recommended)
cargo run --release -- --color

# Load a custom OBJ model
cargo run --release -- model.obj --color

# Load with a texture
cargo run --release -- model.obj --texture diffuse.png --color
```

## Controls

| Key | Action |
|-----|--------|
| 1-5 | Switch demo scene |
| Tab | Next demo |
| WASD / Arrows | Orbit camera (manual override) |
| +/- | Zoom in/out |
| Space | Pause/resume |
| Q | Quit |

## Demo Scenes

1. **Triforce** - Golden Triforce with breathing orbit camera
2. **Suzanne** - Blender's monkey head with animated smile morph
3. **Torus** - Golden torus with swooping camera
4. **Spaceship** - Hand-modeled ship with flyby camera
5. **Goblet** - Lathe-turned golden goblet with elegant orbit

## Architecture

```
src/
├── engine/
│   ├── raster/       # Rasterizer, clipping, shaders, depth buffer
│   ├── render/       # Scene, camera, renderer, materials, textures
│   ├── terminal/     # ASCII & color terminal output (parallel)
│   └── animation/    # Keyframe animation with interpolation
├── loader/           # OBJ file loading & geometry reduction
└── main.rs           # App, demos, camera choreography
models/               # Bundled OBJ models (Blender exports)
```

## Building

```bash
cargo build --release
```

## License
© 2026 — [AGPL-3.0 + Commons Clause](LICENSE). Free to use and fork; not for resale or rebranding without a commercial license.
