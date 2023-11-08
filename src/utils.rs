use std::fs;
use std::path::Path;
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use crate::load_config::{load_admins, load_creator_id};

lazy_static! {
    static ref CREATOR_ID: u64 = load_creator_id();
    static ref ADMINS: Vec<u64> = load_admins();
}

pub fn get_random_file() -> String {
    let path = "cats";

    let paths_dir = fs::read_dir(path).unwrap();
    let paths_cat: Vec<String> = paths_dir.map(|data| data.unwrap().path().display().to_string()).collect();

    let random_pic = paths_cat.choose(&mut rand::thread_rng()).unwrap();

    format!("{random_pic}")
}

pub fn get_file(path: &str) -> &Path {
    let path_info = Path::new(path);
    if !path_info.exists() {
        panic!("Image by path - [{path}] not found");
    };

    path_info
}

pub fn check_permission(user_id: u64) -> bool {
    ADMINS.contains(&user_id)
}

pub fn check_creator(user_id: u64) -> bool {
    CREATOR_ID.eq(&user_id)
}