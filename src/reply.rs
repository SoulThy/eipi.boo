#[derive(Clone, Debug)]
pub struct Reply {
    pub text: String,
    pub name: String,
}

pub const MAX_LENGTH: usize = 100;
