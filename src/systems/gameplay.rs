use crate::effects::{ActiveEffects, VisualEffects, EntityEffects};
use crate::game::{Game, GameState};
use bevy::prelude::*;
use crate::world::Velocity;
use rand::Rng;
use crate::enemies::Enemy;

pub fn apply_effects(
    game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EntityEffects, &mut Velocity, &mut Transform)>,
) {
    if game.state != GameState::Running {
        return;
    }

    for (entity, mut effects, mut velocity, mut transform) in query.iter_mut() {
        for effect in &mut effects.active {
            effect.tick(time.delta());
            if !effect.is_expired() {
                effect.effect().apply(entity, &mut velocity, &mut transform);
            } else {
                effect.effect().undo(entity, &mut velocity, &mut transform);
            }
        }
    }
}

pub fn cleanup_effects(
    game: Res<Game>,
    time: Res<Time>,
    mut query: Query<(&mut ActiveEffects, &mut VisualEffects)>,
) {
    if game.state != GameState::Running {
        return;
    }

    for (mut effects, mut visual_effects) in query.iter_mut() {
        for effect in &mut effects.effects {
            effect.consume_time(time.delta_seconds());
        }
        effects.effects.retain(|effect| effect.is_active());
        visual_effects.effects.retain(|effect| !effect.is_expired());
    }
}

pub fn random_enemy_jump(
    game: Res<Game>,
    mut query: Query<&mut Velocity, With<Enemy>>
) {
    if game.state != GameState::Running {
        return;
    }

    let mut rng = rand::thread_rng();

    // TODO: Make it smarter.
    for mut velocity in query.iter_mut() {
        let mut v = velocity.current();
        if v.y == 0.0 && rng.gen_bool(0.01) {
            velocity.set_vertical(crate::player::VELOCITY_ON_JUMP * 1.25);
            break;
        }
    }
}
