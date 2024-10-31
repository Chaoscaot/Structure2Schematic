use std::collections::HashMap;
use quartz_nbt::{NbtCompound, NbtTag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpongeSchematic {
    pub data_version: i32,
    pub metadata: NbtCompound,
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub offset: [i32; 3],
    pub palette_max: i32,
    pub palette: HashMap<String, i32>,
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Option<Vec<Entity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntity {
    pub id: String,
    pub pos: [i32; 3],
    #[serde(flatten)]
    pub data: HashMap<String, NbtTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub pos: [i32; 3],
    #[serde(flatten)]
    pub data: HashMap<String, NbtTag>,
}