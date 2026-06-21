use bevy::prelude::*;
use rand::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Coin;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Count {value: 0})
            .add_systems(Startup, init_coin);
    }
}

#[derive(Resource)]
pub struct Count {
    pub value: i16
}

#[derive(Resource)]
pub struct MaxCount {
    pub max_count: i16
}

fn init_coin(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();

    let coin_count = 5;

    commands.insert_resource(MaxCount {
        max_count: coin_count
    });

    for _ in 0..coin_count {
        let x = rng.gen_range(-10.0..20.0);
        let z = rng.gen_range(-10.0..20.0);
        let y = rng.gen_range(0.0..2.5);

        let pos = Vec3::new(x, y, z);

        let monet = (
            SceneBundle {
                scene: asset_server.load("monet.glb#Scene0"),
                transform: Transform {
                    translation: pos,
                    scale: Vec3::new(0.01, 0.01, 0.01),
                    ..default()
                },
                ..default()
            },
            Coin,
            Collider::ball(0.4),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Name::new("Monet")
        );

        commands.spawn(monet).with_children(|parent| {
            parent.spawn((
                SpotLightBundle {
                    spot_light: SpotLight {
                        color: Color::rgba(1.0, 0.96, 0.37, 1.0),
                        intensity: 800.0,
                        outer_angle: 0.4,
                        inner_angle: 0.3,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                },
                Name::new("MonetLight"),
            ));
        });
    }
}