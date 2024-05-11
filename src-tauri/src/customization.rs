use std::{
    fs::File,
    path::Path, io::Read,
};

use log::{info, warn, error};
use serde_json::Value;

pub async fn new_character(first_name: &str, surname: &str) -> Result<(), std::io::Error> {

    if !Path::new("characters").exists() {
        std::fs::create_dir_all("characters")?;
    }

    match !Path::new(&format!("characters/{}.json", first_name)).is_file() {
        true => {
            warn!("Character File {} does not Exist, creating...", first_name);
            let mut fallback = File::open("Fallback.json")?;
            let mut fallback_json: String = String::new();

            fallback.read_to_string(&mut fallback_json)?;

            let mut template_json:Value = serde_json::from_str(&fallback_json)?;

            let mut new_character = File::create(format!("characters/{}{}.json", first_name, surname))?;

            template_json["FirstName"] = serde_json::Value::String(first_name.into());
            template_json["LastName"] = serde_json::Value::String(surname.into());
            match serde_json::to_writer(&mut new_character, &template_json) {
                Ok(_) => {info!("Operation finished successfully")},
                Err(e) => {
                    eprintln!("Operation failed due to {:#?}", &e);
                    error!("Operation failed due to {:#?}", &e);
        },
            };
            return Ok(());
        },
        false => {
            warn!("Character File {} is already present, skipping creation step...", first_name);
            return Ok(());
        },
    }
}

pub async fn modify_gender(username: &str, surname: &str, gender: u8) -> Result<(), std::io::Error> {
    warn!("Setting GenderRace");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;

    json["PlayerGUID"] = serde_json::Value::Number(gender.into());
    json["PlayerModel"] = serde_json::Value::Number(gender.into());

    let mut file = File::create(file_path)?;

    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };
    Ok(())
}

pub async fn modify_eyes(username: &str, surname: &str, eye_color: usize) -> Result<(), std::io::Error> {
    warn!("Setting Eye Color");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;
    json["EyeColor"] = serde_json::Value::Number(eye_color.into());

    let mut file = File::create(file_path)?;

    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };
    Ok(())
}

pub async fn modify_hair(username: &str, surname: &str, hair_type: &str, haircolor: usize) -> Result<(), std::io::Error> {
    warn!("Setting Hair");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;

    let hair_format: String = format!("{}", hair_type);

    json["PlayerHair"] = serde_json::Value::String(hair_format);
    json["HairColor"] = serde_json::Value::Number(haircolor.into());

    let mut file = File::create(file_path)?;
    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };

    Ok(())
}

pub async fn modify_skintone(username: &str, surname: &str, new_skintone: &str) -> Result<(), std::io::Error> {
    warn!("Setting Skintone");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;

    json["Skintone"] = serde_json::Value::String(new_skintone.to_string());

    let mut file = File::create(file_path)?;
    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };
    Ok(())
}

pub async fn modify_extras(username: &str, surname: &str, extra: &str) -> Result<(), std::io::Error> {
    warn!("Setting Extras");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;

    json["HumanBeardsPixieWings"] = serde_json::Value::String(extra.into());

    let mut file = File::create(file_path)?;

    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };
    Ok(())
}

pub async fn modify_facepaint(username: &str, surname: &str, facepaint: &str) -> Result<(), std::io::Error> {
    warn!("Setting FacePaint");
    let file_path = format!("characters/{}{}.json", username, surname);

    let mut file = File::open(&file_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut json: Value = serde_json::from_str(&buffer)?;

    json["FacePaint"] = serde_json::Value::String(facepaint.into());

    let mut file = File::create(file_path)?;

    match serde_json::to_writer(&mut file, &json) {
        Ok(_) => {info!("Operation finished successfully")},
        Err(e) => {
            eprintln!("Operation failed due to {:#?}", &e);
            error!("Operation failed due to {:#?}", &e);
        },
    };
    Ok(())
}