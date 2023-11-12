#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
}

struct ShowPrepassSettings {
    show_depth: u32,
    show_normals: u32,
    show_motion_vectors: u32,
    show_deferred_data: u32,
    prepass_view: u32,
}
@group(1) @binding(0) var<uniform> settings: ShowPrepassSettings;

@fragment
fn fragment(
#ifdef MULTISAMPLED
    @builtin(sample_index) sample_index: u32,
#endif
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
#ifndef MULTISAMPLED
    let sample_index = 0u;
#endif
    if settings.show_depth == 1u {
        let depth = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
        return vec4(depth, depth, depth, 1.0);
    } else if settings.show_normals == 1u {
        let normal = bevy_pbr::prepass_utils::prepass_normal(mesh.position, sample_index);
        return vec4(normal, 1.0);
    } else if settings.show_motion_vectors == 1u {
        let motion_vector = bevy_pbr::prepass_utils::prepass_motion_vector(mesh.position, sample_index);
        return vec4(motion_vector / globals.delta_time, 0.0, 1.0);
    } else if settings.show_deferred_data == 1u {
        let pbr_input = bevy_pbr::pbr_deferred_functions::prepass_pbr_input(mesh.position);
        let material = pbr_input.material;
        //return material.base_color;
        return vec4(pbr_input.occlusion, 1.0);
        // return vec4(material.metallic, material.perceptual_roughness, material.reflectance, 1.0);
    }

    return vec4(0.0);
}
