use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use rand::{distributions::{Distribution, Standard}, Rng};
use std::io::Bytes;
use bson::to_bson;

// #[derive(Read, Write)]
// struct DataType(Vec<Move>);

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: u32,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            direction: Direction::Right,
            distance: 10,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

fn main() {

    let mut a = data_gen(1000);

    println!("a[10] = {:?}", a[100]);
    let mut file = File::create("a.bson").unwrap();

    let b: bson::Bson = bson::to_bson(&a).unwrap();

    println!("{:?}", b.as_array().unwrap()[0]);


    let mut file = File::open("a.bson").unwrap();

    let deserialized = bson::Document::from_reader(&mut file).unwrap();

    println!("deserialized[10] = {:?}", deserialized);

    // let mut a: [Move; 1000] = [Move {
    //     direction: Direction::Right,
    //     distance: 10,
    // }; 1000];
    //
    // let mut file = File::create("a.bson").unwrap();
    //
    // // let doc = bson::Document::from_reader(&mut a.as_slice()).unwrap();
    //
    // for m in a {
    //     let b: bson::Bson = bson::to_bson(&m).unwrap();
    //     b.as_document().unwrap().to_writer(&mut file);
    //     // file.write(Bytes::from(bson::to_bson(&m).unwrap().into()));
    //     // println!("{:?}", bson::to_bson(&m).unwrap().to_string());
    // }
    // // file.write(bson::to_bson(&a).unwrap().to_string().as_bytes());
    //
    // let mut file = File::open("a.bson").unwrap();
    // let len_per_record= file.metadata().unwrap().len() / 1000;
    // for i in 0..1000 {
    //     let d = bson::Document::from_reader(&mut file).unwrap();
    //     let a: bson::Bson = bson::from_document(d).unwrap();
    //     // let mut s = String::new();
    //     // file.read_to_string(&mut s);
    //     // let b: bson::Bson = s[(i) as usize..(i + len_per_record) as usize].into();
    //     // let a: Move = bson::from_bson(b).unwrap();
    //     println!("{:?}", a);
    // }
}

fn data_gen(num: usize) -> Vec<Move> {
    let mut ret = Vec::with_capacity(num);

    for i in 0..num {
        ret.push(Move {
            direction: rand::random(),
            distance: i as u32,
        });
    }

    ret
}