# rotsprite

Rust implementation/library of the RotSprite algorithm.

<a href="https://actions-badge.atrox.dev/tversteeg/rotsprite/goto"><img src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Ftversteeg%2Frotsprite%2Fbadge&style=flat" alt="Build Status"/></a>
<a href="https://crates.io/crates/rotsprite"><img src="https://img.shields.io/crates/v/rotsprite.svg" alt="Version"/></a>
<a href="https://docs.rs/rotsprite"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
<img src="https://img.shields.io/crates/l/rotsprite.svg" alt="License"/>

Works with many types of pixel buffers.

## Example

Before rotation:

![Example](example/king-by-buck.png?raw=true)

After rotation:

![Example](docs/example1.png?raw=true)

### Run the example

On Linux you need the `xorg-dev` package as required by minifb. `sudo apt install xorg-dev`

```sh
cargo run --example minifb
```
