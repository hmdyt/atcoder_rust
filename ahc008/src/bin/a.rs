use proconio::input;
use std::io::{stdout, Write};

struct Pos {
    x: usize,
    y: usize,
}
struct Human {
    pos: Pos,
}
struct Animal {
    pos: Pos,
    attr: usize,
}

struct Room {
    can_put: [[bool; 30]; 30],
    can_walk: [[bool; 30]; 30],
}

fn init(room: &mut Room, humans: &mut Vec<Human>, animals: &mut Vec<Animal>) {
    // init animals
    input! {n: usize}
    for _ in 0..n {
        input! {px: usize, py: usize, pt: usize}
        let pos = Pos {
            x: px - 1,
            y: py - 1,
        };
        let attr = pt;
        animals.push(Animal { pos, attr });
    }
    // init humans
    input! {m: usize}
    for _ in 0..m {
        input! {hx: usize, hy: usize}
        let pos = Pos {
            x: hx - 1,
            y: hy - 1,
        };
        humans.push(Human { pos });
    }
    // init room
    for animal in animals.iter() {
        room.can_walk[animal.pos.x][animal.pos.y] = false;
        room.can_put[animal.pos.x][animal.pos.y] = false;
        if animal.pos.x < 29 {
            room.can_put[animal.pos.x + 1][animal.pos.y + 0] = false;
        }
        if 0 < animal.pos.x {
            room.can_put[animal.pos.x - 1][animal.pos.y + 0] = false;
        }
        if animal.pos.y < 29 {
            room.can_put[animal.pos.x + 0][animal.pos.y + 1] = false;
        }
        if 0 < animal.pos.y {
            room.can_put[animal.pos.x + 0][animal.pos.y - 1] = false;
        }
    }
    for human in humans.iter() {
        room.can_walk[human.pos.x][human.pos.y] = false;
        room.can_put[human.pos.x][human.pos.y] = false;
    }
}
fn main() {
    let mut room = Room {
        can_put: [[true; 30]; 30],
        can_walk: [[true; 30]; 30],
    };
    let mut humans = Vec::new();
    let mut animals = Vec::new();
    init(&mut room, &mut humans, &mut animals);
    for _ in 0..300 {
        println!("{}", ".".to_string().repeat(humans.len()));
        stdout().flush().unwrap();
        input! {_: [String; animals.len()]}
    }
    animals[0].attr = 0;
}
