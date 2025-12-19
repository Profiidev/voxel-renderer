use crate::voxel::chunk::CHUNK_SIZE;
use bevy::{math::USizeVec3, prelude::*};
use noise::{NoiseFn, Perlin};

pub struct ChunkBlockData {
  pub(super) data: [u8; (CHUNK_SIZE + 2) * (CHUNK_SIZE + 2) * (CHUNK_SIZE + 2)],
  pub(super) chunk_pos: IVec3,
}

impl ChunkBlockData {
  pub fn create(seed: u32, chunk_pos: IVec3) -> Self {
    let noise = Perlin::new(seed);
    let mut height_map = [[0f64; CHUNK_SIZE + 2]; CHUNK_SIZE + 2];

    for (x, row) in height_map.iter_mut().enumerate() {
      for (z, cell) in row.iter_mut().enumerate() {
        let height = noise.get([
          (chunk_pos.x as f64 * CHUNK_SIZE as f64 + x as f64) / 50.0,
          (chunk_pos.z as f64 * CHUNK_SIZE as f64 + z as f64) / 50.0,
        ]) * 20.0;
        *cell = height;
      }
    }

    let mut data = [0u8; (CHUNK_SIZE + 2) * (CHUNK_SIZE + 2) * (CHUNK_SIZE + 2)];

    for (x, row) in height_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        for y in 0..CHUNK_SIZE + 2 {
          if (y as i32 + (chunk_pos.y * CHUNK_SIZE as i32)) <= cell.round() as i32 {
            let index = get_index(x, y, z);
            data[index] = 1;
          }
        }
      }
    }

    Self { data, chunk_pos }
  }

  #[inline]
  pub fn get(&self, pos: USizeVec3) -> u8 {
    self.data[get_index(pos.x, pos.y, pos.z)]
  }

  #[inline]
  pub fn empty(&self, pos: USizeVec3) -> bool {
    self.get(pos) == 0
  }
}

#[inline]
fn get_index(x: usize, y: usize, z: usize) -> usize {
  x * (CHUNK_SIZE + 2) * (CHUNK_SIZE + 2) + y * (CHUNK_SIZE + 2) + z
}
