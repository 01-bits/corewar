use crate::constants::MEM_SIZE;
 // red  forom file >>>
    pub fn read_u32_be(bytes: &[u8], offset: usize) -> Option<u32> {
        let chunk: &[u8] = bytes.get(offset..offset + 4)?;
        Some(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    }

pub fn read_padded_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).to_string()
}
// for arena by circular>>
pub fn wrap_address(address: i32) -> usize {
    let size = i32::from(MEM_SIZE);
    address.rem_euclid(size) as usize
}

pub fn address_from_offset(base: usize, offset: i32) -> usize {
    wrap_address(base as i32 + offset)
}

pub fn read_i16_at(arena: &[u8; MEM_SIZE as usize], start: usize) -> i16 {
    i16::from_be_bytes([arena[start], arena[wrap_address(start as i32 + 1)]])
}

pub fn read_i32_at(arena: &[u8; MEM_SIZE as usize], start: usize) -> i32 {
    i32::from_be_bytes([
        arena[start],
        arena[wrap_address(start as i32 + 1)],
        arena[wrap_address(start as i32 + 2)],
        arena[wrap_address(start as i32 + 3)],
    ])
}

pub fn write_i32_at(arena: &mut [u8; MEM_SIZE as usize], start: usize, value: i32) {
    let bytes = value.to_be_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        arena[wrap_address(start as i32 + index as i32)] = *byte;
    }
}
