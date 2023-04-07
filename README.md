# rotsprite

Rust implementation/library of the RotSprite algorithm.

![CI](https://github.com/tversteeg/rotsprite/workflows/CI/badge.svg?branch=master)
<a href="https://crates.io/crates/rotsprite"><img src="https://img.shields.io/crates/v/rotsprite.svg" alt="Version"/></a>
<a href="https://docs.rs/rotsprite"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
<img src="https://img.shields.io/crates/l/rotsprite.svg" alt="License"/>

Works with many types of pixel buffers.


![Large](docs/example-large.png?raw=true)
![Small](docs/example-small.png?raw=true)

| Left Picture | Middle Picture | Right Picture|
|-|-|-|
| Source Image | Rotated 30° using RotSprite | Rotated 30° using naive rotation |

## Example

## Demos

### [WASM Demo](https://tversteeg.nl/rotsprite/window/)

Web: https://tversteeg.nl/rotsprite/window

Uses the `["blit"]` feature flag.

#### Local

```console
cargo run --example window
```

## Credits

[RotSprite algorithm - Xenowhirl](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite)<br/>
[Pixel Art - Redshrike](https://opengameart.org/content/3-form-rpg-boss-harlequin-epicycle)
