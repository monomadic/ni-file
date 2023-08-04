use crate::Error;

use super::{
    filename_list::FileNameListPreK51, objects::bprogram::BProgram,
    structured_object::StructuredObject,
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum StructuredObjectData {
    Program(BProgram),
    FileNameListPreK51(FileNameListPreK51),
    UnsupportedType(u16),
}

impl TryFrom<StructuredObject> for StructuredObjectData {
    type Error = Error;

    fn try_from(so: StructuredObject) -> Result<Self, Self::Error> {
        Ok(match so.id {
            0x28 => StructuredObjectData::Program(BProgram::try_from(so)?),
            // 0x3D => {
            //     StructuredObjectData::FileNameListPreK51(FileNameListPreK51::read(&mut reader)?)
            // }
            _ => StructuredObjectData::UnsupportedType(so.id),
        })
    }
}
