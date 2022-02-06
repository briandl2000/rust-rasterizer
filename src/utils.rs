

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32
{
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize
{
    x+y*width
}