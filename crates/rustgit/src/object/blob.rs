use crate::object::{Object, ObjectBuffer, ObjectType};

/// In memory data representation of a git blob object
pub struct Blob {
    content: Box<[u8]>,
}

impl Blob {
    pub fn new(content: Box<[u8]>) -> Self {
        Blob { content }
    }
}

impl Object for Blob {
    fn to_buffer(&self) -> ObjectBuffer {
        ObjectBuffer::new(ObjectType::Blob, &self.content)
    }
}
