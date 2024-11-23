use super::error::Result;
use crate::{compression::decomp_text, game::get_psynergies};
use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::app::{AppState, GSRomID};

#[tauri::command]
pub fn load_rom(state: tauri::State<Mutex<AppState>>, file_path: String) -> Result<()> {
    println!("File path is {}", file_path);
    state.lock().unwrap().rom_file_path = file_path.clone();

    let mut file = File::open(Path::new(file_path.as_str()))?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    println!("ROM loaded");

    let rom_data = Arc::<[u8]>::from(buffer.into_boxed_slice());

    let decompressed_text = decomp_text(&rom_data)?;
    let txt = Arc::<[u8]>::from(decompressed_text.into_boxed_slice());

    let gs_rom_id = GSRomID::from_rom_data(&rom_data).unwrap();

    {
        let mut s = state.lock().unwrap();
        s.rom_data = rom_data.clone();
        s.decomp_text = txt.clone();
    }

    if let Ok(psynergies) = get_psynergies(&gs_rom_id, &rom_data, &txt) {
        dbg!(psynergies);
    }

    Ok(())
}
