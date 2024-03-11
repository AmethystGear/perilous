use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::random;

use crate::chain::{simulate_chain, Chain, ChainLink};

#[derive(Component)]
pub struct SnapLink((usize, usize));

struct Tip {
    target: Vec2,
    velocity: Vec2,
}

#[derive(Component)]
pub struct AnchorBoy {
    chains: Vec<(Chain, Tip)>,
    room_bounds: (Vec2, Vec2),
    max_chain_speed: f32,
    chain_accel: f32,
    max_speed: f32,
    accel: f32,
    health: f32,
    active: bool,
}

struct ChainSettings {
    len: usize,
    start_dist: f32,
    start_angle: f32,
    chain_len: f32,
    chain_radius: f32,
    anchor_len: f32,
    anchor_radius: f32,
}

fn generate_chain(settings: &ChainSettings) -> Vec<ChainLink> {
    let mut chain = vec![];
    let mut angle = settings.start_angle;
    let mut dist = settings.start_dist;
    for i in 0..settings.len {
        let pt = Vec2::new(angle.cos(), angle.sin()) * dist;
        let (len, radius) = if i == settings.len - 1 {
            (settings.anchor_len, settings.anchor_radius)
        } else {
            (settings.chain_len, settings.chain_radius)
        };
        let chain_dir = angle + PI / 2.0 + (random::<f32>() - 0.5);
        let chain_dir = Vec2::new(chain_dir.cos(), chain_dir.sin()) * len;
        let next_chain_pt = pt + chain_dir;

        let link = ChainLink {
            loc: pt,
            prev_loc: pt,
            len,
            radius,
            constrain: true,
        };
        chain.push(link);
        dist = next_chain_pt.length();
        angle = next_chain_pt.to_angle();
    }
    chain
}

pub fn setup_anchor_boy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
) {
    let mut chain_settings = ChainSettings {
        len: 39,
        start_dist: 30.0,
        start_angle: 0.0,
        chain_len: 10.0,
        chain_radius: 5.0,
        anchor_len: 90.0,
        anchor_radius: 40.0,
    };
    let chain_left = generate_chain(&chain_settings);
    chain_settings.start_angle = PI;
    let chain_right = generate_chain(&chain_settings);

    let anchor_boy = AnchorBoy {
        chains: vec![
            (
                Chain(chain_left),
                Tip {
                    target: Vec2::ZERO,
                    velocity: Vec2::ZERO,
                },
            ),
            (
                Chain(chain_right),
                Tip {
                    target: Vec2::ZERO,
                    velocity: Vec2::ZERO,
                },
            ),
        ],
        room_bounds: (Vec2::ZERO, Vec2::ZERO),
        max_chain_speed: 200.0,
        chain_accel: 100.0,
        max_speed: 10.0,
        accel: 3.0,
        health: 100.0,
        active: false,
    };

    for i in 0..anchor_boy.chains.len() {
        for j in 1..anchor_boy.chains[i].0 .0.len() {
            let res_name = if j == anchor_boy.chains[i].0 .0.len() - 1 {
                "anchor.png"
            } else if j % 2 == 0 {
                "chain_0.png"
            } else {
                "chain_1.png"
            };

            let img = server.load(res_name);
            commands.spawn((
                SpriteBundle {
                    texture: img,
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                SnapLink((i, j)),
            ));
        }
    }

    let circle = Mesh2dHandle(meshes.add(Circle { radius: 25.0 }));
    let handles = commands
        .spawn((
            SpatialBundle {
                ..Default::default()
            },
            AnchorSpin {
                angle: 0.0,
            },
        ))
        .with_children(|parent| {
            let handle = server.load("handle.png");
            parent.spawn((
                SpriteBundle {
                    texture: handle.clone(),
                    sprite: Sprite {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    transform: Transform::from_scale(Vec3::new(0.05, 0.05, 0.0))
                        .with_translation(Vec3::new(-23.0, 0.0, 1.0))
                        .with_rotation(Quat::from_rotation_z(PI/2.)),
                    ..Default::default()
                },
            ));
            parent.spawn((SpriteBundle {
                texture: handle.clone(),
                sprite: Sprite {
                    color: Color::BLACK,
                    ..Default::default()
                },
                transform: Transform::from_scale(Vec3::new(0.05, 0.05, 0.0))
                    .with_translation(Vec3::new(23.0, 0.0, 1.0))
                    .with_rotation(Quat::from_rotation_z(-PI/2.)),
                ..Default::default()
            },));
        })
        .id();
    commands
        .spawn((
            anchor_boy,
            MaterialMesh2dBundle {
                mesh: circle,
                material: materials.add(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            let anchor = server.load("anchor.png");
            parent.spawn((SpriteBundle {
                texture: anchor,
                transform: Transform::from_scale(Vec3::new(0.075, 0.075, 0.0))
                    .with_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            },));
        })
        .push_children(&[handles]);
}

#[derive(Component)]
pub struct AnchorSpin {
    pub angle: f32,
}

pub fn set_angle_according_to_spin(mut spin: Query<(&AnchorSpin, &mut Transform)>) {
    let (&AnchorSpin { angle }, mut transform) = spin.single_mut();
    transform.rotation = Quat::from_rotation_z(angle);
}

pub fn anchor_boy(
    mut anchor_boy: Query<(&mut AnchorBoy, &mut Transform)>,
    mut spin: Query<&mut AnchorSpin, Without<AnchorBoy>>,
) {
    let (mut anchor_boy, mut anchor_boy_transform) = anchor_boy.single_mut();
    for (chain, tip) in &mut anchor_boy.chains {
        simulate_chain(
            chain,
            (Vec2::new(-500.0, -500.0), Vec2::new(500.0, 500.0)),
            10,
        );
    }
    let mut spin = spin.single_mut();
    spin.angle += 0.1;
    let pos = Vec2::new(spin.angle.cos(), spin.angle.sin()) * 30.0;
    anchor_boy.chains[0].0.0[0].loc = pos + anchor_boy_transform.translation.xy();
    let pos = Vec2::new((spin.angle + PI).cos(), (spin.angle + PI).sin()) * 30.0;
    anchor_boy.chains[1].0.0[0].loc = pos + anchor_boy_transform.translation.xy();

    anchor_boy_transform.translation += Vec3::new(1.0, 0.0, 0.0);
}

pub fn snap_links_to_chains(
    anchor_boy: Query<(&AnchorBoy, &Transform)>,
    mut links_to_snap: Query<(&SnapLink, &mut Transform), Without<AnchorBoy>>,
) {
    let (anchor_boy, anchor_boy_transform) = anchor_boy.single();
    for (&SnapLink((chain_id, link_id)), mut link_transform) in links_to_snap.iter_mut() {
        let chain_loc =
            anchor_boy.chains[chain_id].0 .0[link_id].loc;
        let prev_chain_loc = anchor_boy.chains[chain_id].0 .0[link_id - 1].loc;
        let angle = (chain_loc - prev_chain_loc).to_angle();
        let loc = (chain_loc + prev_chain_loc) / 2.0;
        let scale_factor = anchor_boy.chains[chain_id].0 .0[link_id].len;
        if link_id == anchor_boy.chains[chain_id].0 .0.len() - 1 {
            link_transform.scale = Vec3::new(scale_factor / 512.0, scale_factor / 512.0, 1.0);
        } else {
            link_transform.scale =
                Vec3::new(scale_factor * 1.5 / 512.0, scale_factor * 1.5 / 512.0, 1.0);
        }
        link_transform.translation = Vec3::new(loc.x, loc.y, 0.0);
        link_transform.rotation = Quat::from_rotation_z(angle + PI / 2.0);
    }
}

pub fn set_link_properties(mut links: Query<(&mut Visibility, &mut Sprite), With<SnapLink>>) {
    for (mut visibility, mut sprite) in links.iter_mut() {
        *visibility = Visibility::Visible;
        sprite.color.set_r(0.0);
        sprite.color.set_g(0.0);
        sprite.color.set_b(0.0);
    }
}
