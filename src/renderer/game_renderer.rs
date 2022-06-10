use crate::game::camera::Camera;
use crate::renderer::camera::CameraRenderer;
use crate::renderer::context::RenderContext;
use crate::renderer::util::{any_sized_as_u8_slice, any_slice_as_u8_slice};
use crate::renderer::vertex::Vertex;
use nalgebra::{Point3, Vector3};
use std::mem;
use std::time::Duration;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    include_spirv, include_spirv_raw, Buffer, BufferAddress, ColorTargetState, FragmentState,
    MultisampleState, PolygonMode, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor,
    ShaderModuleDescriptorSpirV, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use winit::window::Window;

pub struct GameRenderer {
    camera_renderer: CameraRenderer,
    camera_bind_group: wgpu::BindGroup,
    cubes_pipeline_layout: wgpu::PipelineLayout,
    wireframe_only: bool,
    render_pipeline: wgpu::RenderPipeline,
    // render_pipeline_descriptor: RenderPipelineDescriptor<'static>,
    cubes_vertex_buffer: Buffer,
    cubes_indices_buffer: Buffer,
}

impl GameRenderer {
    pub fn new(render_context: &RenderContext, camera: &Camera) -> Self {
        let camera_renderer = CameraRenderer::new(render_context, camera);

        let camera_bind_group_layout =
            render_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Create bind group layout: Bind group layout descriptor"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
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
                    label: Some("Create bind group: Bind group descriptor"),
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_renderer.as_entire_binding(),
                    }],
                });

        let cubes_pipeline_layout =
            render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Create pipeline layout: Pipeline layout descriptor"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let vertex_shader_module =
            render_context
                .device
                .create_shader_module(&ShaderModuleDescriptor {
                    label: Some("Shader module vert"),
                    source: include_spirv!("./shaders/vertex.vert.spv").source,
                });

        let fragment_shader_module =
            render_context
                .device
                .create_shader_module(&ShaderModuleDescriptor {
                    label: Some("Shader module frag"),
                    source: include_spirv!("./shaders/fragment.frag.spv").source,
                });

        let vertices = [
            Vertex {
                position: Point3::new(-0.5, -0.5, 0.5),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, 0.5),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, 0.5),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, 0.5),
            },
            Vertex {
                position: Point3::new(-0.5, -0.5, -0.5),
            },
            Vertex {
                position: Point3::new(0.5, -0.5, -0.5),
            },
            Vertex {
                position: Point3::new(0.5, 0.5, -0.5),
            },
            Vertex {
                position: Point3::new(-0.5, 0.5, -0.5),
            },
        ];
        let indices: [u16; 4] = [0, 3, 1, 2];

        let cubes_vertex_buffer = render_context
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Cubes vertex buffer init"),
                contents: any_slice_as_u8_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let cubes_indices_buffer =
            render_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: Some("Cubes indices buffer init"),
                    contents: any_slice_as_u8_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let cubes_vertex_buffer_layout = VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        };

        let color_targets_state = [ColorTargetState {
            format: TextureFormat::Bgra8UnormSrgb,
            blend: None,
            write_mask: Default::default(),
        }];

        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Create render pipeline: Render pipeline descriptor"),
            layout: Some(&cubes_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "main",
                buffers: &[cubes_vertex_buffer_layout],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: Default::default(),
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
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
            cubes_pipeline_layout,
            cubes_vertex_buffer,
            render_pipeline,
            cubes_indices_buffer,
            // render_pipeline_descriptor,
        }
    }

    pub fn prerender(&self, render_context: &RenderContext, camera: &Camera) {
        self.camera_renderer.update(render_context, camera);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.cubes_vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.cubes_indices_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..4, 0, 0..1);
    }

    pub fn set_display_wireframe_only(
        &mut self,
        render_context: &RenderContext,
        wireframe_only: bool,
    ) {
        self.wireframe_only = wireframe_only;

        // self.render_pipeline_descriptor.primitive.polygon_mode = if wireframe_only {
        //     PolygonMode::Line
        // } else {
        //     PolygonMode::Fill
        // };
        // self.render_pipeline = render_context
        //     .device
        //     .create_render_pipeline(&self.render_pipeline_descriptor);
    }
}
