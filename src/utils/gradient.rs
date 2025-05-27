use bevy::{
    color::{ColorToComponents, Srgba},
    image::Image,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

pub fn gradient(offsets: &Vec<f32>, colors: &Vec<Srgba>) -> Image {
    const TEXTURE_SIZE: usize = 64;
    const TEXTURE_COLOR_CHANNELS: usize = 4;

    assert_eq!(
        offsets.len(),
        colors.len(),
        "Offsets and colors must have the same length"
    );
    assert!(!offsets.is_empty(), "Offsets and colors must not be empty");

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * TEXTURE_COLOR_CHANNELS];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / (TEXTURE_SIZE - 1) as f32;

            let color = interpolate_colors(&offsets, &colors, u);

            let index = (y * TEXTURE_SIZE + x) * TEXTURE_COLOR_CHANNELS;
            texture_data[index] = (color.red * 255.0) as u8;
            texture_data[index + 1] = (color.green * 255.0) as u8;
            texture_data[index + 2] = (color.blue * 255.0) as u8;
            texture_data[index + 3] = (color.alpha * 255.0) as u8;
        }
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn interpolate_colors(offsets: &[f32], colors: &[Srgba], u: f32) -> Srgba {
    if offsets.len() == 1 {
        return colors[0];
    }

    for i in 0..offsets.len() - 1 {
        if u >= offsets[i] && u <= offsets[i + 1] {
            let color_a = colors.get(i).unwrap();
            let color_b = colors.get(i + 1).unwrap();

            let t = (u - offsets[i]) / (offsets[i + 1] - offsets[i]);
            return Srgba::from_vec4(color_a.to_vec4().lerp(color_b.to_vec4(), t));
        }
    }

    colors[colors.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::color::Srgba;

    #[test]
    fn test_single_color_gradient() {
        let offsets = vec![0.0];
        let colors = vec![Srgba::new(1.0, 0.0, 0.0, 1.0)]; // Red

        let image = gradient(&offsets, &colors);

        let data = image.data.expect("Should not be empty");

        for i in (0..data.len()).step_by(4) {
            assert_eq!(data[i], 255); // Red
            assert_eq!(data[i + 1], 0); // Green
            assert_eq!(data[i + 2], 0); // Blue
            assert_eq!(data[i + 3], 255); // Alpha
        }
    }

    #[test]
    fn test_two_color_gradient() {
        let offsets = vec![0.0, 1.0];
        let colors = vec![
            Srgba::new(1.0, 0.0, 0.0, 1.0),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        ];
        let image = gradient(&offsets, &colors);

        let data = image.data.expect("Should not be empty");
        for x in 0..64 {
            let u = x as f32 / 63.0;
            let color = interpolate_colors(&offsets, &colors, u);

            let index = x * 4;
            assert_eq!(data[index], (color.red * 255.0) as u8);
            assert_eq!(data[index + 1], (color.green * 255.0) as u8);
            assert_eq!(data[index + 2], (color.blue * 255.0) as u8);
            assert_eq!(data[index + 3], (color.alpha * 255.0) as u8);
        }
    }

    #[test]
    fn test_multiple_color_gradient() {
        let offsets = vec![0.0, 0.5, 1.0];
        let colors = vec![
            Srgba::new(1.0, 0.0, 0.0, 1.0),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
            Srgba::new(0.0, 0.0, 1.0, 1.0),
        ];
        let image = gradient(&offsets, &colors);

        let data = image.data.expect("Should not be empty");
        for x in 0..64 {
            let u = x as f32 / 63.0;
            let color = interpolate_colors(&offsets, &colors, u);

            let index = x * 4;
            assert_eq!(data[index], (color.red * 255.0) as u8);
            assert_eq!(data[index + 1], (color.green * 255.0) as u8);
            assert_eq!(data[index + 2], (color.blue * 255.0) as u8);
            assert_eq!(data[index + 3], (color.alpha * 255.0) as u8);
        }
    }

    #[test]
    #[should_panic(expected = "Offsets and colors must have the same length")]
    fn test_mismatched_offsets_and_colors() {
        let offsets = vec![0.0, 1.0];
        let colors = vec![Srgba::new(1.0, 0.0, 0.0, 1.0)];
        gradient(&offsets, &colors);
    }

    #[test]
    #[should_panic(expected = "Offsets and colors must not be empty")]
    fn test_empty_offsets_and_colors() {
        let offsets = vec![];
        let colors = vec![];
        gradient(&offsets, &colors);
    }
}
