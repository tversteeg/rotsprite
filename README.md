# rotsprite

[![Build Status](https://github.com/tversteeg/rotsprite/workflows/CI/badge.svg)](https://github.com/tversteeg/rotsprite/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/rotsprite.svg)](https://crates.io/crates/rotsprite)
[![Documentation](https://docs.rs/rotsprite/badge.svg)](https://docs.rs/rotsprite)
[![License: AGPL-3.0-or-later](https://img.shields.io/crates/l/rotsprite.svg)](#license)
[![Downloads](https://img.shields.io/crates/d/rotsprite.svg)](#downloads)

### [Documentation](https://docs.rs/rotsprite/)

<!-- cargo-rdme start -->

Pixel Art rotation algorithms that works with many types of pixel buffers.

This library allows you to rotate pixel art using the [rotsprite](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite) algorithm.

<!-- cargo-rdme end -->

![Large](docs/example-large.png?raw=true)
![Small](docs/example-small.png?raw=true)

| Left Picture | Middle Picture | Right Picture|
|-|-|-|
| Source Image | Rotated 30° using RotSprite | Rotated 30° using naive rotation |

## Demos

### [WASM Demo](https://tversteeg.nl/rotsprite/window/)

Web: https://tversteeg.nl/rotsprite/window

Uses the `["blit"]` feature flag.

#### Local

```console
cargo run --example window
```

#### Credits

[RotSprite algorithm - Xenowhirl](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite)<br/>
[Pixel Art - Redshrike](https://opengameart.org/content/3-form-rpg-boss-harlequin-epicycle)
