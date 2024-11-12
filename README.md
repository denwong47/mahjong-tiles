# Rust Structs for Mahjong Tiles with SVG graphics

![Rust CI](https://github.com/denwong47/mahjong-tiles/actions/workflows/rust-CI.yml/badge.svg?branch=main)

This repository contains Rust structs defining Chinese version of Mahjong tiles, together with vector graphics encoded as base64 strings. The tiles are available in the regular and black variants. Below are example screenshots of the tiles.

<div style="text-align:center">
<img src ="https://raw.githubusercontent.com/FluffyStuff/riichi-mahjong-tiles/master/ExampleRegular.png" />
<img src ="https://raw.githubusercontent.com/FluffyStuff/riichi-mahjong-tiles/master/ExampleBlack.png" />
</div>

## Rust Structs

The Mahjong tiles are exposed as the struct `Tile` in the `mahjong` module:

```rust
use mahjong_tiles::{Tile, TileCategory, IsTile, HonourCategory, HasSvgData, SvgStyle};

let tile = Tile::TONG(1);

assert_eq!(tile.category(), TileCategory::TONG);
assert_eq!(tile.value(), Some(1));
assert_eq!(tile.chinese_name(), "一筒");
assert_eq!(tile + 1, Some(Tile::TONG(2)));

let svg = tile.svg_data(mahjong_tiles::SvgStyle::DARK).unwrap();
assert!(svg.starts_with("data:image/svg+xml;base64,"));

let tile = Tile::WAN(9);

assert_eq!(tile.category(), TileCategory::WAN);
assert_eq!(tile.value(), Some(9));
assert_eq!(tile.chinese_name(), "九万");
assert_eq!(tile - 1, Some(Tile::WAN(8)));

let tile = Tile::EAST;

assert_eq!(tile.category(), TileCategory::HONOUR(HonourCategory::WIND));
```


## License
All assets are in the [public domain](https://creativecommons.org/publicdomain/zero/1.0/).
