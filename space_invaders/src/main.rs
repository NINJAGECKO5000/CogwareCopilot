use env_logger::Env;

use space_invaders::{StdFrameBuffer};


pub fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

}