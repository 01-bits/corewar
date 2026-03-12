
pub enum Param {
    Register(u8),
    Direct(i32),
    Indirect(i16),
}