pub trait Parse: Sized {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>>;
}

impl Parse for i32 {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(crate::data_types::var_int::read_var_int(bytes, index)?)
    }
}

impl Parse for String {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        crate::data_types::string::read_string(bytes, index)
    }
}

impl Parse for u16 {
    fn parse(bytes: &[u8], index: &mut usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(crate::data_types::unsigned_short::read_unsigned_short(
            bytes, index,
        ))
    }
}
