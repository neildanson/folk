use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

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

    let tree_handle = asset_server.load("guy.glb#Scene0");
    for x in -25..25 {
        for z in -25..25 {
            commands
                .spawn(SceneBundle {
                    scene: tree_handle.clone_weak(),
                    transform: Transform::from_translation(Vec3::new(
                        2.0 * x as f32,
                        0.0,
                        2.0 * z as f32,
                    ))
                    .with_scale(Vec3::splat(1.0)),

                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(0.4, 1.0, 0.4));
            //.insert(ColliderDebugColor(Color::GREEN));
        }
    }

    let animations : Vec<Handle<AnimationClip>>= 
        (1..=4)
        .map(|i| format!("guy.glb#Animation{:?}", i))
        .map(|name| {
            println!("Loading {}", name);
            asset_server.load(name)
        }
        )
        .into_iter()
        .collect::<Vec<_>>();
    commands.insert_resource(Animations(animations));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    //1 Idle
    //2 Run
    //3 Walk
    for mut player in &mut players {
        player.play(animations.0[2].clone_weak()).repeat();
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
        .add_systems(Update, (setup_scene_once_loaded, setup_ground))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        //.add_system(controls)
        .run();
}
