use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, hello).run();
}

fn hello() {
    println!("Hello! Happy to be here.")
}
