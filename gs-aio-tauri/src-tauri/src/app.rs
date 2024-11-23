use std::sync::Arc;

use crate::compression;

pub type RomData = Arc<[u8]>;

const LANGUAGE_ADDRESS: usize = 0xAF;
const VERSION_ADDRESS: usize = 0xA0;
const VERSION_ADDRESS_LENGTH: usize = 15;

#[derive(Default)]
pub struct AppState {
    pub rom_file_path: String,
    pub rom_data: Arc<[u8]>,
    pub decomp_text: Arc<[u8]>,
}

#[derive(Debug, Clone, Copy)]
pub struct GSRomID {
    pub version: GSRomVersion,
    pub language: GSRomRegion,
}

impl GSRomID {
    pub fn from_rom_data(src: &[u8]) -> Result<GSRomID, ()> {
        let language = GSRomRegion::try_from(src)?;
        let version = GSRomVersion::try_from(src)?;

        return Ok(GSRomID { language, version });
    }

    pub fn get_abilities_table_address(&self) -> usize {
        return match (&self.version, &self.language) {
            (_, GSRomRegion::Spanish) => 0xC0C20,
            (_, GSRomRegion::USAEurope) => 0x0B7C14,
            _ => 0,
        };
    }

    pub fn get_text_db_offset(&self) -> usize {
        return match (&self.version, &self.language) {
            (_, GSRomRegion::Spanish) => 461,
            _ => 0,
        };
    }
}

#[derive(Debug)]
pub struct GSRomData {
    pub id: GSRomID,
    pub decomp_text: Arc<[u8]>,
}

#[derive(Debug, Clone, Copy)]
pub enum GSRomRegion {
    Spanish,
    Italian,
    USAEurope,
    Unknown,
}

impl From<&str> for GSRomRegion {
    fn from(value: &str) -> Self {
        let str_val = String::from(value);
        return match str_val.trim().to_lowercase().as_str() {
            "s" => GSRomRegion::Spanish,
            "e" => GSRomRegion::USAEurope,
            rom_language => {
                dbg!(rom_language);
                GSRomRegion::Unknown
            }
        };
    }
}

impl TryFrom<&[u8]> for GSRomRegion {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        return match value.get(LANGUAGE_ADDRESS) {
            Some(val) => std::str::from_utf8(&[*val])
                .map(GSRomRegion::from)
                .map_err(|_| ()),
            None => Err(()),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GSRomVersion {
    BrokenSeal,
    TheLostAge,
    // Not supported yet!
    DarkDawn,
    Unknown,
}

impl From<&str> for GSRomVersion {
    fn from(value: &str) -> Self {
        return match value {
            "Golden_Sun_AAGS" => GSRomVersion::BrokenSeal,
            "GOLDEN_SUN_BAGF" => GSRomVersion::TheLostAge,
            rom_version => {
                dbg!(rom_version);
                GSRomVersion::Unknown
            }
        };
    }
}

impl TryFrom<&[u8]> for GSRomVersion {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(GSRomVersion::from(
            compression::get_string(value, VERSION_ADDRESS, VERSION_ADDRESS_LENGTH).as_str(),
        ))
    }
}
