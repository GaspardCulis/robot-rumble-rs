use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::utils::gradient;

#[derive(Component)]
struct Background;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StarsMaterial>::default())
            .add_systems(Startup, setup)
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
    #[texture(1)]
    #[sampler(2)]
    colorscheme_texture: Option<Handle<Image>>,
}

impl Material2d for StarsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background/stars.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarsMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let colorscheme = images.add(gradient(
        &vec![0.0, 0.143, 0.286, 0.429, 0.571, 0.714, 0.857, 1.0],
        &vec![
            "#202215", "#3a2802", "#963c3c", "#ca5a2e", "#ff7831", "#f39949", "#ebc275", "#dfd785",
        ]
        .into_iter()
        .map(|hex| Srgba::hex(hex).unwrap())
        .collect(),
    ));

    let material = materials.add(StarsMaterial {
        size: 10.0,
        octaves: 8,
        seed: 69.42,
        pixels: 500.0,
        uv_correct: Vec2::ONE,
        colorscheme_texture: Some(colorscheme),
    });

    commands.spawn((
        Background,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
            transform: Transform::from_scale(Vec3::splat(500.0)).with_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: -10.,
            }),
            material,
            ..default()
        },
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
        bg_transform.scale = Vec3::splat(scale);
    }
}
