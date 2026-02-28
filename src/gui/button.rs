use crate::java::JInt;

pub struct Button {
    pub x: JInt,
    pub y: JInt,
    pub w: JInt,
    pub h: JInt,
    pub msg: String,
    pub id: JInt,
}

impl Button {
    #[must_use]
    pub const fn new(id: JInt, x: JInt, y: JInt, w: JInt, h: JInt, msg: String) -> Self {
        Self {
            x,
            y,
            w,
            h,
            msg,
            id,
        }
    }
}
