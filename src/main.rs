//! How to use an external thread to run an infinite task and communicate with a channel.
use crate::forza7::plugin::{Forza7Plugin, StreamEvent};
pub mod forza7;
use bevy::prelude::*;
use bevy::window::{WindowPlugin, WindowResolution};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("b1b1b1").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(1200., 600.),
                title: "Forza telemetry".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(Forza7Plugin)
        .add_startup_system(setup)
        .add_system(update_telemetry)
        .run();
}

#[derive(Component)]
struct Speed;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font_family = asset_server.load("fonts/secularone.ttf");
    commands
        .spawn(NodeBundle {
            // Area
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            section_2(builder, font_family);
        });
}

fn section_2(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            render_speed(builder, font);
        });
}

fn render_speed(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(200.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::hex("d9d9d9").unwrap()),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(65.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    // TODO Check Text2dBundle
                    builder.spawn((TextBundle::from_section(
                        "000",
                        TextStyle {
                            font: font.clone(),
                            font_size: 154.0,
                            color: Color::BLACK,
                        },
                    ), Speed));
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(45.0), Val::Percent(100.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "KM",
                        TextStyle {
                            font: font.clone(),
                            font_size: 72.0,
                            color: Color::BLACK,
                        },
                    ));

                    builder.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(70.0), Val::Px(3.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    });
                    builder.spawn(TextBundle::from_section(
                        "H",
                        TextStyle {
                            font: font.clone(),
                            font_size: 72.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}


fn update_telemetry(
    mut reader: EventReader<StreamEvent>,
    mut query: Query<&mut Text, With<Speed>>
) {
    let mut text = query.single_mut();
    for event in reader.iter() {
        text.sections[0].value = (event.telemetry.engine.speed * 3.6).round().to_string();
    }
}

