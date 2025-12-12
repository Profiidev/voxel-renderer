use bevy::{
  asset::RenderAssetUsages,
  math::USizeVec3,
  mesh::{
    Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat,
  },
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
  shader::ShaderRef,
};
use noise::{NoiseFn, Perlin};

const CHUNK_SIZE: usize = 16;
const SHADER_PATH: &str = "shaders/chunk.wgsl";
pub const DATA_ATTRIBUTE: MeshVertexAttribute =
  MeshVertexAttribute::new("Quad data", 658854091321, VertexFormat::Uint32);

pub struct ChunkData {
  chunk_pos: Vec2,
  data: Vec<Vec<Vec<bool>>>,
}

impl ChunkData {
  pub fn new() -> Self {
    let seed = 42;
    let chunk_coords = Vec2::ZERO;

    let noise = Perlin::new(seed);
    let mut hight_map = Vec::with_capacity(CHUNK_SIZE + 2);

    for x in 0..CHUNK_SIZE + 2 {
      let mut row = Vec::with_capacity(CHUNK_SIZE + 2);
      for z in 0..CHUNK_SIZE + 2 {
        let height = noise.get([
          (chunk_coords.x as f64 * CHUNK_SIZE as f64 + x as f64) / 10.0,
          (chunk_coords.y as f64 * CHUNK_SIZE as f64 + z as f64) / 10.0,
        ]);
        row.push(height.round());
      }
      hight_map.push(row);
    }

    let mut cube_map = vec![vec![vec![false; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];
    for x in 0..CHUNK_SIZE + 2 {
      for z in 0..CHUNK_SIZE + 2 {
        let height = hight_map[x][z] as isize + (CHUNK_SIZE as isize / 2);
        for y in 0..CHUNK_SIZE + 2 {
          if (y as isize) <= height {
            cube_map[x][y][z] = true;
          }
        }
      }
    }

    Self {
      chunk_pos: chunk_coords,
      data: cube_map,
    }
  }

  pub fn mesh(self) -> Mesh {
    let mut plain_data = Vec::new();
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for x in 1..CHUNK_SIZE + 1 {
      for y in 1..CHUNK_SIZE + 1 {
        for z in 1..CHUNK_SIZE + 1 {
          let pos = USizeVec3::new(x, y, z);
          if !self.get(pos) {
            continue;
          }

          for dir in 0..6 {
            let neighbor_pos = match dir {
              0 => USizeVec3::new(x + 1, y, z),
              1 => USizeVec3::new(x - 1, y, z),
              2 => USizeVec3::new(x, y + 1, z),
              3 => USizeVec3::new(x, y - 1, z),
              4 => USizeVec3::new(x, y, z + 1),
              5 => USizeVec3::new(x, y, z - 1),
              _ => unreachable!(),
            };

            if neighbor_pos.x >= CHUNK_SIZE + 2
              || neighbor_pos.y >= CHUNK_SIZE + 2
              || neighbor_pos.z >= CHUNK_SIZE + 2
              || !self.get(neighbor_pos)
            {
              let (base, dir1, dir2) = match dir {
                0 => (pos + USizeVec3::X, USizeVec3::Z, USizeVec3::Y),
                1 => (pos, USizeVec3::Z, USizeVec3::Y),
                2 => (pos + USizeVec3::Y, USizeVec3::X, USizeVec3::Z),
                3 => (pos, USizeVec3::X, USizeVec3::Z),
                4 => (pos + USizeVec3::Z, USizeVec3::X, USizeVec3::Y),
                5 => (pos, USizeVec3::X, USizeVec3::Y),
                _ => unreachable!(),
              };

              let start_index = positions.len() as u32;
              indices.push(start_index);
              if dir == 2 || dir == 5 || dir == 0 {
                indices.push(start_index + 3);
                indices.push(start_index + 2);
                indices.push(start_index + 2);
                indices.push(start_index + 1);
              } else {
                indices.push(start_index + 2);
                indices.push(start_index + 3);
                indices.push(start_index + 1);
                indices.push(start_index + 2);
              }
              indices.push(start_index);

              for i in 0..4 {
                let offset = match i {
                  0 => USizeVec3::ZERO,
                  1 => dir1,
                  2 => dir1 + dir2,
                  3 => dir2,
                  _ => unreachable!(),
                };

                let vertex_pos = base + offset;
                positions.push([
                  vertex_pos.x as f32,
                  vertex_pos.y as f32,
                  vertex_pos.z as f32,
                ]);

                let x = vertex_pos.x as u32;
                let y = vertex_pos.y as u32;
                let z = vertex_pos.z as u32;
                let width = 1;
                let height = 1;

                let data: u32 = (x & 0x1F << 27)
                  | ((y & 0x1F) << 22)
                  | ((z & 0x1F) << 17)
                  | ((width & 0x1F) << 12)
                  | ((height & 0x1F) << 7)
                  | (dir & 0x07);

                plain_data.push(data);
              }
            }
          }
        }
      }
    }

    let mut mesh = Mesh::new(
      PrimitiveTopology::TriangleList,
      RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(DATA_ATTRIBUTE, plain_data);
    mesh.insert_indices(Indices::U32(indices));

    mesh
  }

  pub fn get(&self, pos: USizeVec3) -> bool {
    self.data[pos.x][pos.y][pos.z]
  }
}

pub type ChunkMaterialPlugin = MaterialPlugin<ChunkMaterial>;

#[derive(Resource)]
pub struct ChunkMaterialHandle(pub Handle<ChunkMaterial>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {}

impl Material for ChunkMaterial {
  fn fragment_shader() -> ShaderRef {
    SHADER_PATH.into()
  }

  fn vertex_shader() -> ShaderRef {
    SHADER_PATH.into()
  }

  fn specialize(
    _pipeline: &MaterialPipeline,
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayoutRef,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.0.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      DATA_ATTRIBUTE.at_shader_location(1),
    ])?;

    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
  }
}
