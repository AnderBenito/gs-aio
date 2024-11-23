use crate::{
    app::GSRomID,
    compression::{get_int16, get_text_short},
};

#[derive(Debug)]
pub struct PsinergyData {
    pub name: String,
    pub power: u16,
    pub pp_cost: u8,
    pub range: u8,
    pub icon_id: i16,
}

pub fn get_psynergies(
    rom_id: &GSRomID,
    rom_data: &[u8],
    txt: &[u8],
) -> Result<Vec<PsinergyData>, ()> {
    // Get psynergy names from decompressed text
    let names_result: Result<Vec<PsinergyData>, ()> = (0..734)
        .map(|i| -> Result<PsinergyData, ()> { get_psy_data(rom_id, rom_data, txt, i) })
        .collect();

    names_result
}

// 080B7C14 = Ability data/ability table (12 bytes per entry)
//  BYTE = Target
//  BYTE =
//   Left: Flags
//   1 = ?
//   2 = ?
//   4 = Effects can be used outside of battle.
//   8 = Can be used in battle
//   Right: Damage Type
//  BYTE = Element
//  BYTE = Ability Effect
//  SHORT = Icon
//  BYTE = Utility
//  BYTE = Unknown
//  BYTE = Range
//  BYTE = PP cost
//  SHORT = Power
pub fn get_psy_data(
    rom_id: &GSRomID,
    rom_data: &[u8],
    txt: &[u8],
    index: usize,
) -> Result<PsinergyData, ()> {
    let address: usize = rom_id.get_abilities_table_address();
    let psy_name = get_text_short(txt, index + 1447 + rom_id.get_text_db_offset())?;
    let power = get_int16(rom_data, address + 10 + index * 0xC);
    let pp_cost = rom_data.get(address + 9 + index * 0xC).ok_or(())?;
    let range = rom_data.get(address + 8 + index * 0xC).ok_or(())?;
    let icon_id = get_int16(rom_data, address + 4 + index * 0xC);

    Ok(PsinergyData {
        name: psy_name,
        power: power as u16,
        pp_cost: *pp_cost,
        icon_id: icon_id as i16,
        range: *range,
    })
}
