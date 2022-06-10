use crate::renderer::context::RenderContext;
use crate::renderer::error::RendererError;
use image::io::Reader;
use image::{DynamicImage, EncodableLayout, RgbaImage};
use std::io::Cursor;
use std::num::NonZeroU32;
use std::path::Path;
use wgpu::{TextureDimension, TextureFormat};

pub struct ImageTexture {
    texture: wgpu::Texture,
}

impl ImageTexture {
    pub fn load_bytes(
        context: RenderContext,
        buffer: &[u8],
    ) -> Result<ImageTexture, RendererError> {
        let image = image::load_from_memory(buffer).unwrap().to_rgba8();
        let (image_width, image_height) = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: image_width,
            height: image_height,
            depth_or_array_layers: 1,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture image"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        context.queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image.as_bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * image_width),
                rows_per_image: NonZeroU32::new(image_height),
            },
            texture_size,
        );

        Ok(Self { texture })
    }
}
