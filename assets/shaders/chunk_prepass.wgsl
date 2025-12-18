#import bevy_pbr::mesh_functions::{mesh_position_local_to_world, get_world_from_local, mesh_normal_local_to_world};
#import bevy_pbr::prepass_io::FragmentOutput;
#import bevy_pbr::view_transformations::position_world_to_clip;
#import "shaders/chunk_util.wgsl"::{Vertex, unpack}

#ifdef DEFERRED_PREPASS
#import bevy_pbr::rgb9e5
#endif

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let data = unpack(vertex.data);
    var out: VertexOutput;

    let world_to_local = get_world_from_local(vertex.instance_index);
    let world_position = mesh_position_local_to_world(
        world_to_local,
        data.position
    );
    out.world_normal = mesh_normal_local_to_world(data.normal, vertex.instance_index);
    out.position = position_world_to_clip(world_position.xyz);

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