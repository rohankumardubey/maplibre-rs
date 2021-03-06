use std::path::{Path, PathBuf};
use std::{env, fs};

use mbtiles::extract;
use wgsl_validate::validate_project_wgsl;

const MUNICH_X: u32 = 17425;
const MUNICH_Y: u32 = 11365;
const MUNICH_Z: u8 = 15;

/// Tiles which can be used by StaticTileFetcher
fn clean_static_tiles() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    let out = Path::new(&out_dir).join("extracted-tiles");

    if out.exists() && out.is_dir() {
        fs::remove_dir_all(&out).unwrap()
    }

    out
}

fn embed_tiles_statically() {
    let out = clean_static_tiles();

    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let source = Path::new(&root_dir).join(format!("test-data/munich-{}.mbtiles", MUNICH_Z));

    if source.exists() {
        println!("cargo:rustc-cfg=static_tiles");
        // Pack tiles around Munich HBF (100 tiles in each direction)
        extract(
            source,
            out,
            MUNICH_Z,
            (MUNICH_X - 2)..(MUNICH_X + 2),
            (MUNICH_Y - 2)..(MUNICH_Y + 2),
        )
        .unwrap();
    } else {
        // Do not statically embed tiles
    }
}

fn main() {
    validate_project_wgsl();

    embed_tiles_statically();
}
