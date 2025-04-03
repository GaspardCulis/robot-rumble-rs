use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{network::SessionSeed, utils::gradient, GameState};

#[derive(Component)]
struct Background;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<NebulaeMaterial>::default())
            .add_plugins(Material2dPlugin::<StarsMaterial>::default())
            .add_systems(OnEnter(GameState::WorldGen), setup)
            .add_systems(Update, scale_and_center);
    }
}

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
struct StarsMaterial {
    #[uniform(0)]
    pub size: f32,
    #[uniform(0)]
    pub octaves: i32,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub pixels: f32,
    #[uniform(0)]
    pub uv_correct: Vec2,
    #[uniform(0)]
    _wasm_padding: Vec2,
    #[texture(1)]
    #[sampler(2)]
    colorscheme_texture: Option<Handle<Image>>,
}

impl Material2d for StarsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background/stars.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
struct NebulaeMaterial {
    #[uniform(0)]
    pub size: f32,
    #[uniform(0)]
    pub octaves: i32,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub pixels: f32,
    #[uniform(0)]
    pub uv_correct: Vec2,
    #[uniform(0)]
    pub background_color: LinearRgba,
    #[uniform(0)]
    _wasm_padding: Vec2,
    #[texture(1)]
    #[sampler(2)]
    colorscheme_texture: Option<Handle<Image>>,
}

impl Material2d for NebulaeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background/nebulae.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut star_materials: ResMut<Assets<StarsMaterial>>,
    mut nebulae_materials: ResMut<Assets<NebulaeMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<SessionSeed>,
) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.0);

    let background_color = Srgba::hex("#171711").unwrap();
    let colorscheme = images.add(gradient(
        &vec![0.0, 0.143, 0.286, 0.429, 0.571, 0.714, 0.857, 1.0],
        &vec![
            "#202215", "#3a2802", "#963c3c", "#ca5a2e", "#ff7831", "#f39949", "#ebc275", "#dfd785",
        ]
        .into_iter()
        .map(|hex| Srgba::hex(hex).unwrap())
        .collect(),
    ));

    let nebulae = nebulae_materials.add(NebulaeMaterial {
        size: 8.0,
        octaves: 8,
        seed: rng.random(),
        pixels: 500.0,
        uv_correct: Vec2::ONE,
        background_color: Srgba::hex("#001711").unwrap().into(),
        colorscheme_texture: Some(colorscheme.clone_weak()),
        _wasm_padding: Vec2::default(),
    });

    let stars = star_materials.add(StarsMaterial {
        size: 10.0,
        octaves: 8,
        seed: rng.random(),
        pixels: 500.0,
        uv_correct: Vec2::ONE,
        colorscheme_texture: Some(colorscheme),
        _wasm_padding: Vec2::default(),
    });

    let background = color_materials.add(Color::from(background_color));

    commands.spawn((
        Background,
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(nebulae),
        Transform::from_scale(Vec3::splat(500.0)).with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: -10.,
        }),
    ));

    commands.spawn((
        Background,
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(stars),
        Transform::from_scale(Vec3::splat(500.0)).with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: -10.1,
        }),
    ));

    commands.spawn((
        Background,
        Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
        MeshMaterial2d(background),
        Transform::from_scale(Vec3::splat(500.0)).with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: -10.2,
        }),
    ));
}

fn scale_and_center(
    mut query: Query<&mut Transform, (With<Background>, Without<Camera2d>)>,
    window: Query<&Window>,
    camera: Query<&Transform, With<Camera2d>>,
) {
    if window.is_empty() || camera.is_empty() {
        return;
    }

    let window = window.single();
    let camera = camera.single();
    for mut bg_transform in query.iter_mut() {
        let scale = if window.width() > window.height() {
            window.width()
        } else {
            window.height()
        };

        bg_transform.translation.x = camera.translation.x;
        bg_transform.translation.y = camera.translation.y;
        bg_transform.scale = Vec3::splat(scale) * camera.scale;
    }
}
