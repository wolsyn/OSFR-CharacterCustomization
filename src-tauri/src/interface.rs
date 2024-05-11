use crate::database::*;
use crate::customization::*;
use log::error;

const DATABASE_PATH: &str = "database/customization.db";


pub fn open_dir() {
  #[cfg(target_os = "windows")]{
    std::process::Command::new("explorer")
            .arg("characters")
            .spawn()
            .expect("Failed to open folder in explorer");
  }
  #[cfg(target_os = "linux")] {
    std::process::Command::new("xdg-open")
            .arg("characters")
            .spawn()
            .expect("Failed to open folder in file manager");
  }
}

pub mod database_interface {
  use super::*;
  #[tauri::command]
  pub fn hair_type(gender: String) -> Vec<Hair> {
    let hairs: Vec<Hair> = match get_hairs(DATABASE_PATH, &gender) {
        Ok(h) => h,
        Err(e) => {
          eprintln!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          error!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          panic!();
        },
    };
    hairs
  }

  #[tauri::command]
  pub fn hair_color() -> Vec<HairColor>{
    match get_hair_color(DATABASE_PATH) {
      Ok(haircolors) => haircolors,
      Err(e) => {
        eprintln!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
        error!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
        panic!();
      },
    }
  }

  #[tauri::command]
  pub fn eye_color() -> Vec<EyeColor> {
    match get_eye_color(DATABASE_PATH) {
        Ok(eyecolors) => eyecolors,
        Err(e) => {
          eprintln!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          error!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          panic!();
        },
    }
  }

  #[tauri::command]
  pub fn model_extras(gender: String, species: String) -> Vec<Extras> {
    let model_extras: Vec<Extras> = match get_extras_by_gender_species(DATABASE_PATH, &gender, &species) {
        Ok(pw) => {pw},
        Err(e) => {
          eprintln!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          error!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
          panic!();
        },
    };
  model_extras
}

#[tauri::command]
pub fn facepaint() -> Vec<FacePaint> {
  let face_paints: Vec<FacePaint> = match get_facepaints(DATABASE_PATH) {
    Ok(fp) => fp,
    Err(e) => {
      eprintln!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
      error!("Error while retrieving data from Database file {}, due to {:#?}", DATABASE_PATH, &e);
      panic!();
    },
  };
  face_paints
}
}

pub mod customization_interface {
  use super::*;
  #[tauri::command]
  pub async fn new_character(username: String, surname: String){
    match crate::customization::new_character(&username, &surname).await {
        Ok(_) => (),
        Err(e) => {
          eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
          error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        },
    };
  }

  #[tauri::command]
  pub async fn set_genderace(username: String, surname: String, genderrace: u8) {
    match modify_gender(&username, &surname ,genderrace).await {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
      },
    };
  }

  #[tauri::command]
  pub async fn set_eyes(username: String, surname: String, color: usize) { 
    match modify_eyes(&username, &surname ,color).await {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
      },
    }
  }

  #[tauri::command]
  pub async fn set_hair(username: String, surname: String, hairtype: String, haircolor: usize) {
    match modify_hair(&username, &surname , &hairtype,haircolor).await {
        Ok(_) => (),
        Err(e) => {
          eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
          error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        },
    };
  }

  #[tauri::command]
  pub async fn set_skintone(username: String, surname: String, newskintone: String) {
    match modify_skintone(&username, &surname , &newskintone).await {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
      },
    };
  }

  #[tauri::command]
  pub async fn set_extras(username: String, surname: String, extra: String) {
    match modify_extras(&username, &surname , &extra).await {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
      },
    };
    open_dir();
  }

  #[tauri::command]
  pub async fn set_facepaint(username: String, surname: String, facepaint: String) {
    match modify_facepaint(&username, &surname , &facepaint).await {
      Ok(_) => (),
      Err(e) => {
        eprintln!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
        error!("Error ocurred while reading/writting to Character file named {}{}, due to {:#?}", username, surname, &e);
      },
    };
  }
}