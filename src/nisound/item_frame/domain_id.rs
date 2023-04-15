//! Domains are a namespace for [`Item`]s. Couple with an [`ItemID`] to find the correct [`Item`].
//!
//! NISound documents use 'NISD' as their domain, and this is the base domain.
//! Kontakt for example adds the 'NIK4' domain.
//!

#[derive(Debug, Clone)]
pub struct DomainID(pub u32);

pub enum Domain {
    NIK4, // 4KIN
    NISD, // DSIN
    Unknown(String),
}

impl From<u32> for Domain {
    fn from(value: u32) -> Self {
        //let id = value.to_le_bytes().to_vec();
        let id = value.to_string();
        match id.as_str() {
            "NISD" => Domain::NISD,
            "NIK4" => Domain::NIK4,
            _ => Domain::Unknown(id),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_domain_id() {
//         Ok(())
//     }
// }
