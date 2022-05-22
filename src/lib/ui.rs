use bevy::prelude::*;

pub fn defstyle(asset_server: &Res<AssetServer>) -> TextStyle { 
	TextStyle {
		font: asset_server.load("fonts/ubuntu.ttf"),
		font_size: 30.0,
		color: Color::WHITE,
	}
}

pub fn setup_ui_camera(mut commands: Commands) {
	commands.spawn_bundle(UiCameraBundle::default());
}

pub fn setup_ui(
	mut commands: Commands, 
	asset_server: Res<AssetServer>
) {	
	let ui_bundle = commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
    }).id();
    
    let child = commands.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                text: Text::with_section(
                                    "Poentaro: 0",
                                    defstyle(&asset_server),
                                    Default::default(),
                                ),
                                ..default()
                            });
                        });
                }).id();
    
    commands.entity(ui_bundle).push_children(&[child]);
}
