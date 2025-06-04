use super::planet::{Radius, materials::CommonMaterial};
use super::projectile::config::BLACKHOLE_RADIUS;
use super::projectile::{DecayTimer, ProjectileDecayedEvent};
use crate::core::{
    gravity::{Mass, Static},
    physics::PhysicsSet,
};
use crate::entities::projectile::Projectile;
use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};

mod visuals;
use visuals::*;

// TODO: move to config
const BLACKHOLE_MASS: u32 = 100000;
const BH_DECAY_TIME: f32 = 10.;

#[derive(Component, Debug, Reflect, Clone, Copy, PartialEq)]
#[require(Visibility)]
pub struct BlackHole;

pub struct BlackHolePlugin;
impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components::<BlackHole, Static>()
            .register_required_components_with::<BlackHole, Name>(|| Name::new("Blackhole"))
            .register_required_components_with::<BlackHole, Mass>(|| Mass(BLACKHOLE_MASS))
            .register_required_components_with::<BlackHole, Radius>(|| Radius(BLACKHOLE_RADIUS))
            .add_plugins(Material2dPlugin::<BlackHoleMaterial>::default())
            .add_plugins(Material2dPlugin::<BlackHoleRingMaterial>::default())
            .add_systems(Update, add_visuals)
            .add_systems(
                GgrsSchedule,
                handle_blackhole_projectile_decay
                    // Needs to run after `projectile::tick_decay_timers`
                    .after(PhysicsSet::Player)
                    .before(PhysicsSet::Gravity),
            );
    }
}

fn handle_blackhole_projectile_decay(
    mut commands: Commands,
    mut events: EventReader<ProjectileDecayedEvent>,
) {
    for event in events.read() {
        if let Some(r#type) = event.r#type {
            if let Projectile::Blackhole = r#type {
                commands
                    .spawn((
                        BlackHole,
                        event.position.clone(),
                        DecayTimer(Timer::from_seconds(BH_DECAY_TIME, TimerMode::Once)),
                    ))
                    .add_rollback();
            }
        }
    }
}

fn add_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut core_materials: ResMut<Assets<BlackHoleMaterial>>,
    mut ring_materials: ResMut<Assets<BlackHoleRingMaterial>>,
    query: Query<(Entity, &Radius), (With<BlackHole>, Without<Children>)>,
) {
    for (black_hole, bh_radius) in query.iter() {
        let mut common = CommonMaterial {
            pixels: f32::min(bh_radius.0 as f32 / 2., 200.),
            seed: 69.,
            ..Default::default()
        };

        // Material spesific
        common.size = CORE_SIZE;
        common.octaves = CORE_OCTAVES;

        let core = core_materials.add(BlackHoleMaterial {
            common,
            radius: CORE_RADIUS,
            light_width: CORE_LIGHT_WIDTH,
            color_core: Srgba::hex(CORE_PALETTE[0]).unwrap().into(),
            color_inner: Srgba::hex(CORE_PALETTE[1]).unwrap().into(),
            color_outer: Srgba::hex(CORE_PALETTE[2]).unwrap().into(),
            _wasm_padding: Vec2::ZERO,
        });

        common = CommonMaterial {
            pixels: f32::min(bh_radius.0 as f32 / 2., 200.),
            seed: 69.,
            ..Default::default()
        }
        .scale(CORE_SCALE);

        common.rotation = RING_ROTATION;
        common.size = RING_SIZE;
        common.octaves = RING_OCTAVES;

        let ring = ring_materials.add(BlackHoleRingMaterial {
            common,
            disk_width: RING_DISK_WIDTH,
            ring_perspective: RING_DISK_PERSPECTIVE,
            should_dither: true as u32,
            n_colors: 5,
            colors: [
                Srgba::hex(RING_PALETTE[0]).unwrap().into(),
                Srgba::hex(RING_PALETTE[1]).unwrap().into(),
                Srgba::hex(RING_PALETTE[2]).unwrap().into(),
                Srgba::hex(RING_PALETTE[3]).unwrap().into(),
                Srgba::hex(RING_PALETTE[4]).unwrap().into(),
            ],
            _wasm_padding: Vec2::ZERO,
        });

        let core_entity = commands
            .spawn((
                Name::new("BH_CoreMesh"),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(core),
                Transform::from_scale(Vec3::splat(bh_radius.0 as f32 * 2.0)).with_translation(
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 1.,
                    },
                ),
            ))
            .id();
        let ring_entity = commands
            .spawn((
                Name::new("BH_RingMesh"),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(ring),
                Transform::from_scale(Vec3::splat((bh_radius.0 as f32 * 2.0) * CORE_SCALE))
                    .with_translation(Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 2.,
                    }),
            ))
            .id();
        commands.entity(black_hole).add_children(&[core_entity]);
        commands.entity(black_hole).add_children(&[ring_entity]);
    }
}
