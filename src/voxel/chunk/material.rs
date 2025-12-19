use bevy::{
  core_pipeline::core_3d::Transparent3d,
  ecs::{
    query::{QueryItem, ROQueryItem},
    system::{
      SystemParamItem,
      lifetimeless::{Read, SRes},
    },
  },
  mesh::{MeshVertexBufferLayoutRef, VertexBufferLayout, VertexFormat},
  pbr::{
    MaterialPipeline, MaterialPipelineKey, MeshPipeline, MeshPipelineKey, RenderMeshInstances,
    SetMeshBindGroup, SetMeshViewBindGroup, SetMeshViewBindingArrayBindGroup,
  },
  prelude::*,
  render::{
    Render, RenderApp, RenderStartup, RenderSystems,
    extract_component::{ExtractComponent, ExtractComponentPlugin},
    mesh::{RenderMesh, RenderMeshBufferInfo, allocator::MeshAllocator},
    render_asset::RenderAssets,
    render_phase::{
      AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
      RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
    },
    render_resource::{
      AsBindGroup, Buffer, BufferInitDescriptor, BufferUsages, PipelineCache,
      RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError,
      SpecializedMeshPipelines, VertexAttribute, VertexStepMode,
    },
    renderer::RenderDevice,
    sync_world::MainEntity,
    view::ExtractedView,
  },
  shader::ShaderRef,
};
use bytemuck::{Pod, Zeroable};

use crate::voxel::chunk::mesh::DATA_ATTRIBUTE;

const SHADER_PATH: &str = "shaders/chunk.wgsl";
const PREPASS_SHADER_PATH: &str = "shaders/chunk_prepass.wgsl";

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

#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData {
  pub data: u32,
}

#[derive(Component, Deref)]
pub struct InstanceMaterialData(pub Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
  type QueryData = &'static InstanceMaterialData;
  type QueryFilter = ();
  type Out = Self;

  fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self::Out> {
    Some(InstanceMaterialData(item.0.clone()))
  }
}

pub struct ChunkMaterialPlugin;

impl Plugin for ChunkMaterialPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
    app
      .sub_app_mut(RenderApp)
      .add_render_command::<Transparent3d, DrawChunk>()
      .init_resource::<SpecializedMeshPipelines<ChunkPipeline>>()
      .add_systems(RenderStartup, init_chunk_pipeline)
      .add_systems(
        Render,
        (
          queue_chunk.in_set(RenderSystems::QueueMeshes),
          prepare_instance_buffers.in_set(RenderSystems::PrepareResources),
        ),
      );
  }
}

#[allow(clippy::too_many_arguments)]
fn queue_chunk(
  transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
  chunk_pipeline: Res<ChunkPipeline>,
  mut pipelines: ResMut<SpecializedMeshPipelines<ChunkPipeline>>,
  pipeline_cache: Res<PipelineCache>,
  meshes: Res<RenderAssets<RenderMesh>>,
  render_mesh_instances: Res<RenderMeshInstances>,
  material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
  mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
  views: Query<(&ExtractedView, &Msaa)>,
) {
  let draw_chunk = transparent_3d_draw_functions.read().id::<DrawChunk>();

  for (view, msaa) in &views {
    let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
    else {
      continue;
    };

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
    let rangefinder = view.rangefinder3d();

    for (entity, main_entity) in &material_meshes {
      let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*main_entity) else {
        continue;
      };
      let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
        continue;
      };
      let key = view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology());
      let pipeline = pipelines
        .specialize(&pipeline_cache, &chunk_pipeline, key, &mesh.layout)
        .unwrap();
      transparent_phase.add(Transparent3d {
        entity: (entity, *main_entity),
        pipeline,
        draw_function: draw_chunk,
        distance: rangefinder.distance_translation(&mesh_instance.translation),
        batch_range: 0..1,
        extra_index: PhaseItemExtraIndex::None,
        indexed: true,
      })
    }
  }
}

#[derive(Component)]
struct InstanceBuffer {
  buffer: Buffer,
  length: usize,
}

fn prepare_instance_buffers(
  mut commands: Commands,
  query: Query<(Entity, &InstanceMaterialData)>,
  render_device: Res<RenderDevice>,
) {
  for (entity, instance_data) in &query {
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: Some("chunk_instance_buffer"),
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
      contents: bytemuck::cast_slice(instance_data.as_slice()),
    });
    commands.entity(entity).insert(InstanceBuffer {
      buffer,
      length: instance_data.len(),
    });
  }
}

#[derive(Resource)]
struct ChunkPipeline {
  shader: Handle<Shader>,
  mesh_pipeline: MeshPipeline,
}

fn init_chunk_pipeline(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mesh_pipeline: Res<MeshPipeline>,
) {
  commands.insert_resource(ChunkPipeline {
    shader: asset_server.load("shaders/chunk_instance.wgsl"),
    mesh_pipeline: mesh_pipeline.clone(),
  });
}

impl SpecializedMeshPipeline for ChunkPipeline {
  type Key = MeshPipelineKey;

  fn specialize(
    &self,
    key: Self::Key,
    layout: &MeshVertexBufferLayoutRef,
  ) -> std::result::Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
    let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

    descriptor.vertex.shader = self.shader.clone();
    descriptor.vertex.buffers.push(VertexBufferLayout {
      array_stride: size_of::<InstanceData>() as u64,
      step_mode: VertexStepMode::Instance,
      attributes: vec![VertexAttribute {
        format: VertexFormat::Uint32,
        offset: 0,
        shader_location: 3,
      }],
    });
    descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
    Ok(descriptor)
  }
}

type DrawChunk = (
  SetItemPipeline,
  SetMeshViewBindGroup<0>,
  SetMeshViewBindingArrayBindGroup<1>,
  SetMeshBindGroup<2>,
  DrawMeshInstanced,
);

struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
  type Param = (
    SRes<RenderAssets<RenderMesh>>,
    SRes<RenderMeshInstances>,
    SRes<MeshAllocator>,
  );
  type ViewQuery = ();
  type ItemQuery = Read<InstanceBuffer>;

  #[inline]
  fn render<'w>(
    item: &P,
    _view: ROQueryItem<'w, '_, Self::ViewQuery>,
    instance_buffer: Option<ROQueryItem<'w, '_, Self::ItemQuery>>,
    (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
    pass: &mut TrackedRenderPass<'w>,
  ) -> RenderCommandResult {
    let mesh_allocator = mesh_allocator.into_inner();

    let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(item.main_entity())
    else {
      return RenderCommandResult::Skip;
    };
    let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
      return RenderCommandResult::Skip;
    };
    let Some(instance_buffer) = instance_buffer else {
      return RenderCommandResult::Skip;
    };
    let Some(vertex_buffer_slice) = mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
    else {
      return RenderCommandResult::Skip;
    };

    pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));
    pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

    match &gpu_mesh.buffer_info {
      RenderMeshBufferInfo::Indexed {
        count,
        index_format,
      } => {
        let Some(index_buffer_slice) =
          mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
        else {
          return RenderCommandResult::Skip;
        };

        pass.set_index_buffer(index_buffer_slice.buffer.slice(..), 0, *index_format);
        pass.draw_indexed(
          index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
          vertex_buffer_slice.range.start as i32,
          0..instance_buffer.length as u32,
        );
      }
      RenderMeshBufferInfo::NonIndexed => {
        //pass.draw(vertex_buffer_slice.range, 0..instance_buffer.length as u32);
        unreachable!("The chunk mesh is always indexed");
      }
    }
    RenderCommandResult::Success
  }
}
