use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));

    let camera = Camera3dBundle {
        projection: PerspectiveProjection { ..default() }.into(),
        transform: Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::new(0.0, 4.0, 0.0), Vec3::Y)
            .with_scale(Vec3::splat(9.)),
        ..default()
    };
    commands.spawn(camera).insert(Player);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let tree_handle = asset_server.load("guy.glb#Scene0");
    for x in -25..25 {
        for z in -25..25 {
            commands
                .spawn(SceneBundle {
                    scene: tree_handle.clone_weak(),
                    transform: Transform::from_translation(Vec3::new(2.0 * x as f32, 0.0, 2.0 * z as f32))
                        .with_scale(Vec3::splat(1.0)),

                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(0.4, 1.0, 0.4));
                //.insert(ColliderDebugColor(Color::GREEN));
        }
    }

    commands.insert_resource(Animations(vec![
        asset_server.load("guy.glb#Animation0"),
    ]));
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
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
        .add_systems(Update, setup_scene_once_loaded)
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        //.add_system(controls)
        .run();
}
