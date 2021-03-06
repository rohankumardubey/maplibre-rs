use serde::{Deserialize, Serialize};

pub type TileUrl = String;

pub type TileJSONUrl = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TileAddressingScheme {
    #[serde(rename = "xyz")]
    XYZ,
    #[serde(rename = "tms")]
    TMS,
}

impl Default for TileAddressingScheme {
    fn default() -> Self {
        TileAddressingScheme::XYZ
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VectorSource {
    /// String which contains attribution information for the used tiles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// The bounds in which tiles are available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<(f64, f64, f64, f64)>,
    /// Max zoom level at which tiles are available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxzoom: Option<u8>,
    /// Min zoom level at which tiles are available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minzoom: Option<u8>,
    // TODO: promoteId
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<TileAddressingScheme>,
    /// Array of URLs which can contain place holders like {x}, {y}, {z}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<TileUrl>,
    // url: Option<TileJSONUrl>,
    // TODO volatile
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Source {
    #[serde(rename = "vector")]
    Vector(VectorSource),
    #[serde(rename = "raster")]
    Raster(VectorSource),
}
