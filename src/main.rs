use bevy::prelude::*;
use std::io;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, debug_world)
        .add_systems(Update, debug_console)
        .run();
}

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Friend;

fn debug_console( 
    mut commands: Commands,
    friend_query: Query<&Name, With<Friend>>,
    item_query: Query<&Name, With<Item>>,
) {
    println!("-----");
    println!("Debugging");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let command = input.trim();

    match command {
        "items" => {
            for name in &item_query {
                println!("{}", name.0);
            }
        },
        "friends" => {
            for name in &friend_query {
                println!("{}", name.0);
            }
        },
        _ => println!("Invalid command"),
    }
}

fn debug_world(mut commands: Commands) {
    commands.spawn(( Name("Spoon".to_string()), Item ));
    commands.spawn(( Name("Toby".to_string()), Friend ));
}
