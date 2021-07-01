mod board;

fn main() {
    let b = board::Board::new()
        .with_width(3)
        .with_height(3)
        .with_difficulty(1)
        .build();
    println!("Minesweepers!");
    println!("{:#?}", b);
}
