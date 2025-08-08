use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {

    let joint1 = RevoluteJointBuilder::new().local_anchor1(Vec2::new(200.0, 0.0));
    let joint2 = RevoluteJointBuilder::new().local_anchor1(Vec2::new(300.0, 0.0));
    
    let entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))

        .insert(Transform::from_xyz(-500.0, 200.0, 0.0))
        .id();

    let entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Transform::from_xyz(-300., 200.0, 0.0))
        .insert(ImpulseJoint::new(entity, joint1))
        .id();
    

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(5.0))
        .insert(Transform::from_xyz(0.0, 200.0, 0.0))
        .insert(ImpulseJoint::new(entity, joint2));
}