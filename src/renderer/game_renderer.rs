use crate::game::camera::Camera;
use crate::misc::window::Window;
use crate::renderer::camera::CameraRenderer;
use crate::renderer::context::RenderContext;
use crate::renderer::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};
use crate::renderer::vertex::{Vertex, VertexLike};
use nalgebra::{Matrix4, Point3, Vector3};

use crate::game::world::block::BlockRawInstance;
use crate::renderer::texture::Texture;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    include_spirv, BlendComponent, BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites,
    CompareFunction, DepthStencilState, Face, FragmentState, FrontFace, MultisampleState,
    PipelineLayout, PolygonMode, RenderPass, RenderPipelineDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderStages, TextureFormat, VertexBufferLayout,
};

pub struct GameRenderer {
    camera_renderer: CameraRenderer,
    camera_bind_group: wgpu::BindGroup,
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
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let cube_pipeline_layout =
            render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Create pipeline layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
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
                position: Point3::new(-0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 0.0, 1.0),
            },
            // Right
            // 0, 2, 1 & 1, 3, 2
            Vertex {
                position: Point3::new(0.5, 0.5, 0.5),
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, 0.5),
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, -0.5),
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, -0.5),
                normal: Vector3::new(1.0, 0.0, 0.0),
            },
            // Left
            // 1, 0, 3 & 2, 3, 0
            Vertex {
                position: Point3::new(-0.5, 0.5, 0.5),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, -0.5, 0.5),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, -0.5),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, -0.5, -0.5),
                normal: Vector3::new(-1.0, 0.0, 0.0),
            },
            // Back
            // 1, 0, 3 & 1, 3, 2
            Vertex {
                position: Point3::new(-0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 0.0, -1.0),
            },
            // Top
            // 0, 2, 1 & 1, 2, 3
            Vertex {
                position: Point3::new(0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, 0.5),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, -0.5),
                normal: Vector3::new(0.0, 1.0, 0.0),
            },
            // Bottom
            // 0, 1, 3 & 0, 3, 2
            Vertex {
                position: Point3::new(0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, -0.5, 0.5),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
            Vertex {
                position: Point3::new(-0.5, -0.5, -0.5),
                normal: Vector3::new(0.0, -1.0, 0.0),
            },
        ];
        let indices = [
            0u16, 1, 2, 0, 2, 3, // Front
            13, 12, 15, 13, 15, 14, // Back
            4, 5, 6, 5, 7, 6, // Right
            9, 8, 11, 10, 11, 8, // Left
            16, 18, 17, 17, 18, 19, // Top
            20, 21, 23, 20, 23, 22, // Bottom
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
            BlockRawInstance::vertex_buffer_layout(),
        ];

        let color_targets_state = [Some(ColorTargetState {
            format: TextureFormat::Bgra8UnormSrgb,
            blend: Some(BlendState {
                color: BlendComponent::REPLACE,
                alpha: BlendComponent::REPLACE,
            }),
            write_mask: ColorWrites::all(),
        })];

        let depth_texture = Texture::new_depth(&render_context);
        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Create render pipeline: Render pipeline descriptor"),
            layout: Some(&cube_pipeline_layout),
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

        let texture_atlas =
            Texture::load_bytes(render_context, include_bytes!("../assets/atlas.png")).unwrap();

        Self {
            camera_renderer,
            camera_bind_group,
            wireframe_only: render_pipeline_descriptor.primitive.polygon_mode != PolygonMode::Fill,
            cubes_pipeline_layout: cube_pipeline_layout,
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
        }
    }

    pub fn update_blocks(
        &mut self,
        render_context: &RenderContext,
        blocks: &Vec<BlockRawInstance>,
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

        render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.block_instances_buffer.slice(..));

        render_pass.set_index_buffer(
            self.cube_indices_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        render_pass.draw_indexed(0..6 * 6, 0, 0..self.blocks_total);
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
