use std::sync::Arc;

use javarandom::JavaRandom;

use crate::{
    java::{get_milli_time, JByte, JInt},
    level::{level::Level, level_loader_listener::LevelLoaderListener},
};

pub struct LevelGen {
    level_loader_listener: Arc<dyn LevelLoaderListener>,
    width: JInt,
    height: JInt,
    depth: JInt,
    random: JavaRandom,
    blocks: Vec<JByte>,
    coords: Vec<JInt>,
}

impl LevelGen {
    pub fn new(level_loader_listener: Arc<dyn LevelLoaderListener>) -> Self {
        Self {
            level_loader_listener,
            width: 0,
            height: 0,
            depth: 0,
            blocks: vec![],
            coords: vec![0; 1_048_576],
            random: JavaRandom::new(),
        }
    }

    pub fn generate_level(
        &mut self,
        user_name: String,
        width: JInt,
        height: JInt,
        depth: JInt,
    ) -> Level {
        self.level_loader_listener
            .begin_level_loading("Generating level");
        self.width = width;
        self.height = height;
        self.depth = depth;
        self.blocks = vec![0; (width * height * depth) as usize];
        self.level_loader_listener.level_load_update("Raising..");
        // heightmap
        self.level_loader_listener.level_load_update("Eroding..");
        // buildBlocks
        self.level_loader_listener.level_load_update("Carving..");
        // carveTunnels
        self.level_loader_listener.level_load_update("Watering..");
        // addWater
        self.level_loader_listener.level_load_update("Melting..");
        // addLava

        let mut level = Level::set_data(width, depth, height, self.blocks.clone());
        level.create_time = get_milli_time();
        level.creator = user_name;
        "A Nice World".clone_into(&mut level.name);
        level
    }
}
