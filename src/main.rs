use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct Animations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
    run: Handle<AnimationClip>,
}

#[derive(Component, Default)]
struct ActionTimer(Timer);

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));

    let camera = Camera3dBundle {
        projection: PerspectiveProjection { ..default() }.into(),
        transform: Transform::from_xyz(11.0, 3.0, 11.0)
            .looking_at(Vec3::new(0.0, 4.0, 0.0), Vec3::Y)
            .with_scale(Vec3::splat(9.)),
        ..default()
    };
    commands.spawn(camera).insert(Player);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

    let guy = asset_server.load("guy.glb#Scene0");
    commands
        .spawn(SceneBundle {
            scene: guy.clone_weak(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::splat(1.0)),

            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.4, 1.0, 0.4));
    //.insert(ColliderDebugColor(Color::GREEN));
    commands.insert_resource(Animations {
        walk: asset_server.load("guy.glb#Animation3"),
        idle: asset_server.load("guy.glb#Animation1"),
        run: asset_server.load("guy.glb#Animation2"),
    });
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    
    for mut player in &mut players {
        println!("setup_scene_once_loaded");
        player.play(animations.idle.clone_weak()).repeat();
    }
}

fn process_input(
    keys: Res<Input<KeyCode>>,
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for mut animation_player in players.iter_mut() {
        if keys.just_pressed(KeyCode::W) {
            animation_player
                .play_with_transition(animations.run.clone(), Duration::from_secs_f32(0.5))
                .repeat();
        }
        if keys.just_released(KeyCode::W) {
            animation_player
                .play_with_transition(animations.idle.clone(), Duration::from_secs_f32(0.5))
                .repeat();
        }
    }
}

fn main() {
    App::new()
        /*/.insert_resource(WindowDescriptor {
            title: "Bob Ross".to_string(),
            width: 1024.,
            height: 512.,
            ..default()
        })*/
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Update, (setup_scene_once_loaded, process_input))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, setup_ground))
        //.add_system(controls)
        .run();
}
