pub mod ui;
pub mod family;
pub mod glass;
pub mod recipe;
pub mod db;


use human_panic::setup_panic;

fn main() {
    setup_panic!();
    ui::main().unwrap()
}
