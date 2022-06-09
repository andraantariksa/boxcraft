use crate::renderer::context::RenderContext;
use crate::renderer::error::RendererError;
use image::io::Reader;
use image::{DynamicImage, RgbaImage};
use std::io::Cursor;
use std::path::Path;
use wgpu::{TextureDimension, TextureFormat};

pub struct ImageTexture {
    texture: wgpu::Texture,
}

impl ImageTexture {
    fn load_bytes(context: RenderContext, bytes: &[u8]) -> Result<ImageTexture, RendererError> {
        let image = Reader::new(Cursor::new(bytes)).decode().unwrap();
        let texture_size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        });

        // context.queue.write_texture(
        //     wgpu::ImageCopyTextureBase {
        //         texture: (),
        //         mip_level: 0,
        //         origin: Default::default(),
        //         aspect: Default::default(),
        //     },
        //     bytes,
        //     wgpu::ImageDataLayout {
        //         offset: 0,
        //         bytes_per_row: None,
        //         rows_per_image: None,
        //     },
        //     texture_size,
        // );

        Ok(Self { texture })
    }
}
