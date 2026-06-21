use bevy::prelude::*;
use bevy::window::*;
use bevy_rapier3d::prelude::*;

mod player;
mod camera;
mod cursor;
mod coin;
mod game;

use crate::player::player::*;
use crate::camera::camera::*;
use crate::cursor::cursor::*;
use crate::coin::coin::*;
use crate::game::game::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin {
            enabled: true,
            ..default()
        },
        DefaultPlugins.set(WindowPlugin {
            primary_window : Some(Window {
                resolution: WindowResolution::new(800.0, 800.0),
                title: "Coin collector 3d".into(),
                ..default()
            }),
            ..default()
        }),
        PlayerPlugin,
        Camera3dPlugin,
        CursorPlugin,
        CoinPlugin,
        GamePlugin,
    ))
        .add_systems(Startup, setup);
    app.add_state::<GameState>();
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_w = 60.0;
    let floor_h = 1.0;
    let floor_d = 60.0;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(floor_w, floor_h, floor_d))),
            material: materials.add(Color::rgb(0.2, 0.7, 0.2).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        Collider::cuboid(floor_w / 2.0, floor_h / 2.0, floor_d / 2.0),
        Name::new("Floor"),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}