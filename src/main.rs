pub mod db;
pub mod family;
pub mod glass;
pub mod recipe;
pub mod ui;

use human_panic::setup_panic;

fn main() {
    setup_panic!();
    ui::main().unwrap()
}
