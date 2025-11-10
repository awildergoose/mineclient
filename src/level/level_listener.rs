use crate::java::JInt;

pub trait LevelListener {
    fn tile_changed(&self, var1: JInt, var2: JInt, var3: JInt);
    fn light_column_changed(&self, var1: JInt, var2: JInt, var3: JInt, var4: JInt);
    fn all_changed(&self);
}
