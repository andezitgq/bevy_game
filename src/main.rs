pub mod lib;

use std::f32::consts::PI as pi;
use std::ops::Mul;
use bevy::prelude::*;
use bevy::window::*;
use bevy::gltf::{Gltf, GltfMesh, GltfExtras};
use bevy::render::render_resource::{SamplerDescriptor, FilterMode};
use bevy_obj::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;
use serde_json::Value;

use lib::orbit_camera::*;
use lib::ui::*;
use lib::menu::*; 

//Derivo de Komponantoj
#[derive(Default)]
struct ColliderMeshes(Handle<Gltf>);

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
			mode: WindowMode::BorderlessFullscreen,
			cursor_locked: false,
			cursor_visible: true,
			present_mode: PresentMode::Mailbox,
			..default()
		})
		.insert_resource(Screen(0.0, 0.0))
		.insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        
		.add_plugins(DefaultPlugins)
		.add_plugin(ObjPlugin)
		.add_plugin(EguiPlugin)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        
        .add_loopless_state(GameState::MainMenu) 

        .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
        
        .add_enter_system(GameState::InGame, setup)
        .add_enter_system(GameState::InGame, spawn_camera)
        .add_enter_system(GameState::InGame, setup_ui)
        .add_exit_system(GameState::InGame, despawn_with::<InGame>)
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(main_menu)
                .into()
        )
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(scene_processing)
                .with_system(spawn_coins)
                .with_system(control_character)
				.with_system(pan_orbit_camera)
				.with_system(get_coin)
                .into()
        )
        
        .add_system(texture_filtering)
        .add_system(setup_ui_camera)
        .add_system(screen_size)

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
    .insert(Sensor(false))
    .insert(Ground);
    
    let gltf: Handle<Gltf> = assets.load("scenes/scene1.glb");
    commands.insert_resource(ColliderMeshes(gltf));
    
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
}

fn scene_processing(
	mut commands: Commands,
	mut er_gltf: EventReader<AssetEvent<Gltf>>,
	mut er_gltfmesh: EventReader<AssetEvent<GltfMesh>>,
	cmeshes: Res<ColliderMeshes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets_mesh: Res<Assets<Mesh>>,
){
	for ev in er_gltf.iter() {
		if let AssetEvent::Created { handle } = ev {
			let scene = assets_gltf.get(handle).unwrap();
			
			if *handle == cmeshes.0 {
				commands.spawn_scene(scene.scenes[0].clone());
				for gltfmesh in scene.meshes.iter() {
					let gltfmesh = assets_gltfmesh.get(gltfmesh);
					if let Some(gltfmesh) = gltfmesh {
						for primitive in gltfmesh.primitives.iter() {
							let mesh = assets_mesh.get(primitive.mesh.clone());
							if let Some(mesh) = mesh {
								if let Some(collider) = Collider::bevy_mesh(&mesh) {
									commands.spawn()
									.insert(collider);
								}
							}
						}
					}
				}
			}
		}
	}
}

fn spawn_coins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Transform, &GltfExtras), Added<GltfExtras>>
){
    for (t, gltf_extras) in query.iter() {
        let v: Value = serde_json::from_str(&gltf_extras.value).expect("Couldn't parse GltfExtra value as JSON");
		if v["type"].as_str() == Some("coin") {
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
						translation: t.translation,
						rotation: Quat::from_axis_angle(Vec3::X, radian(0.0)),
						scale: Vec3::new(1.0, 1.0, 1.0),
					},
					..default()
				},
			});
		}
    }
}


fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 16.0, -16.0);
    let radius = translation.length();

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(PanOrbitCamera {
        radius,
        ..Default::default()
    });
}

fn control_character(
	keys: Res<Input<KeyCode>>,
	mut camera_query: Query<(&mut PanOrbitCamera, &Transform), Without<Player>>,
	mut player_query: Query<(&Health, &Transform, &mut ExternalForce, &mut ExternalImpulse, Entity, &mut IsGround, &Player)>,
	mut ground_query: Query<(Entity, &Ground)>,
	mut collision_events: EventReader<CollisionEvent>,
){	
	let (_health, transform, mut _player_force, mut _player_impulse, player_ent, mut is_ground, _player) = player_query.single_mut();
	let (mut poc, camera_transform) = camera_query.single_mut();
	
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
	
	let ct0 = Vec3::new(camera_transform.translation.x,			//
						camera_transform.translation.y - 16.0,	//Kameraa Transformo je NULA Alteco
						camera_transform.translation.z); 		//
	let direct_vector = Vec3::new((transform.translation.x - ct0.x) / 16.0,
								  0.0,
								  (transform.translation.z - ct0.z) / 16.0);
	let perp_vector = Quat::from_axis_angle(Vec3::Y, radian(90.0)).mul(direct_vector);
	
	if keys.pressed(KeyCode::W) { _player_impulse.impulse = direct_vector;}
	if keys.pressed(KeyCode::S) { _player_impulse.impulse = -direct_vector;}
	if keys.pressed(KeyCode::A) { _player_impulse.impulse = perp_vector;}
	if keys.pressed(KeyCode::D) { _player_impulse.impulse = -perp_vector;}
	
	if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::A) { _player_impulse.impulse = direct_vector + perp_vector;}
	if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::A) { _player_impulse.impulse = perp_vector - direct_vector;}
	if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::D) { _player_impulse.impulse = direct_vector - perp_vector;}
	if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::D) { _player_impulse.impulse =-direct_vector - perp_vector;}
       
    if keys.just_pressed(KeyCode::Space) && is_ground.0 == true { _player_impulse.impulse = Vec3::new(0.0, 25.0, 0.0);}
        
    poc.focus = transform.translation;
}

fn get_coin(
	mut commands: Commands,
	mut q_player: Query<(Entity, &mut XP, &Player)>,
	mut q_coin: Query<(Entity, &Coin)>,
	mut q_ui: Query<&mut Text>,
	mut collision_events: EventReader<CollisionEvent>,
	assets: Res<AssetServer>,
){
	let mut text = q_ui.single_mut();
	let (player_ent, mut xp, _player) = q_player.single_mut();
	
	for collision_event in collision_events.iter() {
		if let CollisionEvent::Started(ent1, ent2, _flags) = collision_event {
			for (coin_ent, _coin) in q_coin.iter_mut() {
				if (&player_ent == ent1 && &coin_ent == ent2) || (&player_ent == ent2 && &coin_ent == ent1) {
					xp.0 += 1;
					text.sections[0] = TextSection {
							value: String::from("Poentaro: ") + &xp.0.to_string(),
							style: defstyle(&assets),
							..default()
						};
					commands.entity(coin_ent).despawn();
				}
			}
		}
	}
}

fn texture_filtering(
	mut tex_events: EventReader<AssetEvent<Image>>,
	mut images: ResMut<Assets<Image>>
) {
	let desc = SamplerDescriptor {
		mag_filter: FilterMode::Linear,
		min_filter: FilterMode::Linear,
		mipmap_filter: FilterMode::Linear,
		
		..default()
	};
		
	for event in tex_events.iter() {
		if let AssetEvent::Created{handle} = event {
			if let Some(txt) = images.get_mut(handle) {
				txt.sampler_descriptor = desc.clone();
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
