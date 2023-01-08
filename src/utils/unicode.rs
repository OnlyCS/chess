use std::error::Error;

// a function that takes in a unicode hex code and returns the corresponding character
pub fn unicode_from_hex(hex: &str) -> Result<char, Box<dyn Error>> {
    let code = u32::from_str_radix(hex, 16)?;
    let chr = std::char::from_u32(code).ok_or("Invalid unicode code")?;
    Ok(chr)
}
