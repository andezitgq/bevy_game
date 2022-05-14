use bevy::prelude::*;
use bevy::window::*;
use bevy_obj::*;

//Derivo de Komponantoj
#[derive(Component)]
struct XP(u16);
#[derive(Component)]
struct PlayerName(String);
#[derive(Component)]
struct Health(u8);
#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    xp: XP,
    name: PlayerName,
    health: Health,
    _p: Player,
    
    #[bundle]
    pbr: PbrBundle,
}

fn main() {
	App::new()
		.insert_resource(WindowDescriptor{
			title: "Kampludejo".to_string(),
			resizable: true,
			decorations: false,
			mode: WindowMode::BorderlessFullscreen,
			cursor_locked: false,
			cursor_visible: true,
			present_mode: PresentMode::Mailbox,
			..default()
		})
		.insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
		.add_plugins(DefaultPlugins)
		.add_plugin(ObjPlugin)
		.add_startup_system(spawn_scene)
		.add_system(control_character)
		.run();
}

fn spawn_scene(
	mut commands: Commands,	
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
	//Ludkampo
	commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 4.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    //Lumo
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
	//Kamerao
	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
	});
	//Ludanto
	commands.spawn_bundle(PlayerBundle {
		xp: XP(0),
		name: PlayerName(String::from("Player123")),
		health: Health(100),
		_p: Player,
		pbr: PbrBundle {
			mesh: assets.load("models/player.obj"),
			material: materials.add(diffuse_mat("textures/player.jpg", &assets)),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..default()
		},
	});	
}

fn control_character(
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut query: Query<(&Health, &mut Transform, Option<&Player>)>,
){
	
	let (_health, mut transform, _player) = query.single_mut();
	
	if keys.pressed(KeyCode::Up)    { transform.translation.z += 2.0 * time.delta_seconds();}
	if keys.pressed(KeyCode::Down)  { transform.translation.z -= 2.0 * time.delta_seconds();}
	if keys.pressed(KeyCode::Left)  { transform.translation.x += 2.0 * time.delta_seconds();}
	if keys.pressed(KeyCode::Right) { transform.translation.x -= 2.0 * time.delta_seconds();}
	
}

fn diffuse_mat(path: &str, assets: &Res<AssetServer>) -> StandardMaterial {
	StandardMaterial {
		base_color_texture: Some(assets.load(path)),
		alpha_mode: AlphaMode::Blend,
        unlit: true,
		..default()
	}
}
