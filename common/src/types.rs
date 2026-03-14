#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamKind {
    Register,
    Direct,
    Indirect,
}

#[derive(Clone, Copy, Debug)]
pub enum Param {
    Register(u8),
    Direct(i32),
    Indirect(i16),
}

#[derive(Clone, Debug)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub code: Vec<u8>,
    pub load_address: usize,
}
