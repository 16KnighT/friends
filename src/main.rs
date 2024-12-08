mod ui_helper;
mod sim_components;

use bevy::prelude::*;
use ui_helper::UiHelperPlugin;
use sim_components::*;


fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiHelperPlugin)
        .add_systems(Startup, debug_world)
        .run();
}

fn debug_world(mut commands: Commands) {
    

    //spawn items
    commands.spawn(( Name("Spoon".to_string()), Item ));

    //spawn characters
    commands.spawn(( Name("Toby".to_string()), Character ));
    commands.spawn(( Name("Swann".to_string()), Character ));
    commands.spawn(( Name("Dan".to_string()), Character ));
    commands.spawn(( Name("Tamara".to_string()), Character ));
    commands.spawn(( Name("Niamh".to_string()), Character ));
    commands.spawn(( Name("Erin".to_string()), Character ));
}
