use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)] //mark player as component using this macro
pub struct Player; // consider the data I might want to it to hold


// Should I try and use OOP here for the component?
// Have a player initialise function that populates the struct?
// Or is that not very rust?
pub fn create_player(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 0.5, 0.0));
}