pub fn convert_to_f32(bytes: &Vec<u8>, start: usize) -> f32 {
    let value = &bytes[start..(start + 4)];
    return f32::from_le_bytes(value.try_into().unwrap());
}

pub fn convert_to_i32(bytes: &Vec<u8>, start: usize) -> i32 {
    let value = &bytes[start..(start + 4)];
    return i32::from_le_bytes(value.try_into().unwrap());
}

pub fn convert_to_i8(bytes: &Vec<u8>, start: usize) -> i8 {
    let value = &bytes[start..(start + 1)];
    return i8::from_le_bytes(value.try_into().unwrap());
}

pub fn convert_to_u32(bytes: &Vec<u8>, start: usize) -> u32 {
    let value = &bytes[start..(start + 4)];
    return u32::from_le_bytes(value.try_into().unwrap());
}

pub fn convert_to_u16(bytes: &Vec<u8>, start: usize) -> u16 {
    let value = &bytes[start..(start + 2)];
    return u16::from_le_bytes(value.try_into().unwrap());
}

pub fn convert_to_u8(bytes: &Vec<u8>, start: usize) -> u8 {
    let value = &bytes[start..(start + 1)];
    return u8::from_le_bytes(value.try_into().unwrap());
}
