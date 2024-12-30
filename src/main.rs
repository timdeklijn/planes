use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // NOTE: this will have a transform so if the meshes are queried, make
    // sure the camera will not be found.
    commands.spawn(Camera2d);

    let plane_shape = Triangle2d::new(
        Vec2::new(-20., -10.),
        Vec2::new(10., 10.),
        Vec2::new(20., -10.),
    );
    let plane_color = Color::srgb(0.2, 1.0, 0.2);
    let plane_position = Transform::from_xyz(20.0, 0.0, 0.0);

    commands.spawn((
        Mesh2d(meshes.add(plane_shape)),
        MeshMaterial2d(materials.add(plane_color)),
        plane_position,
    ));
}

fn rotate(time: Res<Time>, mut meshes: Query<&mut Transform, With<Mesh2d>>) {
    for mut mesh in &mut meshes {
        mesh.translation.x += 150. * time.delta_secs();
        mesh.rotate_z(0.02);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}
