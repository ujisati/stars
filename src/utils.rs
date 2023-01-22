use rand::prelude::*;
use std::fs;

static mut NAMES: Vec<String> = Vec::new();
static mut USED_NAMES: Vec<String> = Vec::new();

pub fn load_names() -> &'static Vec<String> {
    if unsafe { NAMES.is_empty() } {
        let contents: String = fs::read_to_string("names.csv").unwrap();
        let names: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
        unsafe { NAMES = names };
    }
    unsafe { &NAMES }
}

pub fn random_name() -> String {
    let names = load_names();
    let strategy = random::<u8>() % 3;
    let mut name_used = true;
    let mut name = String::new();
    while name_used {
        name = match strategy {
            0 => random_name_default(names),
            1 => random_name_mixed(names),
            2 => random_name_mixed_with_num(names),
            3 => random_name_default_with_num(names),
            _ => panic!("Invalid strategy"),
        };
        if !unsafe { USED_NAMES.contains(&name) } {
            name_used = false;
        }
    }
    unsafe { USED_NAMES.push(name.clone()) };
    name
}

fn random_name_default(names: &Vec<String>) -> String {
    let index = random::<usize>() % names.len();
    names[index].clone()
}

fn random_name_default_with_num(names: &Vec<String>) -> String {
    let index = random::<usize>() % names.len();
    let name = names[index].clone();
    let number: u32 = random::<u32>() % 999;
    let number_chars: Vec<char> = number.to_string().chars().collect::<Vec<_>>();
    name.chars().chain(number_chars).collect::<String>()
}

fn random_name_mixed(names: &Vec<String>) -> String {
    let index1 = random::<usize>() % names.len();
    let name1: Vec<char> = names[index1].clone().chars().collect::<Vec<_>>();
    let index2 = random::<usize>() % names.len();
    let name2: Vec<char> = names[index2].clone().chars().collect::<Vec<_>>();
    name1[..name1.len() / 2]
        .iter()
        .chain(name2[name2.len() / 2..].iter())
        .collect::<String>()
}

fn random_name_mixed_with_num(names: &Vec<String>) -> String {
    let index1 = random::<usize>() % names.len();
    let name1: Vec<char> = names[index1].clone().chars().collect::<Vec<_>>();
    let index2 = random::<usize>() % names.len();
    let name2: Vec<char> = names[index2].clone().chars().collect::<Vec<_>>();
    let number: u32 = random::<u32>() % 999;
    let number_chars: Vec<char> = number.to_string().chars().collect::<Vec<_>>();
    name1[..name1.len() / 2]
        .iter()
        .chain(name2[name2.len() / 2..].iter())
        .chain(vec!['-'].iter())
        .chain(number_chars.iter())
        .collect::<String>()
}
