// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    let area = area_of(width, height);
    println!("Area is {}", area);
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(p0: i32, p1: i32, p2: i32) -> i32 {
    p0 * p1 * p2
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}
