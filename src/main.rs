use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[macro_use]
extern crate rocket;

mod board;

pub struct CORS;

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
    let used_flags = b.flag(x, y);
    let mut board = b.export_board();
    board.push_str(&used_flags.to_string());
    if b.won() {
        board.push_str("\nWin!");
    } else if b.lost() {
        board.push_str("\nLose!");
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
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, new, flag, reveal])
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
