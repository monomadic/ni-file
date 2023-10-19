// use crate::Error;
//
// use super::KontaktObject;

/// A trait for traversal of a Kontakt object tree.
pub trait KontaktNode {
    fn name(&self) -> String;
    fn children(&self) -> Vec<Box<dyn KontaktNode>>;
    // fn to_object(&self) -> Result<KontaktNode, Error>;
}
