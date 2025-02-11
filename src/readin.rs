use std::fs::File;
use std::io::{Write, Read};
use crate::geometry::Object;

#[allow(dead_code)]
pub fn save_objects_to_file(objects: &[Object], filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string(objects)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_objects_from_file(filename: &str) -> std::io::Result<Vec<Object>> {
    let mut file = File::open(filename)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let objects: Vec<Object> = serde_json::from_str(&json)?;
    Ok(objects)
}

pub fn safe_load_objects(filename: &str, objs: Vec<Object>) -> Vec<Object> {
    let res = load_objects_from_file(filename);
    match res {
        Ok(objects) => objects,
        Err(_) => objs
    }
}