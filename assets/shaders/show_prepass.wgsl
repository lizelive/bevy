#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
    pbr_deferred_types::unpack_unorm4x8_,
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

        // let depth = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
        let normal = bevy_pbr::prepass_utils::prepass_normal(mesh.position, sample_index);
        let motion_vector = bevy_pbr::prepass_utils::prepass_motion_vector(mesh.position, sample_index);
        let pbr_input = bevy_pbr::pbr_deferred_functions::prepass_pbr_input(mesh.position);
        let material = pbr_input.material;
        let depth = pbr_input.frag_coord.z;
        
        switch settings.prepass_view {
            case 0u: { return vec4(0.0); }
            case 1u: { return vec4(depth, depth, depth, 1.0); }
            case 2u: { return vec4(normal, 1.0); }
            case 3u: { return vec4(motion_vector / globals.delta_time, 0.0, 1.0); }
            case 4u: { return vec4(material.metallic, material.perceptual_roughness, material.reflectance, 1.0); }
            case 5u: { return material.base_color; }
            case 6u: { return material.emissive; }
            case 7u: { return vec4(pbr_input.occlusion, 1.0); }
            case 8u: { return vec4(pbr_input.frag_coord.xyz, 1.0); }
            case 9u: {
                return vec4(normalize(pbr_input.world_position.xyz), 1.0); }
            case 10u: { return vec4(material.diffuse_transmission, material.specular_transmission, material.thickness, 1.0); }

            case 11u: { return vec4(material.ior, material.attenuation_distance, material.thickness, 1.0); }

            case 12u: { return material.attenuation_color; }

            case 13u: { return vec4(unpack_unorm4x8_(pbr_input.flags).xyz, 1.0); }
            case 14u: { return vec4(unpack_unorm4x8_(pbr_input.flags).www, 1.0); }

            case 15u: { return vec4(pbr_input.N, 1.0); }

            case 16u: { return vec4(normal, 1.0); }

            case 17u: {
                let dn = normal;
                let gn = pbr_input.N;
                let ve  =dot(dn, gn) * 0.5 + 0.5;
                let dv = 1.0 - (ve * vec3(10.0, 100.0, 1000.0))%1.0; 
                return vec4(dv, 1.0);
            }

            case 18u: {
                let dl = -bevy_pbr::view_transformations::depth_ndc_to_view_z(depth);
                return vec4(dl, (dl) - 6.0, dl % 1.0, 1.0);
            }

            case 19u: {
                let dl = distance(pbr_input.world_position.xyz, bevy_pbr::mesh_view_bindings::view.world_position);
                let dc = length(pbr_input.world_position.xyz);
                return vec4(dc % 1.0, dl % 1.0, 0.0, 1.0);
            }
 
            case 20u: {
                let dc = length(pbr_input.world_position.xyz) * vec3(1.0, 0.1, 10.0);
                return vec4(dc % 1.0, 1.0);
            }

            default: { return vec4(1.0, 0.0, 1.0, 1.0); }
        }


}
