use std::fs;
use std::io::Read;

pub fn run() {
    let mut file = fs::File::open("src/data/01.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let masses: Vec<u32> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // PART I
    let mut fuel1: u32 = 0;
    for mass in masses.iter() {
        fuel1 += fuel_required1(*mass);
    }
    println!("PART I: Fuel required: {}", fuel1);

    // PART II
    let mut fuel2: u32 = 0;
    for mass in masses.iter() {
        fuel2 += fuel_required2(*mass);
    }
    println!("PART II: Fuel required: {}", fuel2);
}

fn fuel_required1(mass: u32) -> u32 {
    mass / 3 - 2
}

fn fuel_required2(mut mass: u32) -> u32 {
    let mut fuel: u32 = 0;
    while mass / 3 >= 2 {
        mass = mass / 3 - 2;
        fuel += mass;
    }
    fuel
}

#[test]
fn test_examples() {
    assert_eq!(fuel_required1(12), 2);
    assert_eq!(fuel_required1(14), 2);
    assert_eq!(fuel_required1(1969), 654);
    assert_eq!(fuel_required1(100756), 33583);
}
