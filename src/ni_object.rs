//!  NIObject
//!
//!  An abstracted model of a container file.
//!

pub use std::convert::TryInto;

use crate::prelude::*;

type ContainerKind = crate::kinds::SegmentType;

#[derive(Debug)]
pub struct NIObject {
    pub kind: ContainerKind,
    pub uuid: [u8; 16],
    pub data: Vec<u8>,
    pub object: Object,
    pub children: Vec<NIObject>,
}

#[derive(Debug)]
pub struct Object {
    pub kind: ContainerKind,
}

impl TryInto<NIObject> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<NIObject, Self::Error> {
        self.as_slice().try_into()
    }
}

impl TryInto<NIObject> for &[u8] {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<NIObject, Self::Error> {
        crate::raw_repository::Repository::read(self).map(NIObject::from)
    }
}

impl From<crate::raw_repository::Repository> for NIObject {
    fn from(r: crate::raw_repository::Repository) -> Self {
        let data = r.data().unwrap(); // TODO: remove unwrap

        NIObject {
            kind: ContainerKind::default(), // TODO: parse
            uuid: r.uuid,
            data: vec![],
            object: Object {
                kind: data.item_id.into(),
            },
            children: r
                .children
                .iter()
                .map(|frame| frame.repository.clone().into())
                .collect(),
        }
    }
}
