use std::collections::HashMap;
use std::io::Cursor;
use quartz_nbt::{NbtCompound, NbtTag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftStructure {
    pub size: [i32; 3],
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub palette: Vec<PaletteEntry>,
    pub blocks: Vec<MinecraftBlock>,
    pub entities: Vec<EntityEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<NbtCompound>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftBlock {
    pub pos: [i32; 3],
    pub state: i32,
    pub nbt: Option<HashMap<String, NbtTag>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityEntry {
    pub pos: [f64; 3],
    #[serde(rename = "blockPos")]
    pub block_pos: [i32; 3],
    pub nbt: HashMap<String, NbtTag>
}

impl TryFrom<Vec<u8>> for MinecraftStructure {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(value);

        let structure: (MinecraftStructure, String) = quartz_nbt::serde::deserialize_from(&mut cursor, quartz_nbt::io::Flavor::GzCompressed).map_err(|_| ())?;

        Ok(structure.0)
    }
}