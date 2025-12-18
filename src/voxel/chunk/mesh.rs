use crate::voxel::chunk::{CHUNK_SIZE, CHUNK_SIZE_POW, generation::ChunkBlockData};
use bevy::{
  asset::RenderAssetUsages,
  math::USizeVec3,
  mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexFormat},
  prelude::*,
};

pub const DATA_ATTRIBUTE: MeshVertexAttribute =
  MeshVertexAttribute::new("Quad data", 658854091321, VertexFormat::Uint32);

impl ChunkBlockData {
  pub fn mesh(self) -> Mesh {
    let mut plain_data = Vec::new();
    let mut indices = Vec::new();

    let chunk_size_mask: u32 = (1 << CHUNK_SIZE_POW) - 1;

    for x in 1..CHUNK_SIZE + 1 {
      for y in 1..CHUNK_SIZE + 1 {
        for z in 1..CHUNK_SIZE + 1 {
          let pos = USizeVec3::new(x, y, z);
          if self.empty(pos) {
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
              || self.empty(neighbor_pos)
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

              let start_index = plain_data.len() as u32;
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

                let x = vertex_pos.x as u32;
                let y = vertex_pos.y as u32;
                let z = vertex_pos.z as u32;
                let width = 1;
                let height = 1;

                let data: u32 = (x & chunk_size_mask) << (32 - CHUNK_SIZE_POW)
                  | ((y & chunk_size_mask) << (32 - 2 * CHUNK_SIZE_POW))
                  | ((z & chunk_size_mask) << (32 - 3 * CHUNK_SIZE_POW))
                  | ((width & chunk_size_mask) << (32 - 4 * CHUNK_SIZE_POW))
                  | ((height & chunk_size_mask) << (32 - 5 * CHUNK_SIZE_POW))
                  | (dir & 7);

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

    mesh.insert_attribute(DATA_ATTRIBUTE, plain_data);
    mesh.insert_indices(Indices::U32(indices));

    mesh
  }
}
