use std::{
    fs::File,
    io::{Cursor, Read, Write},
    sync::Arc,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    java::JInt,
    level::{level::Level, level_loader_listener::LevelLoaderListener},
};

static MAGIC_NUMBER: JInt = 656_127_880;
static CURRENT_VERSION: u8 = 1;

pub struct LevelIo {
    level_loader_listener: Arc<dyn LevelLoaderListener>,
    pub error: Option<String>,
}

fn read_java_utf<R: Read>(reader: &mut R) -> std::io::Result<String> {
    let len = reader.read_u16::<BigEndian>()? as usize;
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf).unwrap())
}

fn write_java_utf<W: Write>(writer: &mut W, s: &str) -> std::io::Result<()> {
    let bytes = s.as_bytes();

    if bytes.len() > u16::MAX as usize {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "string too long",
        ));
    }

    writer.write_u16::<BigEndian>(bytes.len() as u16)?;
    writer.write_all(bytes)?;

    Ok(())
}

impl LevelIo {
    pub fn new(level_loader_listener: Arc<dyn LevelLoaderListener>) -> Self {
        Self {
            level_loader_listener,
            error: None,
        }
    }

    pub fn load(&mut self, mut file: &File) -> anyhow::Result<Level> {
        self.level_loader_listener
            .begin_level_loading("Loading level");
        self.level_loader_listener.level_load_update("Reading..");

        let mut dis = Vec::new();
        if let Err(e) = file.read_to_end(&mut dis) {
            self.error = Some(format!("Failed to load level: {e}"));
            anyhow::bail!("Failed to load level: {e}");
        }

        let mut cur = Cursor::new(dis);
        let magic = cur.read_i32::<BigEndian>().unwrap();
        if magic != MAGIC_NUMBER {
            self.error = Some("Bad level file format".to_owned());
            anyhow::bail!("Bad level file format");
        }

        let version = cur.read_u8().unwrap();
        if version > CURRENT_VERSION {
            self.error = Some("Bad level file format".to_owned());
            anyhow::bail!("Bad level file format");
        }

        let name = read_java_utf(&mut cur).unwrap();
        let creator = read_java_utf(&mut cur).unwrap();
        let create_time = cur.read_i64::<BigEndian>().unwrap();
        let width = cur.read_i16::<BigEndian>().unwrap();
        let height = cur.read_i16::<BigEndian>().unwrap();
        let depth = cur.read_i16::<BigEndian>().unwrap();
        let mut blocks_u8 = vec![0u8; (width * height * depth) as usize];
        cur.read_exact(&mut blocks_u8).unwrap();
        let blocks: Vec<i8> = blocks_u8.into_iter().map(|b| b as i8).collect();
        let mut level = Level::set_data(
            i32::from(width),
            i32::from(depth),
            i32::from(height),
            blocks,
        );
        level.name = name;
        level.creator = creator;
        level.create_time = create_time;
        Ok(level)
    }

    pub fn load_legacy(&mut self, mut file: &File) -> anyhow::Result<Level> {
        self.level_loader_listener
            .begin_level_loading("Loading level");
        self.level_loader_listener.level_load_update("Reading..");

        let mut dis = Vec::new();
        if let Err(e) = file.read_to_end(&mut dis) {
            self.error = Some(format!("Failed to load level: {e}"));
            anyhow::bail!("Failed to load level: {e}");
        }

        let mut cur = Cursor::new(dis);
        let name = "---".to_owned();
        let creator = "unknown".to_owned();
        let create_time = 0;
        let width = 256;
        let height = 256;
        let depth = 64;
        let mut blocks_u8 = vec![0u8; (width * height * depth) as usize];
        cur.read_exact(&mut blocks_u8)?;
        let blocks: Vec<i8> = blocks_u8.into_iter().map(|b| b as i8).collect();
        let mut level = Level::set_data(width, depth, height, blocks);
        level.name = name;
        level.creator = creator;
        level.create_time = create_time;
        Ok(level)
    }

    pub fn save(&self, level: Level, mut file: &File) -> anyhow::Result<()> {
        let mut cur = Cursor::new(Vec::new());
        cur.write_i32::<BigEndian>(MAGIC_NUMBER)?;
        cur.write_u8(CURRENT_VERSION)?;
        write_java_utf(&mut cur, &level.name)?;
        write_java_utf(&mut cur, &level.creator)?;
        cur.write_i64::<BigEndian>(level.create_time)?;
        cur.write_i16::<BigEndian>(level.width as i16)?;
        cur.write_i16::<BigEndian>(level.height as i16)?;
        cur.write_i16::<BigEndian>(level.depth as i16)?;
        let blocks: Vec<u8> = level.blocks.as_slice().iter().map(|b| *b as u8).collect();
        cur.write_all(&blocks)?;
        let mut buf = Vec::new();
        cur.read_to_end(&mut buf)?;
        file.write_all(&buf)?;

        Ok(())
    }
}
