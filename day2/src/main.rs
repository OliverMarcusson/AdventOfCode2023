#![allow(dead_code)]

use std::fs;

#[derive(Debug)]
pub struct Bag {
    red: u32,
    green: u32,
    blue: u32
}

fn get_input(file: &str) -> Vec<String> {
    let input = fs::read_to_string(format!("./src/{}", file).as_str()).expect("Failed to read input.");
    let mut input: Vec<String> = input.split("\n").map(|x| x.to_owned()).collect();
    
    for line in &mut input {
        *line = line.split_once(": ").unwrap().1.to_owned();
    }
    
    input
}
mod part_one {
    pub fn process_game(line: &String) -> crate::Bag {
        let split: Vec<String> = line.split("; ").map(|x| x.to_owned()).collect();
        let mut split2: Vec<Vec<String>> = Vec::new(); 
        
        for element in split {
            split2.push(element.split(", ").map(|x| x.to_owned()).collect::<Vec<String>>());
        }
    
        // let mut split3: Vec<Bag> = Vec::new();
    
        let mut red = (false, 0);
        let mut green = (false, 0);
        let mut blue = (false, 0);
        
        // for every set in the current game
        for thing in split2 {
            // for every specific amount of cubes
            for element in thing {
                red.0 = false;
                green.0 = false;
                blue.0 = false;
    
                let split = element.split(" ").map(|x| x.to_owned()).collect::<Vec<String>>();
                
                match split[1].to_lowercase().as_str() {
                    "red" => {
                        red.0 = true;
                        let num = split[0].parse::<u32>().unwrap();
                        if num > red.1 {
                            red.1 = num;
                        }
                    },
    
                    "green" => {
                        green.0 = true;
                        let num = split[0].parse::<u32>().unwrap();
                        if num > green.1 {
                            green.1 = num;
                        }
                    },
    
                    "blue" => {
                        blue.0 = true;
                        let num = split[0].parse::<u32>().unwrap();
                        if num > blue.1 {
                            blue.1 = num;
                        }
                    },
    
                    _ => {
                        unreachable!();
                    }
    
                }
                
            }
        }
    
        crate::Bag{red: red.1, green: green.1, blue: blue.1}
    }
}

mod part_two {
    pub fn power(game: &String) -> u32 {
        let split: Vec<String> = game.split("; ").map(|x| x.to_owned()).collect(); // game sets
        let mut split2: Vec<Vec<String>> = Vec::new(); 
        
        for element in split {
            split2.push(element.split(", ").map(|x| x.to_owned()).collect::<Vec<String>>());
        }

        let mut red: Vec<u32> = Vec::new();
        let mut green: Vec<u32> = Vec::new();
        let mut blue: Vec<u32> = Vec::new();

        for set in split2 {
            for value in set {
            let value_color = value.split(" ").map(|x| x.to_owned()).collect::<Vec<String>>();

                match value_color[1].to_lowercase().as_str() {
                    "red" => {
                        red.push(value_color[0].parse::<u32>().unwrap())
                    },
    
                    "green" => {
                        green.push(value_color[0].parse::<u32>().unwrap())
                    },

                    "blue" => {
                        blue.push(value_color[0].parse::<u32>().unwrap())
                    },
    
                    _ => {
                        unreachable!();
                    }
    
                }
            }
        }

        red.iter().max().unwrap() * green.iter().max().unwrap() * blue.iter().max().unwrap()

        // println!("{:?}", split2);
    }
}

fn part_one_fn() {
    let input = get_input("input.txt");
    let max = Bag {red: 12, green: 13, blue: 14};
    let mut sum = 0;
    
    for (id, line) in input.iter().enumerate() {
        let game_values = part_one::process_game(line);
        if game_values.red <= max.red && game_values.green <= max.green && game_values.blue <= max.blue {
            sum += id + 1
        } 
    }
    println!("{sum}");
}

fn part_two_fn() {
    let input = get_input("input.txt");
    let mut sum = 0;
    
    for game in input {
        sum += part_two::power(&game);
    }
    
    println!("{sum}");
}

fn main() {
    part_two_fn();
}
