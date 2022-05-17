use std::f32::consts::PI as pi;
use std::f32;
use bevy::prelude::*;
use bevy::window::*;
use bevy::input::mouse::MouseMotion;
use bevy_obj::*;
use bevy_rapier3d::prelude::*;

//Derivo de Komponantoj
#[derive(Default)]
struct Rotation {
	alpha: f32,
	beta: f32,
}

#[derive(Component)]
struct XP(u16);

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct Health(u8);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct IsGround(bool);

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Coin;

#[derive(Component)]
struct PlayerEmpty;

#[derive(Bundle)]
struct PhysicsBundle {
	rigidbody: RigidBody,
	collider: Collider,
	sensor: Sensor,
	friction: Friction,
	restitution: Restitution,
	is_ground: IsGround,
	
	velocity: Velocity,
	gravity: GravityScale,
	mass_properties: MassProperties,
	locked_axes: LockedAxes,
	dominance: Dominance,
	sleeping: Sleeping,
	damping: Damping,
	ccd: Ccd,
	act: ActiveCollisionTypes,
	events: ActiveEvents,
	
	force: ExternalForce,
	impulse: ExternalImpulse,
}

impl Default for PhysicsBundle {
	fn default() -> PhysicsBundle {
		return PhysicsBundle {
			rigidbody: RigidBody::Fixed,
			collider: Collider::cuboid(1.0, 1.0, 1.0),
			restitution: Restitution::default(),
			sensor: Sensor(true),
			friction: Friction::default(),
			is_ground: IsGround(false),
			
			gravity: GravityScale(1.0),
			velocity: Velocity {
				linvel: Vec3::new(0.0, 0.0, 0.0),
				angvel: Vec3::new(0.0, 0.0, 0.0),
			},
			mass_properties: MassProperties::default(),
			locked_axes: LockedAxes::default(),
			damping: Damping::default(),
			dominance: Dominance::group(0),
			sleeping: Sleeping::disabled(),
			ccd: Ccd::enabled(),
			act: ActiveCollisionTypes::default(),
			events: ActiveEvents::COLLISION_EVENTS,
			
			force: ExternalForce::default(),
			impulse: ExternalImpulse::default(),
		}
	}
}

#[derive(Bundle)]
struct PlayerBundle {
    xp: XP,
    name: PlayerName,
    health: Health,
    _p: Player,
	
	#[bundle]
	physics: PhysicsBundle,
    
    #[bundle]
    pbr: PbrBundle,
}

#[derive(Bundle)]
struct CoinBundle {
	_c: Coin,
	
	#[bundle]
	physics: PhysicsBundle,
	
	#[bundle]
	pbr: PbrBundle,
}

fn main() {
	App::new()
		.insert_resource(WindowDescriptor{
			title: "Kampludejo".to_string(),
			resizable: true,
			decorations: false,
			mode: WindowMode::Windowed,
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
        .insert_resource(Rotation {alpha: 0.0, beta: 0.0})
		.add_plugins(DefaultPlugins)
		.add_plugin(ObjPlugin)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
		.add_startup_system(setup)
		.add_system(control_character)
		.add_system(get_coin)
		.run();
}

fn setup(
	mut windows: ResMut<Windows>,
	mut commands: Commands,	
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
	let window = windows.primary_mut();
	window.set_cursor_lock_mode(true);
	window.set_cursor_visibility(false);
	
	//Ludkampo
    commands.spawn()
    .insert(Collider::cuboid(64.0, 0.5, 64.0))
    .insert(Sensor(true))
    .insert(Ground);
    
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 128.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(64.0, 0.1, 64.0))
    .insert(Ground);
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
	//Ludanto
	commands.spawn_bundle(PlayerBundle {
		xp: XP(0),
		name: PlayerName(String::from("Player123")),
		health: Health(100),
		_p: Player,
		
		physics: PhysicsBundle {
			rigidbody: RigidBody::Dynamic,
			collider: Collider::ball(1.0f32),
			sensor: Sensor(false),
			
			restitution: Restitution::coefficient(0.7),
			dominance: Dominance::group(2),
			..default()
		},
		
		pbr: PbrBundle {
			mesh: assets.load("models/player.obj"),
			material: materials.add(diffuse_mat("textures/player.jpg", &assets)),
			transform: Transform::from_xyz(0.0, 4.0, 0.0),
			..default()
		},
	});
	
	let empty_child = commands.spawn_bundle(TransformBundle {
		local: Transform::from_xyz(0.0, 0.0, 0.0),
		..default()
	})
	.insert(PlayerEmpty)
	.id();
	
	let camera_child = commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(0.0, 16.0, -16.0),
        ..default()
	})
	.id();
	
	commands.entity(empty_child).push_children(&[camera_child]);
	
	//Moneroj
	commands.spawn_bundle(CoinBundle {
		_c: Coin,
		
		physics: PhysicsBundle {
			collider: Collider::cuboid(0.6, 0.2, 0.6),
			..default()
		},
		
		pbr: PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Torus {
				radius: 0.5,
				ring_radius: 0.1,
				subdivisions_segments: 16,
				subdivisions_sides: 16,
			})),
			material: materials.add(golden_mat()),
			transform: Transform {
				translation: Vec3::new(4.0, 2.0, 4.0),
				rotation: Quat::from_axis_angle(Vec3::X, radian(0.0)),
				scale: Vec3::new(1.0, 1.0, 1.0),
			},
			..default()
		},
	});

}

fn control_character(
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut rotation: ResMut<Rotation>,
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
	mut empty_query:  Query<&mut Transform, (With<PlayerEmpty>, Without<Player>, Without<Camera>)>,
	mut player_query: Query<(&Health, &Transform, &mut ExternalForce, &mut ExternalImpulse, Entity, &mut IsGround, &Player)>,
	mut ground_query: Query<(Entity, &Ground)>,
	mut collision_events: EventReader<CollisionEvent>,
	mut mouse_events: EventReader<MouseMotion>,
){	
	let (_health, transform, mut _player_force, mut _player_impulse, player_ent, mut is_ground, _player) = player_query.single_mut();
	let mut empty_transform  = empty_query.single_mut();
	let mut camera_transform = camera_query.single_mut();
	
	for (ground_ent, _ground) in ground_query.iter_mut() {
		for collision_event in collision_events.iter() {
			if let CollisionEvent::Started(ent1, ent2, _flags) = collision_event {
				if (ground_ent.eq(ent1) && player_ent.eq(ent2)) || (ground_ent.eq(ent2) && player_ent.eq(ent1)) {
					is_ground.0 = true;
				}
			}
			
			if let CollisionEvent::Stopped(ent1, ent2, _flags) = collision_event {
				if (ground_ent.eq(ent1) && player_ent.eq(ent2)) || (ground_ent.eq(ent2) && player_ent.eq(ent1)) {
					is_ground.0 = false;
				}
			}
		}
	}
	
	if keys.pressed(KeyCode::Up)    { _player_impulse.impulse = Vec3::new(0.0, 0.0, 1.0);}
	if keys.pressed(KeyCode::Down)  { _player_impulse.impulse = Vec3::new(0.0, 0.0, -1.0);}
	if keys.pressed(KeyCode::Left)  { _player_impulse.impulse = Vec3::new(1.0, 0.0, 0.0);}
	if keys.pressed(KeyCode::Right) { _player_impulse.impulse = Vec3::new(-1.0, 0.0, 0.0);}
	
	if keys.pressed(KeyCode::Up) && keys.pressed(KeyCode::Left) 	{ _player_impulse.impulse = Vec3::new(1.0, 0.0, 1.0);}
	if keys.pressed(KeyCode::Down) && keys.pressed(KeyCode::Left)   { _player_impulse.impulse = Vec3::new(1.0, 0.0, -1.0);}
	if keys.pressed(KeyCode::Up) && keys.pressed(KeyCode::Right)    { _player_impulse.impulse = Vec3::new(-1.0, 0.0, 1.0);}
	if keys.pressed(KeyCode::Down) && keys.pressed(KeyCode::Right)  { _player_impulse.impulse = Vec3::new(-1.0, 0.0, -1.0);}
       
    if keys.just_pressed(KeyCode::Space) && is_ground.0 == true { _player_impulse.impulse = Vec3::new(0.0, 25.0, 0.0);}			
	
	camera_transform.rotation = camera_transform.looking_at(Vec3::ZERO, Vec3::Z).rotation;
	empty_transform.translation = transform.translation;
	
	for ev in mouse_events.iter() {
		rotation.alpha += radian(ev.delta.y);
		rotation.beta  += radian(ev.delta.x);
				
		/*empty_transform.rotate(Quat::from_axis_angle(Vec3::new(
		
												f32::cos(rotation.alpha),
												0.0,
												f32::sqrt(1.0 - (f32::cos(rotation.alpha)*f32::cos(rotation.alpha))),
		
																), radian(ev.delta.y)));   //ŝanĝo laŭ X
																
		empty_transform.rotate(Quat::from_axis_angle(Vec3::new(
		
												0.0,
												f32::cos(rotation.beta),
												f32::sqrt(1.0 - (f32::cos(rotation.beta)*f32::cos(rotation.beta))),
		
																), radian(-ev.delta.x)));	  //ŝanĝo laŭ Y*/
																
		empty_transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), radian(-ev.delta.x)));
		
    }
}

fn get_coin(
	mut commands: Commands,
	mut q_player: Query<(Entity, &mut XP, &Player)>,
	mut q_coin: Query<(Entity, &Coin)>,
	mut collision_events: EventReader<CollisionEvent>,
){

	let (player_ent, mut xp, _player) = q_player.single_mut();
	for (coin_ent, _coin) in q_coin.iter_mut() {
		for collision_event in collision_events.iter() {
			if let CollisionEvent::Started(ent1, ent2, _flags) = collision_event {
				if (player_ent.eq(ent1) && coin_ent.eq(ent2)) || (player_ent.eq(ent2) && coin_ent.eq(ent1)) {
					xp.0 += 1;
					commands.entity(coin_ent).despawn();
				}
			}
		}
	}
	
}

fn diffuse_mat(path: &str, assets: &Res<AssetServer>) -> StandardMaterial {
	StandardMaterial {
		base_color_texture: Some(assets.load(path)),
		alpha_mode: AlphaMode::Blend,
        unlit: true,
		..default()
	}
}

fn golden_mat() -> StandardMaterial {
	return StandardMaterial {
		base_color: Color::GOLD,
		perceptual_roughness: 0.2,
		metallic: 1.0,
		reflectance: 1.0,
		unlit: false,
		double_sided: false,
		..default()
	};
}

fn _print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn radian(deg: f32) -> f32 {
	return deg * pi / 180.0; 
}

