use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Resource, Default, Debug)]
struct Data {
    circle: usize,
    square: usize,
    pi: f64,
}

#[derive(Component)]
struct Droplet {
    timer: Timer,
}

#[derive(Component)]
struct Circle;

#[derive(Component)]
struct Square;

fn main() {
    App::new()
        .init_resource::<AssetsLoading>()
        .add_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(Data::default())
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_load_assets))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_level))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_droplets)
                .with_system(despawn_droplets)
                .with_system(check_collisions)
                .with_system(update_ui)
        )
        .run();
}

fn load_assets(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let cyl_mesh: Handle<Mesh> = asset_server.load("containers.glb#Mesh0/Primitive0");
    let box_mesh: Handle<Mesh> = asset_server.load("containers.glb#Mesh1/Primitive0");

    loading.0.push(cyl_mesh.clone_untyped());
    loading.0.push(box_mesh.clone_untyped());
}

fn check_load_assets(
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {}
        LoadState::Loaded => {
            app_state.set(AppState::InGame).unwrap();
        }
        _ => {
            info!("loading assets");
        }
    }
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    loading: Res<AssetsLoading>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 2.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 12.0, 9.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(12.0, 1.0, 9.0))),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed);

    // circle container
    let mesh_handle: Handle<Mesh> = loading.0[0].clone().typed::<Mesh>();
    let mesh = &meshes.get(&mesh_handle);
    commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW_GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(-2.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::from_bevy_mesh(mesh.unwrap(), &ComputedColliderShape::TriMesh).unwrap())
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(0b0010),
            Group::from_bits_truncate(0b0011),
        ));

    // circle collider
    commands
        .spawn(Collider::cylinder(0.01, 3.0))
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(0b0100),
            Group::from_bits_truncate(0b0101),
        ))
        .insert(Sensor)
        .insert(GlobalTransform::from_xyz(-2.0, 1.0, 0.0))
        .insert(RigidBody::Fixed)
        .insert(Circle);

    // square container
    let mesh_handle: Handle<Mesh> = loading.0[1].clone().typed::<Mesh>();
    let mesh = &meshes.get(&mesh_handle);
    commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_handle,
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                ..default()
            }),
            transform: Transform::from_xyz(3.5, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::from_bevy_mesh(mesh.unwrap(), &ComputedColliderShape::TriMesh).unwrap())
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(0b0010),
            Group::from_bits_truncate(0b0011),
        ));

    // square collider
    commands
        .spawn(Collider::cuboid(1.5, 0.005, 1.5))
        .insert(CollisionGroups::new(
            Group::from_bits_truncate(0b0100),
            Group::from_bits_truncate(0b0101),
        ))
        .insert(Sensor)
        .insert(TransformBundle::from(Transform::from_xyz(3.5, 1.0, 0.0)))
        .insert(RigidBody::Fixed)
        .insert(Square);

    commands.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.),
                left: Val::Px(10.),
                ..default()
            },
            ..default()
        },
        text: Text::from_sections([
            TextSection {
                value: "0".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::YELLOW_GREEN,
                },
                ..default()
            },
            TextSection {
                value: " / ".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                ..default()
            },
            TextSection {
                value: "0".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::ORANGE_RED,
                },
                ..default()
            },
            TextSection {
                value: " = 0".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                ..default()
            },
        ]),
        ..default()
    });
}

fn spawn_droplets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // increase this for more rain
    for _ in 0..5 {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.15,
                    depth: 0.0,
                    ..default()
                })),
                transform: Transform::from_xyz(
                    thread_rng().gen_range(-7.5..=7.5),
                    15.0,
                    thread_rng().gen_range(-5.0..=5.0),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.15))
            .insert(CollisionGroups::new(
                Group::from_bits_truncate(0b0001),
                Group::from_bits_truncate(0b0110),
            ))
            .insert(Ccd::enabled())
            .insert(Restitution::coefficient(0.1))
            .insert(Droplet {
                timer: Timer::from_seconds(20.0, TimerMode::Once),
            });
    }
}

fn check_collisions(
    mut data: ResMut<Data>,
    rapier_context: Res<RapierContext>,
    mut q_droplet: Query<(Entity, &mut Droplet, &mut CollisionGroups)>,
    q_circle: Query<Entity, With<Circle>>,
    q_square: Query<Entity, With<Square>>,
) {
    let circle = q_circle.get_single().unwrap();
    let square = q_square.get_single().unwrap();

    for (drop_ent, mut droplet, mut col_grp) in q_droplet.iter_mut() {
        if rapier_context.intersection_pair(drop_ent, circle) == Some(true) {
            data.circle += 1;
            col_grp.filters = Group::from_bits_truncate(0b0010);
            droplet.timer = Timer::from_seconds(3.0, TimerMode::Once);
        }

        if rapier_context.intersection_pair(drop_ent, square) == Some(true) {
            data.square += 1;
            col_grp.filters = Group::from_bits_truncate(0b0010);
            droplet.timer = Timer::from_seconds(3.0, TimerMode::Once);
        }
    }

    if data.square > 0 {
        data.pi = data.circle as f64 / data.square as f64;
        // println!("{:?}/{:?} = {:?}", data.circle, data.square, data.pi);
    }
}

fn despawn_droplets(
    mut commands: Commands,
    mut q_droplet: Query<(Entity, &mut Droplet, &Transform)>,
    time: Res<Time>,
) {
    for (drop_ent, mut droplet, trans) in q_droplet.iter_mut() {
        droplet.timer.tick(time.delta());
        if droplet.timer.just_finished() || trans.translation.y < -1.0 {
            commands.entity(drop_ent).despawn();
        }
    }
}

fn update_ui(mut q_text: Query<&mut Text>, data: Res<Data>) {
    let mut text = q_text.get_single_mut().unwrap();
    text.sections[0].value = format!("{}", data.circle);
    text.sections[2].value = format!("{}", data.square);
    text.sections[3].value = format!(" = {:.4}", data.pi);
}
