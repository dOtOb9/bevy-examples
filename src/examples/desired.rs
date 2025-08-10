use bevy::{picking::window, prelude::*, winit::cursor};
use bevy_rapier2d::{na::{Vector, Vector2}, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, apply_force)
        
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(30.),
        GravityScale(0.0), 
        ColliderMassProperties::Density(1.),
        Transform::from_xyz(100.0, 100.0, 0.0),
        Velocity::default(),
    ));
}

fn apply_force(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &mut Velocity), With<RigidBody>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single().unwrap();

    let cursor_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));
    let width = window.width();
    let height = window.height();

    let max_speed = 7000.;
    let max_force = 50000.;


    for (entity, transform, velocity) in query.iter() {
        let desired = Vec2::new(cursor_position.x - width/2., height/2.-cursor_position.y) - transform.translation.truncate();

        let desired = desired.normalize_or_zero() * max_speed;

        let impulse = (desired - velocity.linvel).clamp_length_max(max_force);

        println!("Impulse: {:?}", impulse);

        commands.entity(entity).remove::<ExternalImpulse>();
        commands.entity(entity).insert(ExternalImpulse {
            impulse: impulse,
            torque_impulse: 0.0,
        });
    }
}