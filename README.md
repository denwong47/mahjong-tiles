# Rust Structs for Mahjong Tiles with SVG graphics
This repository contains vector graphics and PNG exports of riichi mahjong tiles. The tiles are available in the regular and black variants. Below are example screenshots of the tiles.

<div style="text-align:center">
<img src ="https://raw.githubusercontent.com/FluffyStuff/riichi-mahjong-tiles/master/ExampleRegular.png" />
<img src ="https://raw.githubusercontent.com/FluffyStuff/riichi-mahjong-tiles/master/ExampleBlack.png" />
</div>

## Rust Structs

The Mahjong tiles are exposed as the struct `MahjongTile` in the `mahjong` module:

```rust
use mahjong_tiles::{MahjongTile, TileCategory, IsMahjongTile, HonourCategory, HasSvgData, SvgStyle};

let tile = MahjongTile::TONG(1);

assert_eq!(tile.category(), TileCategory::TONG);
assert_eq!(tile.value(), Some(1));
assert_eq!(tile.chinese_name(), "一筒");
assert_eq!(tile + 1, Some(MahjongTile::TONG(2)));

let svg = tile.svg_data(mahjong_tiles::SvgStyle::DARK).unwrap();
assert!(svg.starts_with("data:image/svg+xml;base64,"));

let tile = MahjongTile::WAN(9);

assert_eq!(tile.category(), TileCategory::WAN);
assert_eq!(tile.value(), Some(9));
assert_eq!(tile.chinese_name(), "九万");
assert_eq!(tile - 1, Some(MahjongTile::WAN(8)));

let tile = MahjongTile::EAST;

assert_eq!(tile.category(), TileCategory::HONOUR(HonourCategory::WIND));
```


## License
All assets are in the [public domain](https://creativecommons.org/publicdomain/zero/1.0/).
