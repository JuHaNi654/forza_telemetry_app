pub fn convert_to_f32(bytes: &Vec<u8>, start: usize) -> f32 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(4).collect();
    let mut slice: [u8; 4] = [Default::default(); 4];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return f32::from_le_bytes(slice);
}


pub fn convert_to_i32(bytes: &Vec<u8>, start: usize) -> i32 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(4).collect();
    let mut slice: [u8; 4] = [Default::default(); 4];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return i32::from_le_bytes(slice);
}

pub fn convert_to_i8(bytes: &Vec<u8>, start: usize) -> i8 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(1).collect();
    let mut slice: [u8; 1] = [Default::default(); 1];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return i8::from_le_bytes(slice);
}

pub fn convert_to_u32(bytes: &Vec<u8>, start: usize) -> u32 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(4).collect();
    let mut slice: [u8; 4] = [Default::default(); 4];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return u32::from_le_bytes(slice);
}

pub fn convert_to_u16(bytes: &Vec<u8>, start: usize) -> u16 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(2).collect();
    let mut slice: [u8; 2] = [Default::default(); 2];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return u16::from_le_bytes(slice);
}

pub fn convert_to_u8(bytes: &Vec<u8>, start: usize) -> u8 {
    let value: Vec<&u8> = bytes.iter().skip(start).take(1).collect();
    let mut slice: [u8; 1] = [Default::default(); 1];
    value.iter().zip(slice.iter_mut()).map(|(&x, y)| *y = *x).count();
    return u8::from_le_bytes(slice);
}

