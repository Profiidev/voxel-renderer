use bevy::{
  mesh::MeshVertexBufferLayoutRef,
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
  shader::ShaderRef,
};

use crate::voxel::chunk::mesh::DATA_ATTRIBUTE;

const SHADER_PATH: &str = "shaders/chunk.wgsl";
const PREPASS_SHADER_PATH: &str = "shaders/chunk_prepass.wgsl";
pub type ChunkMaterialPlugin = MaterialPlugin<ChunkMaterial>;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {}

impl Material for ChunkMaterial {
  fn fragment_shader() -> ShaderRef {
    SHADER_PATH.into()
  }

  fn vertex_shader() -> ShaderRef {
    SHADER_PATH.into()
  }

  fn alpha_mode(&self) -> AlphaMode {
    AlphaMode::Opaque
  }

  fn specialize(
    _pipeline: &MaterialPipeline,
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayoutRef,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout
      .0
      .get_layout(&[DATA_ATTRIBUTE.at_shader_location(0)])?;

    //descriptor.primitive.polygon_mode = bevy::render::render_resource::PolygonMode::Line;
    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
  }

  fn prepass_vertex_shader() -> ShaderRef {
    PREPASS_SHADER_PATH.into()
  }

  fn prepass_fragment_shader() -> ShaderRef {
    PREPASS_SHADER_PATH.into()
  }
}
