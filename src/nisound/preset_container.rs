/// Represents a repository file. Usually has a `RepositoryRoot` as the first enclosing `Item`.
pub struct NIPresetContainer(Item);

impl NIPresetContainer {
    pub fn read<R: ReadBytesExt>(reader: R) -> Result<Self> {
        log::debug!("NIPresetContainer::read()");
        Ok(Self(Item::read(reader)?))
    }
}
