use bevy::prelude::*;
use std::io;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, debug_world)
        .add_systems(Startup, debug_ui)
        .add_systems(Update, update_debug_ui)
        .run();
}

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct CharacterList;

#[derive(Component)]
pub struct ItemList;

fn debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //display
    commands.spawn(Camera2d::default());
    commands.spawn(Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.0),
        ..default()
    })
    .with_children(|parent| {

        //character list
        parent
            .spawn((Node {
                width: Val::Px(200.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .with_children(|parent| {

            parent.spawn((
                Text::new("Character List"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                Label,
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                CharacterList,
            ));

        });

        //item list
        parent
            .spawn((Node {
                width: Val::Px(200.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .with_children(|parent| {
            
            parent.spawn((
                Text::new("Item List"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                Label,
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ItemList,
            ));

        });

    });
}

fn update_debug_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    
    mut character_list_query: Query<Entity, With<CharacterList>>,
    mut item_list_query: Query<Entity, With<ItemList>>,
    character_query: Query<&Name, Added<Character>>,
    item_query: Query<&Name, Added<Item>>,
) {
    let characters = character_query.iter();
    let items = item_query.iter();

    if let Ok(character_list) = character_list_query.get_single() {
        for character in characters {
            commands.entity(character_list).with_children(|parent| {
    
                parent.spawn((
                    Text::new(format!("{}", character.0)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    Label,
                ));
    
            });
        }
    };
    if let Ok(item_list) = item_list_query.get_single() {
        for item in items {
            commands.entity(item_list).with_children(|parent| {
    
                parent.spawn((
                    Text::new(format!("{}", item.0)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    Label,
                ));
    
            });
        }
    };

}

fn debug_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    

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
