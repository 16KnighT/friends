use bevy::{prelude::*, ui::FocusPolicy};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSystem,
    TextInputTextColor, TextInputTextFont,
};

use crate::DebugButtonActions;

pub fn spawn_text(
    commands: &mut Commands,
    text: &str,
    size: f32,
) -> Entity {

    return commands.spawn((
            Text::new(text),
                TextFont {
                    font_size: size,
                    ..default()
                },
                Label,
    )).id();

}

pub fn spawn_container(
    commands: &mut Commands,
    height_percent: f32,
    width_percent: f32,
    flex_direction: FlexDirection,
    background_colour: Option<Color>,
) -> Entity {

    //if let colour = Some(background_colour) {

    //}

    let container =  commands.spawn((
        Node {
            height: Val::Percent(height_percent),
            width: Val::Percent(width_percent),
            border: UiRect::all(Val::Px(2.)),
            flex_direction: flex_direction,
            ..default()
        },
    )).id();

    if let Some(colour) = background_colour {
        commands.entity(container).insert(BackgroundColor(colour));
    }

    return container;

}


pub fn spawn_text_input(
    commands: &mut Commands,
    width_px: f32,
    placeholder: &str,
    font_size: f32,
) -> Entity {

    return commands.spawn((
        Node {
            width: Val::Px(width_px),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font_size: font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(Color::WHITE)),
        TextInputPlaceholder {
            value: placeholder.to_string(),
            ..default()
        },
        TextInputInactive(true),
    )).id();

}

pub fn spawn_button(
    commands: &mut Commands,
    text: &str,
    button_action: DebugButtonActions,
) -> Entity {

    let button = commands.spawn((
        Button,
        DebugButtonActions::CreateCharacter,
        Node {
            width: Val::Px(50.0),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )).with_child((
        Text::new(text),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )).id();

    commands.entity(button).insert(button_action);
    return button;
}