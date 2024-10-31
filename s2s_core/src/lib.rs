pub mod formats;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use quartz_nbt::io::Flavor;
    use quartz_nbt::serde::serialize;
    use crate::formats::schematic::SpongeSchematic;

    #[test]
    fn it_works() {
        let schem = SpongeSchematic {
            data_version: 100,
            metadata: Default::default(),
            width: 2,
            height: 1,
            length: 2,
            offset: [0, 0, 0],
            palette_max: 2,
            palette: HashMap::from_iter(vec![("minecraft:stone".to_string(), 0), ("minecraft:air".to_string(), 1)]),
            block_data: vec![0, 0, 1, 0],
            block_entities: vec![],
            entities: None,
        };

        let schem_data = serialize(
            &schem,
            Some("schematic"),
            Flavor::GzCompressed
        ).unwrap();

        println!("{:?}", schem_data);
    }
}