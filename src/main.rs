use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello)
        .add_systems(Startup, setup)
        .run();
}

fn hello() {
    println!("Hello!")
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(5.0).into()),
        material: materials.add(Color::YELLOW_GREEN.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("alien.glb#Scene0"),
        transform: Transform::from_xyz(-1.0, 1.0, 3.0)
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI)),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
