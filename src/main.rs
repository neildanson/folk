use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom, Offset};

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct Animations {
    idle: Handle<AnimationClip>,
    walk: Handle<AnimationClip>,
    walk_backwards: Handle<AnimationClip>,
    run: Handle<AnimationClip>,
    run_backwards : Handle<AnimationClip>,
    left_turn : Handle<AnimationClip>,
    right_turn : Handle<AnimationClip>,
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
        //transform: Transform::from_xyz(0.0, 50.0, 0.0),
        ..default()
    };
    let third_person_camera = ThirdPersonCamera {
        offset_enabled: true,
        offset: Offset::new(0.0, 2.0),
        zoom: Zoom::new(5.0, 10.0),
        ..default()
    };

    commands.spawn((third_person_camera, camera));

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
        .insert(Player)
        .insert(ThirdPersonCameraTarget)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.4, 1.0, 0.4))
        .insert(ColliderDebugColor(Color::GREEN))
        ;
    commands.insert_resource(Animations {
        idle: asset_server.load("guy.glb#Animation1"),
        left_turn: asset_server.load("guy.glb#Animation2"),
        right_turn: asset_server.load("guy.glb#Animation3"),
        run: asset_server.load("guy.glb#Animation4"),
        run_backwards: asset_server.load("guy.glb#Animation5"),
        walk: asset_server.load("guy.glb#Animation6"),
        walk_backwards: asset_server.load("guy.glb#Animation6"),
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
    mut animation_players: Query<(&Parent, &mut AnimationPlayer)>,
    parent_query: Query<&Parent>,//, With<Player>>,
    mut player: Query<&mut Transform>,//, With<Player>>,
) {
    for (parent, mut animation_player) in animation_players.iter_mut() {
        let transition_duration = Duration::from_secs_f32(0.1);
        if keys.just_pressed(KeyCode::W) && keys.pressed(KeyCode::ShiftLeft) {
            animation_player
                .play_with_transition(animations.run.clone(), transition_duration)
                .repeat();
        }

        if keys.pressed(KeyCode::W) && keys.just_pressed(KeyCode::ShiftLeft) {
            animation_player
                .play_with_transition(animations.run.clone(), transition_duration)
                .repeat();
        }

        if keys.just_pressed(KeyCode::W) && !keys.pressed(KeyCode::ShiftLeft) {
            animation_player
                .play_with_transition(animations.walk.clone(), transition_duration)
                .repeat();
        }

        if keys.pressed(KeyCode::W) && keys.just_released(KeyCode::ShiftLeft) {
            animation_player
                .play_with_transition(animations.walk.clone(), transition_duration)
                .repeat();
        }

        if keys.just_released(KeyCode::W) {
            animation_player
                .play_with_transition(animations.idle.clone(), transition_duration)
                .repeat();
        }
        //Should make this a function
        let parent_entity = parent_query.get(parent.get()).unwrap();
        let mut player  = player.get_mut(parent_entity.get()).unwrap();
        if keys.pressed(KeyCode::W) {
            if keys.pressed(KeyCode::ShiftLeft) {
                player.translation += Vec3::new(0.0, 0.0, 0.02);
            } else {
                player.translation += Vec3::new(0.0, 0.0, 0.01);
            }
        }

        if keys.just_pressed(KeyCode::A) {
            animation_player
                .play_with_transition(animations.left_turn.clone(), transition_duration);
        }

        if keys.pressed(KeyCode::A) {
            player.rotation *= Quat::from_rotation_y(0.005);
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
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, (setup_scene_once_loaded, process_input))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, setup_ground))
        //.add_system(controls)
        .run();
}
