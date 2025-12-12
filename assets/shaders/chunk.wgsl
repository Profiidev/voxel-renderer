#import bevy_render::mesh_functions::{mesh_position_local_to_clip, get_model_matrix};

struct Vertex {
  @builtin(instance_index) instance_index: u32,
  @location(0) pos: vec3<f32>,
  @location(1) data: u32,
}

struct UnpackedData {
  x: f32,
  y: f32,
  z: f32,
  height: f32,
  width: f32,
}

// format: xxxxxyyyyyzzzzzwwwwwhhhhh-------
fn unpack(data: u32) -> UnpackedData {
  let x: f32 = f32((data >> 27) & 0x1F);
  let y: f32 = f32((data >> 22) & 0x1F);
  let z: f32 = f32((data >> 17) & 0x1F);
  let width: f32 = f32((data >> 12) & 0x1F);
  let height: f32 = f32((data >> 7) & 0x1F);
  return UnpackedData(x, y, z, height, width);
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
  let data = unpack(vertex.data);

  var out: VertexOutput;
  let local_position = vec4<f32>(
    data.x,
    data.y,
    data.z,
    1.0,
  );
  out.clip_position = mesh_position_local_to_clip(get_model_matrix(vertex.instance_index), local_position);

  return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(0.0, 1.0, 1.0, 1.0);
}