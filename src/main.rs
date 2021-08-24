mod board;
mod solver;

use board::Board;
use solver::solve;

fn main() {
    let mut b = Board::new(9);
    println!("empty board:\n{}", b);

    b[0] = 12;
    println!("set a value: b[0] == {}, b[1] == {}", b[0], b[1]);

    println!("new board:\n{}", b);

    for i in 0..b.degree {
        b[(i, 3)] = 3;
    }

    for col in b.cols() {
        println!("column {:?}", col);
    }

    println!("\n{}", b);

    for z in b.zones() {
        println!("{:?}", z);
    }
}
