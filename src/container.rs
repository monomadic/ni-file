//!  Container
//!
//!  An abstracted model of a container file.
//!

type ContainerKind = crate::ni_segment::SegmentType;

#[derive(Debug)]
pub struct Container {
    pub kind: ContainerKind,
    pub uuid: [u8; 16],
    pub data: Vec<u8>,
    pub object: Object,
    pub children: Vec<Container>,
}

#[derive(Debug)]
pub struct Object {
    pub kind: ContainerKind,
}

impl From<crate::ni_repository::Repository> for Container {
    fn from(r: crate::ni_repository::Repository) -> Self {
        let data = r.data().unwrap(); // TODO: remove unwrap

        Container {
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
