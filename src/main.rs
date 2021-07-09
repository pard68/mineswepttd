#[macro_use]
extern crate rocket;

mod board;

#[get("/")]
fn index() -> &'static str {
    "Minesweeper!"
}

#[get("/new/<width>/<height>/<difficulty>")]
fn new(width: usize, height: usize, difficulty: usize) -> String {
    board::Board::new()
        .with_width(width)
        .with_height(height)
        .with_difficulty(difficulty)
        .build()
        .export_state()
}

#[post("/flag/<x>/<y>?<send_state>", data = "<board_state>")]
fn flag(x: usize, y: usize, send_state: bool, board_state: String) -> String {
    let mut b = board::Board::from(board_state);
    b.flag(x, y);
    let mut board = b.export_board();
    println!("{:?}", send_state);
    if b.won() {
        board.push_str("Win!");
    } else if b.lost() {
        board.push_str("Lose!");
    }
    if send_state {
        board + "\n" + &b.export_state()
    } else {
        board
    }
}

#[post("/reveal/<x>/<y>?<send_state>", data = "<board_state>")]
fn reveal(x: usize, y: usize, send_state: bool, board_state: String) -> String {
    let mut b = board::Board::from(board_state);
    b.reveal(x, y);
    let mut board = b.export_board();
    if b.won() {
        board.push_str("Win!");
    } else if b.lost() {
        board.push_str("Lose!");
    }
    if send_state {
        board + "\n" + &b.export_state()
    } else {
        board
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new, flag, reveal])
}
