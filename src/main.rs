mod board;

use board::Board;

fn main() {
    let mut b = Board::new(9);
    println!("empty board:\n{}", b);

    b[0] = 12;
    println!("set a value: b[0] == {}, b[1] == {}", b[0], b[1]);

    println!("new board:\n{}", b);
}
