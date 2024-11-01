use std::collections::HashMap;
use convert_case::{Case, Casing};
use quartz_nbt::{NbtCompound, NbtTag};
use quartz_nbt::io::NbtIoError;
use quartz_nbt::serde::Array;
use serde::{Deserialize, Serialize, Serializer};
use crate::formats::structure::MinecraftStructure;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeSchematic {
    pub data_version: i32,
    pub metadata: NbtCompound,
    pub width: i16,
    pub height: i16,
    pub length: i16,
    pub offset: Array<Vec<i32>>,
    pub palette_max: i32,
    pub palette: HashMap<String, i32>,
    #[serde(serialize_with = "vec_i32_as_varint")]
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Option<Vec<Entity>>,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntity {
    pub id: String,
    pub pos: Array<Vec<i32>>,
    #[serde(flatten)]
    pub data: HashMap<String, NbtTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    #[serde(rename = "Pos")]
    pub pos: Array<Vec<i32>>,
    #[serde(flatten)]
    pub data: HashMap<String, NbtTag>,
}

impl From<MinecraftStructure> for SpongeSchematic {
    fn from(value: MinecraftStructure) -> Self {
        let width = value.size[0];
        let height = value.size[1];
        let length = value.size[2];
        let mut palette: HashMap<String, i32> = value.palette.iter().enumerate().map(|(index, value)| {
            if let Some(nbt) = value.properties.clone() {
                (format!("{}[{}]", value.name, nbt.into_iter().map(|(name, value)| format!("{}={}", name, remove_prefix_suffix_if_present(value.to_string(), "\""))).reduce(|a, b| format!("{},{}", a, b)).unwrap()), index as i32 + 1)
            } else {
                (value.name.clone(), index as i32 + 1)
            }
        }).collect();
        palette.insert("minecraft:air".to_string(), 0);
        let mut block_data: Vec<i32> = Vec::with_capacity(value.blocks.len());
        block_data.resize((width * height * length) as usize, 0);
        let mut block_entities: Vec<BlockEntity> = Vec::new();

        for block in value.blocks {
            let x = block.pos[0];
            let y = block.pos[1];
            let z = block.pos[2];
            let index = (x + z * width + y * width * length) as usize;
            block_data[index] = block.state + 1;

            if let Some(nbt) = block.nbt {
                block_entities.push(BlockEntity {
                    pos: Array::from(Vec::from(block.pos)),
                    id: value.palette[block.state as usize].name.clone(),
                    data: nbt.iter().map(|(name, value)| (name.clone().to_case(Case::Pascal), value.clone())).collect(),
                })
            }
        }

        SpongeSchematic {
            data_version: value.data_version,
            metadata: NbtCompound::from_iter(vec![
                ("ConvertedBy".to_string(), NbtTag::from("Structure2Schematic")),
                ("GitHub".to_string(), NbtTag::from("https://github.com/Chaoscaot/Structure2Schematic")),
                ("Author".to_string(), NbtTag::from("Chaoscaot")),
            ]),
            width: width as i16,
            height: height as i16,
            length: length as i16,
            offset: Array::from(vec![0, 0, 0]),
            palette_max: palette.len() as i32,
            palette,
            block_data,
            block_entities,
            entities: None,
            version: 2,
        }
    }
}

impl SpongeSchematic {
    pub fn to_bytes(&self) -> Result<Vec<u8>, NbtIoError> {
        quartz_nbt::serde::serialize(&self, Some("Schematic"), quartz_nbt::io::Flavor::GzCompressed)
    }
}

fn vec_i32_as_varint<S>(val: &Vec<i32>, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let bytes = write_varint_array(val);
    ser.serialize_bytes(&bytes[..])
}

fn remove_prefix_suffix_if_present(string: String, prefix: &str) -> String {
    if string.starts_with(prefix) && string.ends_with(prefix) {
        string[prefix.len()..string.len() - prefix.len()].to_string()
    } else {
        string
    }
}

#[inline]
pub fn read_varint_array(read: &Vec<i8>) -> Vec<i32> {
    let mut data = Vec::new();
    let mut value: i32 = 0;
    let mut position = 0;
    let mut current_byte;
    let mut cursor = 0;
    loop {
        match read.get(cursor) {
            Some(byte) => { current_byte = *byte as u8; cursor += 1; },
            None => break,
        };

        value |= (((current_byte & 0x7F) as u32) << position) as i32;

        if(current_byte & 0x80) == 0 {
            data.push(value);
            value = 0;
            position = 0;
        } else {
            position += 7;

            if position > 32 {
                panic!("VarInt too big");
            }
        }
    }
    data
}

#[inline]
pub fn write_varint_array(data: &Vec<i32>) -> Vec<u8> {
    let mut write = Vec::new();
    for value in data {
        let mut value = *value as u32;
        loop {
            let mut current_byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                current_byte |= 0x80;
            }

            write.push(current_byte);

            if value == 0 {
                break;
            }
        }
    }
    write
}