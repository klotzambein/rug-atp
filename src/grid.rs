use std::{
    cmp::{max, min},
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use glium::VertexBuffer;
use glium::{texture::Texture2d, Display};
use gui_framework::{
    canvas::{CanvasError, CanvasObject, DrawingContext},
    graphics::primitives::{Sprite, Vf2},
    texture::load_png_texture,
};

use euclid::{Box2D, Point2D, Vector2D};

use crate::tile::TileTexture;

pub struct CanvasGrid {
    chunks: Vec<GridChunk>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    texture: Texture2d,
}

impl CanvasObject for CanvasGrid {
    fn draw<'a>(&self, ctx: &mut DrawingContext<'a>) -> Result<(), CanvasError> {
        let transform = ctx
            .view_transform
            .inverse()
            .expect("Failed to inverse model transform");

        // Bounding box
        let bb = Box2D::new(
            transform.transform_point(Point2D::new(-1., -1.)),
            transform.transform_point(Point2D::new(1., 1.)),
        );

        let min_x = min(max((bb.min.x / 320.0) as i32, 0) as usize, self.width);
        let min_y = min(max((bb.min.y / 320.0) as i32, 0) as usize, self.height);
        let max_x = min(max((bb.max.x / 320.0) as i32, 0) as usize + 1, self.width);
        let max_y = min(max((bb.max.y / 320.0) as i32, 0) as usize + 1, self.height);

        for y in min_y..max_y {
            let i_row = y * self.width;
            for (c, x) in self.chunks[i_row + min_x..i_row + max_x]
                .iter()
                .zip(min_x..)
            {
                let model_transform = ctx
                    .model_transform
                    .then_translate(Vector2D::new(x as f32 * 320., y as f32 * 320.));
                ctx.programs.draw_sprites(
                    ctx.target,
                    c.vertex_buffer.slice(..).unwrap(),
                    &self.texture,
                    model_transform,
                    ctx.view_transform,
                )?;
            }
        }

        Ok(())
    }
}

impl CanvasGrid {
    pub fn new(display: &Display, width: usize, height: usize) -> CanvasGrid {
        CanvasGrid {
            chunks: std::iter::repeat_with(|| GridChunk::new(display))
                .take(width * height)
                .collect(),
            width: width,
            height: height,
            texture: load_png_texture(display, include_bytes!("./../../../assets/tileset.png")),
        }
    }
    pub fn update_chunk(
        &self,
        chunk: (usize, usize),
        tiles: impl Iterator<Item = TileTexture> + Clone,
    ) {
        assert!(chunk.0 < self.width, chunk.1 < self.height);
        let chunk = &self.chunks[chunk.1 * self.width + chunk.0];
        let hash = {
            let tiles = tiles.clone();
            let mut hasher = DefaultHasher::new();
            for t in tiles {
                t.hash(&mut hasher);
            }
            hasher.finish()
        };
        if hash != chunk.hash {
            let data = tiles
                .enumerate()
                .map(|(i, t)| Sprite {
                    vertex: Vf2::new((i % 32) as f32 * 10.0 + 0.5, (i / 32) as f32 * 10.0 + 0.5),
                    size: Vf2::new(10., 10.),
                    texture_index: t as i32,
                })
                .collect::<Vec<_>>();
            chunk.vertex_buffer.write(&data)
        }
    }
}

pub struct GridChunk {
    vertex_buffer: VertexBuffer<Sprite>,
    hash: u64,
}

impl GridChunk {
    pub fn new(display: &Display) -> GridChunk {
        GridChunk {
            vertex_buffer: VertexBuffer::new(
                display,
                &(0..32 * 32)
                    .map(|i| Sprite {
                        vertex: Vf2::new(
                            (i % 32) as f32 * 10.0 + 0.5,
                            (i / 32) as f32 * 10.0 + 0.5,
                        ),
                        size: Vf2::new(10., 10.),
                        texture_index: i % 64,
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
            hash: 0,
        }
    }
}
