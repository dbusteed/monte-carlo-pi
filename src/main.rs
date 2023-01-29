use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::{thread_rng, Rng};

#[derive(Component)]
struct Droplet;

#[derive(Component)]
struct Circle;

#[derive(Component)]
struct Square;

#[derive(Resource, Default, Debug)]
struct Data {
    circle: usize,
    square: usize,
    pi: f64,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(WorldInspectorPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(Data::default())
        .add_startup_system(setup)
        .add_system(rain)
        .add_system(check_collisions)
        .add_system(despawn_droplets)
        .add_system(update_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 13.0, 2.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(14.0, 1.0, 8.0))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::new(3.0))),
        transform: Transform {
            translation: Vec3::new(-3.5, 1.0, 0.0),
            rotation: Quat::from_rotation_x(f32::to_radians(-90.0)),
            scale: Vec3::ONE,
        },
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // splitting the Circle sprite mesh and the collider cause of rotation issues
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(0.0))),
            transform: Transform::from_xyz(-3.5, 1.0, 0.0),
            // material: materials.add(StandardMaterial {
            //     base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
            //     ..default()
            // }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cylinder(0.01, 3.0))
        .insert(Sensor)
        .insert(Circle);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(3.0, 0.01, 3.0))),
            transform: Transform::from_xyz(3.5, 1.0, 0.0),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.5, 0.005, 1.5))
        .insert(Sensor)
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
        text: Text::from_section(
            "0 / 0 = 0.0",
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                font_size: 30.0,
                color: Color::BLACK,
            },
        ),
        ..default()
    });
}

fn rain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // keyboard: Res<Input<KeyCode>>,
) {
    for _ in 0..5 {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.1,
                    depth: 0.0,
                    ..default()
                })),
                transform: Transform::from_xyz(
                    thread_rng().gen_range(-7.5..=7.5),
                    15.0,
                    thread_rng().gen_range(-4.5..=4.5),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.1))
            .insert(Droplet);
    }
}

fn check_collisions(
    mut commands: Commands,
    mut data: ResMut<Data>,
    rapier_context: Res<RapierContext>,
    q_droplet: Query<Entity, With<Droplet>>,
    q_circle: Query<Entity, With<Circle>>,
    q_square: Query<Entity, With<Square>>,
) {
    let circle = q_circle.get_single().unwrap();
    let square = q_square.get_single().unwrap();

    for droplet in q_droplet.iter() {
        if rapier_context.intersection_pair(droplet, circle) == Some(true) {
            data.circle += 1;
            commands.entity(droplet).despawn();
        }

        if rapier_context.intersection_pair(droplet, square) == Some(true) {
            data.square += 1;
            commands.entity(droplet).despawn();
        }
    }

    if data.square > 0 {
        data.pi = data.circle as f64 / data.square as f64;
    }
}

fn despawn_droplets(mut commands: Commands, q_droplet: Query<(Entity, &Transform), With<Droplet>>) {
    for (drop, trans) in q_droplet.iter() {
        if trans.translation.y < -1.0 {
            commands.entity(drop).despawn();
        }
    }
}

fn update_ui(mut q_text: Query<&mut Text>, data: Res<Data>) {
    let mut text = q_text.get_single_mut().unwrap();
    text.sections[0].value = format!("{} / {} = {:.4}", data.circle, data.square, data.pi);
}
