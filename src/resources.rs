use bevy::prelude::*;
use log;
use rand::prelude::*;
use rand::Rng;
use std::fs;
use std::path;

#[derive(Resource)]
pub struct Config {
    pub galaxy_dimension: u32,
    pub num_stars: u32,
}

impl Config {
    pub fn validate(self) -> Self {
        if self.num_stars > self.galaxy_dimension.pow(2) {
            panic!("num_stars must be less than galaxy_dimension^2");
        }
        self
    }
}

impl FromWorld for Config {
    fn from_world(world: &mut World) -> Self {
        log::info!("creating config");
        Config {
            galaxy_dimension: 25,
            num_stars: 50,
        }
        .validate()
    }
}

#[derive(Resource)]
pub struct NameGenerator {
    names: Vec<String>,
    used_names: Vec<String>,
}

impl NameGenerator {
    pub fn new() -> Self {
        let mut name_generator = NameGenerator {
            names: Vec::new(),
            used_names: Vec::new(),
        };
        name_generator.load_names();
        name_generator
    }
    fn load_names(&mut self) {
        log::info!("loading names");
        let contents: String = fs::read_to_string("names.csv").expect("unable to read file");
        self.names = contents.split("\n").map(|s| s.to_string()).collect();
        log::info!("loaded {} names", self.names.len());
    }

    pub fn random_name(&mut self) -> String {
        let strategy = random::<u8>() % 3;
        let mut name_used = true;
        let mut name = String::new();
        while name_used {
            name = match strategy {
                0 => Self::random_name_default(&self.names),
                1 => Self::random_name_mixed(&self.names),
                2 => Self::random_name_mixed_with_num(&self.names),
                3 => Self::random_name_default_with_num(&self.names),
                _ => panic!("Invalid strategy"),
            };
            if !self.used_names.contains(&name) {
                name_used = false;
            }
        }
        self.used_names.push(name.clone());
        log::trace!("generated name {}", name);
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
}

impl FromWorld for NameGenerator {
    fn from_world(world: &mut World) -> Self {
        log::info!("creating name generator");
        NameGenerator::new()
    }
}
