pub mod formats;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use quartz_nbt::io::Flavor;
    use quartz_nbt::serde::{serialize, Array};
    use crate::formats::schematic::SpongeSchematic;
    use crate::formats::structure::{MinecraftStructure};

    #[test]
    fn save_schem() {
        let schem = SpongeSchematic {
            data_version: 100,
            metadata: Default::default(),
            width: 2,
            height: 1,
            length: 2,
            offset: Array::from(vec![0, 0, 0]),
            palette_max: 2,
            palette: HashMap::from_iter(vec![("minecraft:stone".to_string(), 0), ("minecraft:air".to_string(), 1)]),
            block_data: vec![0, 0, 1, 0],
            block_entities: vec![],
            entities: None,
            version: 2,
        };

        let schem_data = serialize(
            &schem,
            Some("schematic"),
            Flavor::GzCompressed
        ).unwrap();
    }

    #[test]
    fn load_structure() {
        let mut file = File::open("tests/plains_meeting_point_1.nbt").unwrap();
        let (structure, str): (MinecraftStructure, String) = quartz_nbt::serde::deserialize_from(&mut file, Flavor::GzCompressed).unwrap();

        println!("{:?}", structure);
    }

    #[test]
    fn convert_structure() {
        let mut file = File::open("tests/plains_meeting_point_1.nbt").unwrap();
        let (structure, str): (MinecraftStructure, String) = quartz_nbt::serde::deserialize_from(&mut file, Flavor::GzCompressed).unwrap();
        let schem = SpongeSchematic::from(structure);

        let schem_data = serialize(
            &schem,
            Some("schematic"),
            Flavor::GzCompressed
        ).unwrap();

        File::create("tests/out.schem").unwrap().write(&schem_data).unwrap();
    }
}