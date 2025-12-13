#import bevy_pbr::{
    mesh_functions::{mesh_position_local_to_clip, get_world_from_local, mesh_normal_local_to_world},
    prepass_io::{FragmentOutput},
}
#import "shaders/chunk_util.wgsl"::{UnpackedData, Vertex, unpack, normals}

#ifdef DEFERRED_PREPASS
#import bevy_pbr::rgb9e5
#endif

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let data = unpack(vertex.data);
    let normal = normals[data.direction];

    out.world_normal = mesh_normal_local_to_world(normal, vertex.instance_index);

    var model = get_world_from_local(vertex.instance_index);
    out.position = mesh_position_local_to_clip(model, data.position);

    return out;
}

#ifdef PREPASS_FRAGMENT
@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.frag_depth = in.position.z;
#ifdef NORMAL_PREPASS
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
#endif

#ifdef DEFERRED_PREPASS
    // There isn't any material info available for this default prepass shader so we are just writing 
    // emissive magenta out to the deferred gbuffer to be rendered by the first deferred lighting pass layer.
    // This is here so if the default prepass fragment is used for deferred magenta will be rendered, and also
    // as an example to show that a user could write to the deferred gbuffer if they were to start from this shader.
    out.deferred = vec4(0u, bevy_pbr::rgb9e5::vec3_to_rgb9e5_(vec3(1.0, 0.0, 1.0)), 0u, 0u);
    out.deferred_lighting_pass_id = 1u;
#endif

    return out;
}
#endif // PREPASS_FRAGMENT