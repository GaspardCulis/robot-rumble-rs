#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

struct OrbitMaterial {
    time: f32,
    base_color: vec4<f32>, // Tu peux l'utiliser pour un effet de teinte
    saturation: f32,
    alpha: f32,
}

@group(0) @binding(0)
var<uniform> orbit: OrbitMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    //TODO: corriger un problème sur le passage de la variable de temps (actuellement, elle est toujours à 0)
    let centered_uv = in.uv - vec2(0.5, 0.5);

    let angle = atan2(centered_uv.y, centered_uv.x);  
    let hue = (angle + orbit.time * 5.0) / (2.0 * 3.14159); 
    let wrapped_hue = fract(hue + orbit.time * 0.1);  

    let hsv = vec3(wrapped_hue, 1.0, 1.0);
    let rgb = hsv2rgb(hsv);

    let final_color = mix(rgb, orbit.base_color.rgb, 0.2); 

    return vec4(final_color, orbit.alpha);
}

// Utilitaire HSV -> RGB
fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let c = hsv.z * hsv.y;
    let h6 = hsv.x * 6.0;
    let f = h6 - 2.0 * floor(h6 / 2.0);  // Calcul du facteur
    let x = c * (1.0 - abs(f - 1.0));
    let m = hsv.z - c;

    var rgb: vec3<f32>;

    if (hsv.x < 1.0 / 6.0) {
        rgb = vec3(c, x, 0.0);
    } else if (hsv.x < 2.0 / 6.0) {
        rgb = vec3(x, c, 0.0);
    } else if (hsv.x < 3.0 / 6.0) {
        rgb = vec3(0.0, c, x);
    } else if (hsv.x < 4.0 / 6.0) {
        rgb = vec3(0.0, x, c);
    } else if (hsv.x < 5.0 / 6.0) {
        rgb = vec3(x, 0.0, c);
    } else {
        rgb = vec3(c, 0.0, x);
    }

    return rgb + vec3(m);
}

