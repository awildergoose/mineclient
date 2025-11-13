use crate::java::{JByte, JInt};

pub struct LevelGen {
    width: JInt,
    height: JInt,
    depth: JInt,
}

impl LevelGen {
    pub fn new(width: JInt, height: JInt, depth: JInt) -> Self {
        Self {
            width,
            depth,
            height,
        }
    }

    pub fn generate_map(&self) -> Vec<JByte> {
        // TODO stub
        Vec::new()
    }
}
