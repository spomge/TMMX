// libs
use std::io::stdin;
use std::string::String;
use serde_json::Value;
use std::fs;

// crates
use crate::file::find_json_object;


#[allow(dead_code)]

#[derive(Debug)]
pub struct ItemInfo {

    name: String,
    value: i64,
    demand: u8,
    _type: String

}

pub fn auto_correct(missed_name: String) -> Option<String> {

    // finds the item.json folder
    let items_path = find_json_object("../Items.json");
    let json_file: String = fs::read_to_string(items_path).expect("Could not find the value provided");
    let json_object: Value = serde_json::from_str(&json_file)
        .expect("Json code is invalid!");

    // unwraps json and index's it
    let json_table_index = json_object.as_object().expect("corrupted json");

    // could be vec
    let mut items_it_could_be: Vec<String> = Vec::new();

    let (first_word, _) = missed_name.split_once(' ').unwrap_or((missed_name.as_str(), ""));
    for (name, _value) in json_table_index.iter() {
        if name.to_lowercase().contains(&String::from(first_word).to_lowercase()) {
            println!("{}. {}", items_it_could_be.len() + 1, name);
            items_it_could_be.push(name.to_owned());
        }
    }

    if items_it_could_be.is_empty() {
        return None
    }

    let mut line = String::new();
    println!("choose an index: ");
    stdin().read_line(&mut line).expect("could not read input!");

    let read_index: usize = line.trim().parse().expect("needs to be a number");

    let index = items_it_could_be.get(read_index - 1);
    if index.is_some() {
        Some(index.unwrap().to_owned())
    } else {
        None
    }

}

pub fn get_item_total_value(name: &str, amount: i64) -> Option<i64> {

    let maybe_item = get_item_info(name);

    if maybe_item.is_some() {
        Some(maybe_item.unwrap().value * amount)
    } else {
        None
    }

}

pub fn get_all_items_with(item_type: String) -> Option<Vec<String>> {

    // finds the item.json folder
    let items_path = find_json_object("../Items.json");
    let json_file: String = fs::read_to_string(items_path).expect("Could not find the value provided");
    let json_object: Value = serde_json::from_str(&json_file)
        .expect("Json code is invalid!");

    // unwraps json and index's it
    let json_table_index = json_object.as_object().expect("corrupted json");

    // all items
    let mut items: Vec<String> = Vec::new();

    for (name, value) in json_table_index.iter() {

        if value["type"].as_str().expect("currupted type format") == item_type.as_str() {
            items.push(name.to_owned())
        }

    }

    if items.is_empty() {
        None
    } else {
        Some(items)
    }

}

pub fn get_item_info(name: &str) -> Option<ItemInfo> {

    // finds the item.json folder
    let items_path = find_json_object("../Items.json");
    let json_file: String = fs::read_to_string(items_path).expect("Could not find the value provided");
    let json_object: Value = serde_json::from_str(&json_file)
        .expect("Json code is invalid!");

    // unwraps json and index's it
    let json_table_index = json_object.as_object().expect("corrupted json");
    let value = json_table_index.get(name);

    // if value is something the return some<ItemInfo>
    if value.is_some() {
        Some(ItemInfo {
            name: name.to_owned(),
            value: value.unwrap()["value"].as_i64().unwrap(),
            demand: value.unwrap()["demand"].as_u64().unwrap() as u8,
            _type: String::from(value.unwrap()["type"].as_str().unwrap()),
        })
    } else {
        let item  = auto_correct(String::from(name));
        if item.is_some() {
            get_item_info(item.unwrap().as_str())
        } else {
            None
        }
    }

}

