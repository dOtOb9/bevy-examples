use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins({
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin::default(),
    })
    .add_systems(Startup, setup);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Comands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Circle::new(50.)),
        meshes.add(CircularSector::new(50.0, 1.0));
    ]
}