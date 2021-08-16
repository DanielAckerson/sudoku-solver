mod board;

use board::Board;

fn main() {
    let mut b = Board { cells: [0; 81] };
    println!("empty board:\n{}", b);

    b[0] = 12;
    println!("set a value: b[0] == {}, b[1] == {}", b[0], b[1]);

    println!("new board:\n{}", b);
}
