pub struct User {
    pub name: String,
}

impl User {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}
