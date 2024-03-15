# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5](https://github.com/tversteeg/rotsprite/compare/rotsprite-v0.1.4...rotsprite-v0.1.5) - 2024-03-15

### Added
- *(example)* add WASM example
- support blit with trait

### Fixed
- *(deps)* update rust crate multiversion to 0.7.4
- *(deps)* update rust crate thiserror to 1.0.58
- *(deps)* update rust crate thiserror to 1.0.57
- *(deps)* update rust crate thiserror to 1.0.56
- *(deps)* update rust crate thiserror to 1.0.53
- *(deps)* update rust crate thiserror to 1.0.52
- *(deps)* update rust crate thiserror to 1.0.51
- *(deps)* update rust crate thiserror to 1.0.50
- *(deps)* update rust crate thiserror to 1.0.49
- *(deps)* update rust crate thiserror to 1.0.48
- *(deps)* update rust crate thiserror to 1.0.47
- *(deps)* update rust crate thiserror to 1.0.46
- *(deps)* update rust crate thiserror to 1.0.45
- *(deps)* update rust crate multiversion to 0.7.3
- *(deps)* update rust crate thiserror to 1.0.44
- *(deps)* update rust crate thiserror to 1.0.43
- *(deps)* update rust crate thiserror to 1.0.41
- *(deps)* update rust crate multiversion to 0.7.2

### Other
- *(ci)* fix main branch for automated releases
- *(lib)* document `blit` feature flag
- *(readme)* fix demo URL
- *(example)* simplify window example a lot by using the 'pixel-game-lib' library
- *(ci)* use automated release and README generation
- *(deps)* update rust crate blit to 0.8.5
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate web-time to 1.1.0
- *(deps)* update rust crate log to 0.4.21
- *(deps)* update rust crate image to 0.24.9
- *(deps)* update rust crate bytemuck to 1.14.3
- *(deps)* update rust crate bytemuck to 1.14.2
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate bytemuck to 1.14.1
- *(deps)* update rust crate web-time to v1
- *(deps)* update swatinem/rust-cache action to v2.7.3
- *(deps)* update rust crate image to 0.24.8
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update swatinem/rust-cache action to v2.7.2
- *(deps)* update rust crate web-time to 0.2.4
- *(deps)* update rust crate web-sys to 0.3.66
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate web-time to 0.2.3
- *(deps)* update swatinem/rust-cache action to v2.7.1
- *(deps)* update actions/checkout digest to b4ffde6
- *(deps)* update rust crate blit to 0.8.4
- *(deps)* update rust crate web-time to 0.2.2
- *(deps)* update rust crate web-time to 0.2.1
- *(deps)* update rust crate winit to 0.28.7
- *(deps)* update rust crate blit to 0.8.3
- *(deps)* update actions/checkout digest to 8ade135
- *(deps)* update swatinem/rust-cache action to v2.7.0
- *(deps)* update rust crate bytemuck to 1.14.0
- *(deps)* update actions/checkout action to v4
- *(deps)* update swatinem/rust-cache action to v2.6.2
- *(deps)* update swatinem/rust-cache action to v2.6.1
- *(deps)* update rust crate log to 0.4.20
- *(deps)* update rust crate image to 0.24.7
- *(deps)* update swatinem/rust-cache action to v2.6.0
- *(deps)* update rust crate blit to 0.8.2
- *(deps)* update swatinem/rust-cache action to v2.5.1
- *(deps)* update swatinem/rust-cache action to v2.5.0
- *(deps)* update rust crate pixels to 0.13.0
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate log to 0.4.19
- *(deps)* update rust crate log to 0.4.18
- *(deps)* update rust crate criterion to 0.5.1
- *(deps)* update rust crate criterion to 0.5.0
- *(deps)* update swatinem/rust-cache action to v2.4.0
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate winit to 0.28.6
- *(deps)* update rust crate blit to 0.8.1
- *(deps)* update swatinem/rust-cache action to v2.3.0
- *(deps)* update rust-wasm-bindgen monorepo
- *(deps)* update rust crate winit to 0.28.5
- *(deps)* update rust crate winit to 0.28.4
- update blit & switch WASM example to pixels
- Release
- Add renovate.json
- made the unfound color pixel transparent
- added simple example
- made imports in the minifb example explicit
- Improve performance a tiny bit by using multiversion
- Update dependencies
- Merge branch 'master' of github.com:tversteeg/rotsprite
- [ImgBot] Optimize images
- Fix benchmark
- Replace CI badge in README.md
- Merge branch 'master' of github.com:tversteeg/rotsprite
- (cargo-release) start next development iteration 0.1.4-alpha.0
- (cargo-release) version 0.1.3
- Update example with better source image
- Greatly optimize rotation algorithm
- (cargo-release) start next development iteration 0.1.3-alpha.0
- (cargo-release) version 0.1.2
- Actually fix scale2x
- (cargo-release) start next development iteration 0.1.2-alpha.0
- (cargo-release) version 0.1.1
- Update dependencies
- Fix scale2x algorithm
- Add benchmarks for future improvements
- (cargo-release) start next development iteration 0.1.1-alpha.0
- Fix strict f64 comparison error
- Add upscaled example image
- Fix image reference in README.md
- Improve README.md
- Fix clippy warning
- Update README.md
- First working version of algorithm
- Add rotation for 180 & 270 degrees
- Properly apply scaling in scale2x
- Implement rotate90 as an optimization
- Split into multiple files
- Start implementing rotation algorithm
- Fix clippy warnings
- Refactor scale2x a bit
- Implement scale2x algorithm
- Add example using minifb
- Initial commit
- Initial commit
