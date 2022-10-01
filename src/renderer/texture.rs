use crate::renderer::context::RenderContext;
use crate::renderer::error::RendererError;

use image::EncodableLayout;

use std::num::NonZeroU32;

use wgpu::{
    AddressMode, CompareFunction, Extent3d, FilterMode, ImageCopyTextureBase, ImageDataLayout,
    Origin3d, Sampler, SamplerDescriptor, SurfaceConfiguration, Texture as WGPUTexture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

pub struct Texture {
    pub texture: WGPUTexture,
    pub texture_view: TextureView,
    pub sampler: Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn load_bytes(context: &RenderContext, buffer: &[u8]) -> Result<Texture, RendererError> {
        let image = image::load_from_memory(buffer).unwrap().to_rgba8();
        let (image_width, image_height) = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: image_width,
            height: image_height,
            depth_or_array_layers: 1,
        };

        let texture = context.device.create_texture(&TextureDescriptor {
            label: Some("Create texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        context.queue.write_texture(
            ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            image.as_bytes(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * image_width),
                rows_per_image: NonZeroU32::new(image_height),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor {
            label: Some("Create texture view"),
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let sampler = context.device.create_sampler(&SamplerDescriptor {
            label: Some("Create sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        Ok(Self {
            texture,
            texture_view,
            sampler,
        })
    }

    pub fn new_depth(render_context: &RenderContext) -> Self {
        let texture_size = Extent3d {
            width: render_context.render_surface_config.width,
            height: render_context.render_surface_config.height,
            depth_or_array_layers: 1,
        };

        let texture = render_context.device.create_texture(&TextureDescriptor {
            label: Some("Create texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        let texture_view = texture.create_view(&TextureViewDescriptor {
            label: Some("Create texture view"),
            format: Some(Self::DEPTH_FORMAT),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let sampler = render_context.device.create_sampler(&SamplerDescriptor {
            label: Some("Create sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::LessEqual),
            anisotropy_clamp: None,
            border_color: None,
        });

        Self {
            texture,
            texture_view,
            sampler,
        }
    }
}
