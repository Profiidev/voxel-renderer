struct UnpackedData {
  position: vec4<f32>,
  height: f32,
  width: f32,
  direction: u32,
}

// format: xxxxxyyyyyzzzzzwwwwwhhhhh----ddd
fn unpack(data: u32) -> UnpackedData {
  let x: f32 = f32((data >> 27) & 0x1F);
  let y: f32 = f32((data >> 22) & 0x1F);
  let z: f32 = f32((data >> 17) & 0x1F);
  let width: f32 = f32((data >> 12) & 0x1F);
  let height: f32 = f32((data >> 7) & 0x1F);
  let direction: u32 = data & 7;

  return UnpackedData(vec4<f32>(
    x,
    y,
    z,
    1.0,
  ), height, width, direction);
}

const normals: array<vec3<f32>,6> = array<vec3<f32>,6> (
	vec3<f32>(-1.0, 0.0, 0.0), // Left
	vec3<f32>(1.0, 0.0, 0.0), // Right
	vec3<f32>(0.0, -1.0, 0.0), // Down
	vec3<f32>(0.0, 1.0, 0.0), // Up
	vec3<f32>(0.0, 0.0, -1.0), // Back
	vec3<f32>(0.0, 0.0, 1.0) // Forward
);

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) data: u32,
};