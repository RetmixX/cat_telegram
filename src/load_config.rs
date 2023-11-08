use std::env;
use dotenv::dotenv;

pub fn load_token() -> String {
    dotenv().ok();
    env::var("TOKEN").expect("TOKEN not set")
}

pub fn load_creator_id() -> u64 {
    dotenv().ok();

    env::var("CREATOR_ID").expect("CREATOR_ID not set").parse::<u64>()
        .expect("CREATOR_ID not number")
}

pub fn load_admins() -> Vec<u64> {
    dotenv().ok();
    let data = env::var("ADMINS").expect("ADMIN_IDS not set");
    let mut admins = data.split(",").map(|id| id.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    admins.push(load_creator_id());

    admins
}