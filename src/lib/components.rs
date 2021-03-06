use bevy::prelude::*;
use bevy::gltf::Gltf;
use bevy_rapier3d::prelude::*;
use serde_derive::Deserialize;


//==========STATES==============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    GameLoading,
    InGame,
}

#[derive(Component)]
pub struct Splash;

#[derive(Component)]
pub struct Fade {
	pub is_faded: bool,
}

#[derive(Component)]
pub struct SplashTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct InGame;

#[derive(Default, PartialEq, Clone)]
pub struct Pause(pub bool);

#[derive(Default)]
pub struct Win;

#[derive(Default)]
pub struct GameOver;


//==========DUMMIES==============

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerChild;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct FinishTrigger;

#[derive(Component)]
pub struct DamageTrigger;


//==========RES&COMPS==============
#[derive(Default)]
pub struct MenuAssets {
    pub menu_music: Handle<bevy_kira_audio::AudioSource>,
    pub game_music: Handle<bevy_kira_audio::AudioSource>,
    pub menu_scene: Handle<Scene>,
}

#[derive(Deserialize)]
pub struct PlayerTOML {
    pub playerinfo: PlayerInfo,
}

#[derive(Deserialize)]
pub struct PlayerInfo {
	pub score: usize,
}

#[derive(Default)]
pub struct CurrentLevel(pub String);

#[derive(Default)]
pub struct LevelDialog(pub bool);

#[derive(Default)]
pub struct Screen(pub f32, pub f32);

#[derive(Default)]
pub struct GltfMeshes (pub Handle<Gltf>);

#[derive(Default)]
pub struct LoadedMeshes(pub Vec<Handle<Mesh>>);

#[derive(Component)]
pub struct XP(pub u16);

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct IsGround(pub bool);

#[derive(Bundle)]
pub struct PhysicsBundle {
	pub rigidbody: RigidBody,
	pub collider: Collider,
	pub sensor: Sensor,
	pub friction: Friction,
	pub restitution: Restitution,
	pub is_ground: IsGround,
	
	pub velocity: Velocity,
	pub gravity: GravityScale,
	pub mass_properties: MassProperties,
	pub locked_axes: LockedAxes,
	pub dominance: Dominance,
	pub sleeping: Sleeping,
	pub damping: Damping,
	pub ccd: Ccd,
	pub act: ActiveCollisionTypes,
	pub events: ActiveEvents,
	
	pub force: ExternalForce,
	pub impulse: ExternalImpulse,
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
pub struct PlayerBundle {
    pub xp: XP,
    pub health: Health,
    pub _p: Player,
	
	#[bundle]
	pub physics: PhysicsBundle,
    
    #[bundle]
    pub pbr: PbrBundle,
}

#[derive(Bundle)]
pub struct CoinBundle {
	pub _c: Coin,
	
	#[bundle]
	pub physics: PhysicsBundle,
	
	#[bundle]
	pub pbr: PbrBundle,
}
