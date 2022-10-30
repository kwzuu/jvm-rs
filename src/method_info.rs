use crate::attributes::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct MethodInfo {
    pub(crate) access_flags: u16,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes_count: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
}

impl Clone for MethodInfo {
    fn clone(&self) -> Self {
        let mut attrs = vec![];
        self.attributes.clone_into(&mut attrs);
        MethodInfo {
            access_flags: self.access_flags,
            name_index: self.name_index,
            descriptor_index: self.descriptor_index,
            attributes_count: self.attributes_count,
            attributes: attrs,
        }
    }
}
