#[derive(Debug, Clone)]
pub struct DomainID(pub u32);

pub enum DomainType {
    BNIS,    // 4KIN
    NISound, // DSIN
}
