#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_world, mesh_normal_local_to_world};
#import bevy_pbr::view_transformations::position_world_to_clip;
#import "shaders/chunk_util.wgsl"::{unpack, Vertex};

#import bevy_pbr::pbr_functions::{calculate_view, prepare_world_normal};
#import bevy_pbr::mesh_bindings::mesh;
#import bevy_pbr::pbr_types::pbr_input_new;
#import bevy_pbr::prepass_utils;

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{FragmentOutput},
    pbr_deferred_functions::deferred_output,
};
#else
#import bevy_pbr::{
    forward_io::{FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
};
#endif

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec4<f32>,
    @location(2) blend_color: vec3<f32>,
    @location(3) ambient: f32,
    @location(4) instance_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let data = unpack(vertex.data);
    var out: VertexOutput;

    let world_to_local = get_world_from_local(vertex.instance_index);
    out.world_position = mesh_position_local_to_world(
        world_to_local,
        data.position
    );
    out.position = position_world_to_clip(out.world_position.xyz);
    out.world_normal = mesh_normal_local_to_world(
        data.normal,
        vertex.instance_index
    );
    out.instance_index = vertex.instance_index;

    out.blend_color = vec3<f32>(0.0, 0.2, 0.0);
    out.ambient = 1.0;

    return out;
}

@fragment
fn fragment(input: VertexOutput) -> FragmentOutput {
  var pbr_input = pbr_input_new();

  pbr_input.flags = mesh[input.instance_index].flags;

  pbr_input.V = calculate_view(input.world_position, false);
  pbr_input.frag_coord = input.position;
  pbr_input.world_position = input.world_position;

  pbr_input.world_normal = prepare_world_normal(
    input.world_normal,
    false,
    false,
  );
#ifdef LOAD_PREPASS_NORMALS
  pbr_input.N = prepass_utils::prepass_normal(input.position, 0u);
#else
  pbr_input.N = normalize(pbr_input.world_normal);
#endif

  pbr_input.material.base_color = vec4<f32>(input.blend_color * input.ambient, 1.0);

  //pbr_input.material.reflectance = chunk_material.reflectance;
  //pbr_input.material.perceptual_roughness = chunk_material.perceptual_roughness;
  //pbr_input.material.metallic = chunk_material.metallic;


#ifdef PREPASS_PIPELINE
  // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
  let out = deferred_output(in, pbr_input);
#else
  var out: FragmentOutput;
  // apply lighting
  out.color = apply_pbr_lighting(pbr_input);
  out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

  return out;
}