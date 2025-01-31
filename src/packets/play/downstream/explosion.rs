use minecraft_net_proc::{Field, Packet};
use crate::fields::general::IdOr;
use crate::fields::particles::Particle;
use crate::fields::slot::SoundEvent;

Packet!(Explosion, 0x21, {
    x: Double,
    y: Double,
    z: Double,
    player_velocity: PrefixedOptional<PlayerVelocity>,
    explosion_particle: Particle,
    explosion_sound: IdOr<SoundEvent>
});
Field!(PlayerVelocity, {
    x: Double,
    y: Float,
    z: Float,
});