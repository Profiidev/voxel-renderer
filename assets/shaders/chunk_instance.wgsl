#import bevy_pbr::mesh_functions::{mesh_position_local_to_clip, get_world_from_local};
#import "shaders/chunk_util.wgsl"::Vertex;

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
  var out: VertexOutput;
  out.position = mesh_position_local_to_clip(
    get_world_from_local(vertex.instance_index),
    vec4<f32>(vertex.position, 1.0),
  );
  return out;
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
  return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
