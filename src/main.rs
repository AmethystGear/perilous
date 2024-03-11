use anchorboy::{anchor_boy, set_angle_according_to_spin, set_link_properties, setup_anchor_boy, snap_links_to_chains};
use bevy::prelude::*;
use droplet::{move_droplet, setup_droplet};

mod anchorboy;
mod chain;
mod droplet;
mod marching_squares;
mod mesh;
mod point;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.75, 0.7, 0.75)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_droplet, setup_anchor_boy))
        .add_systems(
            FixedUpdate,
            (
                move_droplet,
                snap_links_to_chains,
                anchor_boy,
                set_link_properties,
                set_angle_according_to_spin,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
