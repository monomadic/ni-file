/// NI app the preset was created in.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum AuthoringApplication {
    GuitarRig,
    Kontakt,
    Kore,
    Reaktor,
    Maschine,
    Absynthe,
    Massive,
    FM8,
    Battery,
    KKontrol,
    SC,
    FXF_01,
    FXF_02,
    FXF_03,
    FXF_04,
    FXF_05,
    FXF_06,
    FXF_07,
    FXF_08,
    FXF_09,
    FXF_10,
    FXF_11,
    FXF_12,
    FXF_13,
    FXF_14,
    FXF_15,
    FXF_16,
    FXF_17,
    FXF_18,
    FXF_19,
    Traktor,
    Unknown(u32),
}

impl From<u32> for AuthoringApplication {
    fn from(app_id: u32) -> Self {
        match app_id {
            1 => AuthoringApplication::GuitarRig,
            2 => AuthoringApplication::Kontakt,
            3 => AuthoringApplication::Kore,
            4 => AuthoringApplication::Reaktor,
            5 => AuthoringApplication::Maschine,
            6 => AuthoringApplication::Absynthe,
            7 => AuthoringApplication::Massive,
            8 => AuthoringApplication::FM8,
            9 => AuthoringApplication::Battery,
            10 => AuthoringApplication::KKontrol,
            11 => AuthoringApplication::SC,
            12 => AuthoringApplication::FXF_01,
            13 => AuthoringApplication::FXF_02,
            14 => AuthoringApplication::FXF_03,
            15 => AuthoringApplication::FXF_04,
            16 => AuthoringApplication::FXF_05,
            17 => AuthoringApplication::FXF_06,
            18 => AuthoringApplication::FXF_07,
            19 => AuthoringApplication::FXF_08,
            20 => AuthoringApplication::FXF_09,
            21 => AuthoringApplication::FXF_10,
            22 => AuthoringApplication::FXF_11,
            23 => AuthoringApplication::FXF_12,
            24 => AuthoringApplication::FXF_13,
            25 => AuthoringApplication::FXF_14,
            26 => AuthoringApplication::FXF_15,
            27 => AuthoringApplication::FXF_16,
            28 => AuthoringApplication::FXF_17,
            29 => AuthoringApplication::FXF_18,
            30 => AuthoringApplication::FXF_19,
            31 => AuthoringApplication::Traktor,
            _ => AuthoringApplication::Unknown(app_id),
        }
    }
}
