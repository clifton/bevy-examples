#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

fn dir_to_equirectangular(dir: vec3<f32>) -> vec2<f32> {
    var PI = 3.1415926;
    let x = atan2(dir.z, dir.x) / (2.0 * PI) + 0.5; // 0-1
    let y = acos(dir.y) / PI; // 0-1
    return vec2<f32>(x, y);
}

fn refract(I: vec3<f32>, N: vec3<f32>, eta: f32) -> vec3<f32> {
    let k = max((1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I))), 0.0);
    return eta * I - (eta * dot(N, I) + sqrt(k)) * N;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    let NdotV = max(dot(N, V), 0.0001);
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);
    fresnel = pow(fresnel, 5.0) * 2.0;

    let glow = pow(NdotV, 10.0) * 50.0;

    var color = vec3(0.0, 0.0, 0.0);
    color = mix(color, vec3(0.5, 0.1, 0.0), glow);

    let bump_coords = dir_to_equirectangular(N * vec3(1.0, -0.5, 1.0) - vec3(0.0, 0.5, 0.0));
    let bump = textureSample(texture, texture_sampler, bump_coords).r;

    var reflect_coords = dir_to_equirectangular(reflect(-V, N));
    let reflection = textureSample(texture, texture_sampler, reflect_coords).rgb;

    var refract_coords = dir_to_equirectangular(refract(-V, N + bump * 2.0, 1.0 / 1.52));
    let refraction = textureSample(texture, texture_sampler, refract_coords).rgb;

    color = (color * refraction) + reflection * (fresnel + 0.05);

    return tone_mapping(vec4(color, 1.0));
}