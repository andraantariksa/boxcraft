use crate::game::camera::Camera;
use crate::misc::window::Window;
use crate::renderer::camera::CameraRenderer;
use crate::renderer::context::RenderContext;
use crate::renderer::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};
use crate::renderer::vertex::{Vertex, VertexLike};
use nalgebra::{Point3, Vector2, Vector3};

use crate::game::world::block::FacesRawInstance;
use crate::renderer::texture::Texture;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    include_spirv, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendState, Buffer,
    BufferBindingType, BufferUsages, ColorTargetState, ColorWrites, CompareFunction,
    DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineLayout,
    PolygonMode, RenderPass, RenderPipelineDescriptor, SamplerBindingType, ShaderModule,
    ShaderModuleDescriptor, ShaderStages, TextureFormat, TextureSampleType, TextureViewDimension,
    VertexBufferLayout,
};

pub struct GameRenderer {
    camera_renderer: CameraRenderer,
    camera_bind_group: BindGroup,
    texture_bind_group: BindGroup,
    wireframe_only: bool,
    render_pipeline: wgpu::RenderPipeline,
    // render_pipeline_descriptor: RenderPipelineDescriptor<'static>,
    cube_vertex_buffer: Buffer,
    cube_indices_buffer: Buffer,
    block_instances_buffer: Buffer,
    fragment_shader_module: ShaderModule,
    vertex_shader_module: ShaderModule,
    color_targets_state: [Option<ColorTargetState>; 1],
    block_instance_vertex_buffer_layout: [VertexBufferLayout<'static>; 2],
    cubes_pipeline_layout: PipelineLayout,

    texture_atlas: Texture,
    pub depth_texture: Texture,

    blocks_total: u32,
}

impl GameRenderer {
    pub fn new(render_context: &RenderContext, window: &Window, camera: &Camera) -> Self {
        let camera_renderer = CameraRenderer::new(render_context, window, camera);

        let camera_bind_group_layout =
            render_context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Create bind group layout: Bind group layout descriptor"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let camera_bind_group =
            render_context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Create bind group"),
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_renderer.as_entire_binding(),
                    }],
                });

        let texture_atlas =
            Texture::load_bytes(render_context, include_bytes!("../assets/atlas.png")).unwrap();

        let texture_bind_group_layout =
            render_context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Create bind group layout: Bind group layout descriptor"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: true },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 2,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let texture_atlas_size = (16, 16);
        let texture_atlas_size_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: any_sized_as_u8_slice(&texture_atlas_size),
                    usage: BufferUsages::UNIFORM,
                });

        let texture_bind_group = render_context
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Create bind group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&texture_atlas.texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&texture_atlas.sampler),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: texture_atlas_size_buffer.as_entire_binding(),
                    },
                ],
            });

        let game_pipeline_layout =
            render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Create pipeline layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let vertex_shader_module =
            render_context
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: Some("Shader module vert"),
                    source: include_spirv!("./shaders/vertex.vert.spv").source,
                });

        let fragment_shader_module =
            render_context
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: Some("Shader module frag"),
                    source: include_spirv!("./shaders/fragment.frag.spv").source,
                });

        let vertices = [
            // Front
            // 0, 1, 2 & 0, 2, 3
            Vertex {
                position: Point3::new(-0.5, -0.5, 0.0),
                normal: Vector3::new(0.0, 0.0, 1.0),
                texture_coordinate: Vector2::new(1.0, 1.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, 0.0),
                normal: Vector3::new(0.0, 0.0, 1.0),
                texture_coordinate: Vector2::new(0.0, 1.0),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, 0.0),
                normal: Vector3::new(0.0, 0.0, 1.0),
                texture_coordinate: Vector2::new(0.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, 0.0),
                normal: Vector3::new(0.0, 0.0, 1.0),
                texture_coordinate: Vector2::new(1.0, 0.0),
            },
        ];
        let indices = [
            0u16, 1, 2, 0, 2, 3, // Front
        ];

        let block_instances_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("Block instances buffer init"),
                    contents: any_sized_as_u8_slice(&()),
                    usage: BufferUsages::VERTEX,
                });
        let cubes_vertices_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("Cubes vertex buffer init"),
                    contents: any_slice_as_u8_slice(&vertices),
                    usage: BufferUsages::VERTEX,
                });
        let cubes_indices_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("Cubes indices buffer init"),
                    contents: any_slice_as_u8_slice(&indices),
                    usage: BufferUsages::INDEX,
                });

        let block_instance_vertex_buffer_layout = [
            Vertex::vertex_buffer_layout(),
            FacesRawInstance::vertex_buffer_layout(),
        ];

        let color_targets_state = [Some(ColorTargetState {
            format: TextureFormat::Bgra8UnormSrgb,
            blend: Some(BlendState {
                color: BlendComponent::REPLACE,
                alpha: BlendComponent::REPLACE,
            }),
            write_mask: ColorWrites::all(),
        })];

        let depth_texture = Texture::new_depth(render_context);
        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Create render pipeline: Render pipeline descriptor"),
            layout: Some(&game_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "main",
                buffers: &block_instance_vertex_buffer_layout,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            // depth_stencil: None,
            depth_stencil: Some(DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &fragment_shader_module,
                entry_point: "main",
                targets: &color_targets_state,
            }),
            multiview: None,
        };

        let render_pipeline = render_context
            .device
            .create_render_pipeline(&render_pipeline_descriptor);

        Self {
            camera_renderer,
            camera_bind_group,
            wireframe_only: render_pipeline_descriptor.primitive.polygon_mode != PolygonMode::Fill,
            cubes_pipeline_layout: game_pipeline_layout,
            render_pipeline,

            cube_vertex_buffer: cubes_vertices_buffer,
            cube_indices_buffer: cubes_indices_buffer,
            block_instances_buffer,

            fragment_shader_module,
            vertex_shader_module,

            color_targets_state,
            block_instance_vertex_buffer_layout, // render_pipeline_descriptor,
            blocks_total: 0,
            depth_texture,
            texture_atlas,
            texture_bind_group,
        }
    }

    pub fn update_blocks(
        &mut self,
        render_context: &RenderContext,
        blocks: &Vec<FacesRawInstance>,
        blocks_total: u32,
    ) {
        self.blocks_total = blocks_total;
        self.block_instances_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("Block instances buffer recreation"),
                    contents: any_slice_as_u8_slice(blocks.as_slice()),
                    usage: BufferUsages::VERTEX,
                })
    }

    pub fn prerender(&self, render_context: &RenderContext, window: &Window, camera: &Camera) {
        self.camera_renderer.update(render_context, window, camera);
    }

    pub fn render<'b>(&'b self, render_pass: &mut RenderPass<'b>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.block_instances_buffer.slice(..));

        render_pass.set_index_buffer(
            self.cube_indices_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.draw_indexed(0..6, 0, 0..self.blocks_total);
    }

    pub fn is_wireframe_only(&self) -> bool {
        self.wireframe_only
    }

    pub fn set_display_wireframe_only(
        &mut self,
        render_context: &RenderContext,
        wireframe_only: bool,
    ) {
        self.wireframe_only = wireframe_only;

        let polygon_mode = if wireframe_only {
            PolygonMode::Line
        } else {
            PolygonMode::Fill
        };
        self.render_pipeline =
            render_context
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: Some("Change render pipeline: Render pipeline descriptor"),
                    layout: Some(&self.cubes_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &self.vertex_shader_module,
                        entry_point: "main",
                        buffers: &self.block_instance_vertex_buffer_layout,
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        unclipped_depth: false,
                        polygon_mode,
                        conservative: false,
                    },
                    depth_stencil: Some(DepthStencilState {
                        format: Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::Less,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(FragmentState {
                        module: &self.fragment_shader_module,
                        entry_point: "main",
                        targets: &self.color_targets_state,
                    }),
                    multiview: None,
                });
    }
}
