use std::f32::consts::{FRAC_PI_2, PI};
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::*;

#[derive(Default, Debug)]
struct ex(usize);

fn main() {
    App::new()
		// RESURSOJ
		.insert_resource(WindowDescriptor{
			title: "Bevia Ludo ĉĝĥĵŝŭ".to_string(),
			resizable: true,
			decorations: false,
			mode: WindowMode::BorderlessFullscreen,
			cursor_locked: false,
			cursor_visible: true,
			present_mode: PresentMode::Mailbox,
			..default()
		})
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(ex(0))
        .add_plugins(DefaultPlugins)
        
        // ARANĜO
        .add_startup_system(setup)
        
        // CIKLO
        .add_system(animate_light_direction)
        .add_system(cursor_grab_system)
        .add_system(exit_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/AntiqueCamera.glb#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    key: Res<Input<KeyCode>>,
    
) {
    
}
