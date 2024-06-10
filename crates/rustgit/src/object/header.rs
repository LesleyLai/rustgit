use crate::object::{ObjectReadError, ObjectType};
use crate::object_reader::ObjectReader;
use crate::parse_utils::parse_usize;
use crate::utils::remove_last;

use std::io::BufRead;

pub struct ObjectHeader {
    pub typ: ObjectType,
    pub size: usize,
}

pub fn read_header(reader: &mut ObjectReader) -> Result<ObjectHeader, ObjectReadError> {
    let mut output = vec![];
    reader
        .read_until(0, &mut output)
        .map_err(|err| ObjectReadError::HeaderReadError(err))?;

    parse_header(&output)
}

fn parse_header(buffer: &[u8]) -> Result<ObjectHeader, ObjectReadError> {
    let separate_point = buffer.iter().position(|&c| c == b' ');
    let separate_point = separate_point.ok_or(ObjectReadError::MissingSpaceSeparator)?;

    let (typ, mut size) = remove_last(&buffer).split_at(separate_point);
    let typ = ObjectType::parse(typ).ok_or(ObjectReadError::UnknownObjectType)?;
    size = &size[1..];
    let size: usize = parse_usize(size)?;

    Ok(ObjectHeader { typ, size })
}
