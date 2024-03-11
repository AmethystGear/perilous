use bevy::math::Vec2;
use itertools::Itertools;

pub struct ChainLink {
    pub loc: Vec2,
    pub prev_loc: Vec2,
    pub len: f32,
    pub radius: f32,
    pub constrain: bool,
}

pub struct Chain(pub Vec<ChainLink>);

/// simulates a chain, there are two assumptions here:
/// 1. the chain is in a rectangular axis aligned room
/// 2. there are no other things the chain needs to collide with
pub fn simulate_chain(
    chain: &mut Chain,
    room_bounds: (Vec2, Vec2),
    chain_constraint_iterations: usize,
) {
    // update chain according to velocities
    for link in chain.0.iter_mut().skip(1) {
        let temp = link.loc;
        link.loc += link.loc - link.prev_loc;
        link.prev_loc = temp;
    }

    // apply chain constraints
    for _ in 0..chain_constraint_iterations {
        let mut start = true;
        for i in 0..chain.0.len()-1 {
            let dist = chain.0[i].loc.distance(chain.0[i + 1].loc);
            let difference = if dist > 0.0 {
                (chain.0[i + 1].len - dist) / dist
            } else {
                0.0
            };
            let translation = (chain.0[i].loc - chain.0[i + 1].loc) * difference;
            if start {
                chain.0[i + 1].loc -= translation;
            } else {
                chain.0[i].loc += translation * 0.5;
                chain.0[i + 1].loc -= translation * 0.5;
            }
            start = false;
        }
    }

    // apply collision constraints to keep chain within room bounds
    for link in chain.0.iter_mut() {
        if !link.constrain {
            continue;
        }
        let rad_vec = Vec2::new(link.radius, link.radius);
        let min_bound = room_bounds.0 + rad_vec;
        let max_bound = room_bounds.1 - rad_vec;
        link.loc.x = link.loc.x.clamp(min_bound.x, max_bound.x);
        link.loc.y = link.loc.y.clamp(min_bound.y, max_bound.y);
    }
}
