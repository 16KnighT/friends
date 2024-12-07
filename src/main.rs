mod ui_helper;

use bevy::{prelude::*, ui::FocusPolicy};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSystem,
    TextInputTextColor, TextInputTextFont,
};
use ui_helper::{spawn_button, spawn_container, spawn_text, spawn_text_input};

const BOX_BACKGROUND_COLOUR: Color = Color::srgb(0.25, 0.25, 0.25);
const TEXTBOX_COLOUR: Color = Color::srgb(0.15, 0.15, 0.15);

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TextInputPlugin)
        .add_systems(Startup, debug_world)
        .add_systems(Startup, debug_ui)
        .add_systems(Update, update_debug_ui)
        .add_systems(Update, focus.before(TextInputSystem))
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

#[derive(Component)]
enum DebugButtonActions {
    CreateCharacter,
    CreateItem,
}

fn debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //display
    commands.spawn(Camera2d::default());
    let display = commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            ..default()
        },
        //allows the background to be docus to take the focus away from the text input
        Interaction::None,
    )).id();
    
    //character list ui
    let character_box = spawn_container(&mut commands, 100., 20., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    let character_header = spawn_text(&mut commands, "Character List", 25.0);
    let character_list = spawn_container(&mut commands, 100., 100., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    commands.entity(character_list).insert(CharacterList);

    commands.entity(character_box).add_children(&[character_header, character_list]);

    //item list ui
    let item_box = spawn_container(&mut commands, 100., 20., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    let item_header = spawn_text(&mut commands, "Item List", 25.0);
    let item_list = spawn_container(&mut commands, 100., 100., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    commands.entity(item_list).insert(ItemList);

    commands.entity(item_box).add_children(&[item_header, item_list]);

    let forms_container = spawn_container(&mut commands, 100., 20., FlexDirection::Column, None);
    let character_form = spawn_container(&mut commands, 25., 100., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    let character_form_header = spawn_text(&mut commands, "Create Character", 25.);
    let character_form_input = spawn_text_input(&mut commands, 200., "Name", 20.);
    let character_submit = spawn_button(&mut commands, "Go", DebugButtonActions::CreateCharacter);

    commands.entity(character_form).add_children(&[character_form_header, character_form_input, character_submit]);

    let item_form = spawn_container(&mut commands, 25., 100., FlexDirection::Column, Some(BOX_BACKGROUND_COLOUR));
    let item_form_header = spawn_text(&mut commands, "Create item", 25.);
    let item_form_input = spawn_text_input(&mut commands, 200., "Name", 20.);
    let item_submit = spawn_button(&mut commands, "Go", DebugButtonActions::CreateItem);

    commands.entity(item_form).add_children(&[item_form_header, item_form_input, item_submit]);

    commands.entity(forms_container).add_children(&[character_form, item_form]);

    //add everything to the screen
    commands.entity(display).add_children(&[character_box, item_box, forms_container]);
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                } else {
                    inactive.0 = true;
                }
            }
        }
    }
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
