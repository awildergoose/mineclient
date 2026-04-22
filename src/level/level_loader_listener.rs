pub trait LevelLoaderListener {
    fn begin_level_loading(&self, status: &str);
    fn level_load_update(&self, status: &str);
}
