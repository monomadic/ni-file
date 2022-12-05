/**
 * Container
 * simplified, rust-like version of NIContainers
 */

type ContainerKind = crate::ni_segment::SegmentType;

#[derive(Debug)]
pub struct Container {
    kind: ContainerKind,
    uuid: [u8; 16],
    data: Vec<u8>,
    object: Object,
    children: Vec<Container>,
}

#[derive(Debug)]
pub struct Object {
    kind: ContainerKind,
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
