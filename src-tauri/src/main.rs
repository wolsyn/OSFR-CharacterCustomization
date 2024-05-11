// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod database;
mod customization;
mod interface;

use std::{io::Write, path::Path, fs::{create_dir_all, File}};
use log::LevelFilter;
use chrono::Local;
use env_logger::Builder;

use interface::{database_interface::*, customization_interface::*};

fn main() {
    if !Path::new("logs").exists() {
        create_dir_all("logs").unwrap();
    }
    let log_file = File::create("logs/log.txt").unwrap();
  Builder::new()
      .format(|buf, record| {
          writeln!(
              buf,
              "{} :{}: [{}] {}",
              Local::now().format("<%d-%m-%Y %H:%M:%S%.3f>"),
              record.file().unwrap(),
              record.level(),
              record.args()
          )
      })
      .filter(None, LevelFilter::Info)
      .target(env_logger::Target::Pipe(Box::new(log_file)))
      .init();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      eye_color,
      hair_type, 
      hair_color,
      model_extras, 
      facepaint, 
      new_character,
      set_genderace,
      set_facepaint,
      set_eyes,
      set_hair,
      set_skintone,
      set_extras
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
