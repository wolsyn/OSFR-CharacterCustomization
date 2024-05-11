use log::error;
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FacePaint {
    id: usize,
    texture_alias: String
}

#[derive(Debug, Serialize)]
pub struct Hair {
    id: usize,
    addr: String,
    name: String
}
#[derive(Debug, Serialize)]
pub struct ModelExtra {
    id: usize,
    addr: String,
}

#[derive(Debug, Serialize)]
pub struct EyeColor {
    name: String,
    color: u8
}
#[derive(Debug, Serialize)]
pub struct HairColor {
    name: String,
    color: u8
}

#[derive(Debug, Serialize)]
pub struct Extras {
    id: usize,
    name: String,
    species: String,
    gender: String,
    addr: String
}

pub fn get_eye_color(path: &str) -> Result<Vec<EyeColor>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut eye_colors: Vec<EyeColor> = vec![];

    let mut stmt = conn.prepare("SELECT name, color FROM Eye_Color")?;

    let eye_iter = stmt.query_map([], |row| {
        Ok(
            EyeColor {
                name: row.get(0)?,
                color: row.get(1)?
            }
        )
    })?;

    for color in eye_iter {
        let color = color.unwrap();
        eye_colors.push(
            EyeColor { name: color.name, color: color.color }
        );
    }

    Ok(eye_colors)
}

pub fn get_facepaints(path: &str) -> Result<Vec<FacePaint>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut facepaints: Vec<FacePaint> = vec![];

    let mut stmt = conn.prepare("SELECT id, texture_alias FROM FacePaint")?;
    let facepaint_iter = stmt.query_map([], |row| {
        Ok(FacePaint {
            id: row.get(0)?,
            texture_alias: row.get(1)?,
        })
    })?;
    for facepaint in facepaint_iter {
        let facepaint = facepaint.unwrap();
        let buff_facepaints = FacePaint {
            id: facepaint.id,
            texture_alias: facepaint.texture_alias
        };
        facepaints.push(buff_facepaints);

    }
    Ok(facepaints)
}

pub fn get_hairs(path: &str, target_gender: &str) -> Result<Vec<Hair>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut hairs: Vec<Hair> = vec![];

    let mut stmt = conn.prepare("SELECT id, addr, name FROM Hair WHERE gender = ?")?;

    let extra_iter = match stmt.query_map([target_gender], |row| {
        Ok(Hair {
            id: row.get(0)?,
            addr: row.get(1)?,
            name: row.get(2)?
        })
    }) {
    Ok(mapped_rows) => {mapped_rows},
    Err(e) => {
        error!("Error iterating Extras due to {:#?}", e);
        panic!();
    },
};

    for hair in extra_iter {
        let hair = hair.unwrap();
        let buff_hair = Hair {
            id: hair.id,
            addr: hair.addr,
            name: hair.name
        };
        hairs.push(buff_hair);

    }
    Ok(hairs)
}

pub fn get_hair_color(path: &str) -> Result<Vec<HairColor>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut hair_colors:Vec<HairColor> = vec![];

    let mut stmt = conn.prepare("SELECT name, color FROM Hair_Color")?;

    let hair_iter = stmt.query_map([], |row| {
        Ok(
            EyeColor {
                name: row.get(0)?,
                color: row.get(1)?
            }
        )
    })?;

    for color in hair_iter {
        let color = color.unwrap();
        hair_colors.push(
            HairColor { name: color.name, color: color.color }
        );
    }

    Ok(hair_colors)
}

pub fn get_extras_by_gender_species(
    path: &str,
    target_gender: &str,
    target_species: &str,
) -> Result<Vec<Extras>, rusqlite::Error> {
    let conn = Connection::open(path).unwrap();
    let mut extras: Vec<Extras> = vec![];

    let sql_query = format!(
        "SELECT id, name, species, gender, addr FROM extras WHERE gender = ? AND species = ?"
    );

    let mut stmt = match conn.prepare(&sql_query) {
        Ok(stm) => {stm},
        Err(e) => {
            error!("Error ocurred while preparing the statement {}, due to {:?}", &sql_query, e);
            panic!();
        },
    };

    let extra_iter = match stmt.query_map([target_gender, target_species], |row| {
            Ok(Extras {
                id: row.get(0)?,
                name: row.get(1)?,
                species: row.get(2)?,
                gender: row.get(3)?,
                addr: row.get(4)?
            })
        }) {
        Ok(mapped_rows) => {mapped_rows},
        Err(e) => {
            error!("Error iterating Extras due to {:#?}", e);
            panic!();
        },
    };

    for extra in extra_iter {
        let extra = extra.unwrap();
        let buff_extra = Extras {
            id: extra.id,
            name: extra.name,
            species: extra.species,
            gender: extra.gender,
            addr: extra.addr,
        };
        extras.push(buff_extra);
    }

    Ok(extras)
}
