#[derive(Debug)]
pub struct AttributeInfo {
    pub name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}

impl Clone for AttributeInfo {
    fn clone(&self) -> Self {
        let mut info = Vec::with_capacity(self.info.len());
        for i in &self.info {
            info.push(*i)
        }
        AttributeInfo {
            name_index: self.name_index,
            attribute_length: self.attribute_length,
            info,
        }
    }
}
