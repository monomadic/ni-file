use crate::ni_repository::ItemFrame;

/**
 * Container
 * simplified, rust-like version of NIContainers
 */

type PropertyKind = crate::ni_segment::SegmentType;

#[derive(Debug)]
pub struct Container {
    uuid: [u8; 16],
    properties: Vec<Property>,
    children: Vec<Container>,
}

impl From<crate::ni_repository::Repository> for Container {
    fn from(r: crate::ni_repository::Repository) -> Self {
        Container {
            uuid: r.uuid,
            properties: vec![],
            children: r
                .children
                .iter()
                .map(|frame| frame.repository.clone().into())
                .collect(),
        }
    }
}

// impl From<ItemFrame> for

#[derive(Debug)]
pub struct Property {
    kind: PropertyKind,
    data: Vec<u8>,
}
