use std::{fs, collections::{HashMap, BTreeMap}};

pub fn day1_1(path: &str) -> i32 {
    let content = fs::read_to_string(path).unwrap();

    let mut total = 0;

    for line in content.lines() {

        let mut word: String = "".to_string();

        for c in line.chars() {
            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        total += word.parse::<i32>().unwrap();
    }

    total
}

pub fn day1_2(path: &str) -> i32 {
    let content = fs::read_to_string(path).unwrap();

    let mut values = HashMap::new();
    values.insert(String::from("one"), 1);
    values.insert(String::from("two"), 2);
    values.insert(String::from("three"), 3);
    values.insert(String::from("four"), 4);
    values.insert(String::from("five"), 5);
    values.insert(String::from("six"), 6);
    values.insert(String::from("seven"), 7);
    values.insert(String::from("eight"), 8);
    values.insert(String::from("nine"), 9);

    let mut total = 0;

    for line in content.lines() {

        let mut word: String = "".to_string();

        let mut vals = BTreeMap::new();

        for (k, v) in values.iter() {
            if let Some(id) = line.find(k) {
                vals.insert(id, v);
            }
        }

        for (k, v) in values.iter() {
            if let Some(id) = line.rfind(k) {
                vals.insert(id, v);
            }
        }

        let first = vals.iter().next();
        let last = vals.iter().next_back();

        for (i, c) in line.chars().enumerate() {
            if first.is_some_and(|(k,_)| k < &i) {
                word.push(char::from_digit(**first.unwrap().1, 10).unwrap());
                break;
            }
            
            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            let ri = line.len()-1 - i;
            if last.is_some_and(|(k,_)| k > &ri) {
                word.push(char::from_digit(**last.unwrap().1, 10).unwrap());
                break;
            }

            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        println!("{} {}", line, word);

        total += word.parse::<i32>().unwrap();
    }

    total
}