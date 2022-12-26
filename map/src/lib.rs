// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{IVec2, IVec3, Mat4, Vec3};
use glium::{uniform, Surface};
use image::DynamicImage;
use std::{cell::RefCell, collections::HashMap};
use wolkenwelten_client::{ClientState, RenderInitArgs, RenderPassArgs, TextMesh, Texture};
use wolkenwelten_core::{
    ChunkBlockData, ChunkFluidData, GameState, Message, Reactor, BLOCKS, CHUNK_BITS, CHUNK_MASK,
    CHUNK_SIZE, FLUIDS,
};

thread_local! {
    pub static MAP_CACHE:RefCell<MapCache> = RefCell::new(Default::default());
}

#[derive(Clone, Debug, Default)]
pub struct MapCacheEntry {
    tile: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

#[derive(Debug, Default)]
pub struct MapCache {
    cache: Option<(IVec2, Texture)>,
}

static MAP_SIZE: (i32, i32) = (512, 512);
static RENDER_SIZE: i32 = 2048;

impl MapCache {
    fn render_image(
        &self,
        game: &GameState,
        cam_pos: (i32, i32),
        size: (i32, i32),
    ) -> DynamicImage {
        let mut chunks: HashMap<IVec2, Vec<(i32, &ChunkBlockData, &ChunkFluidData)>> =
            HashMap::new();
        let world = game.world();
        let blocks = world.blocks();
        let fluids = world.fluids();
        for (pos, chunk) in blocks.iter() {
            let k = IVec2::new(pos.x, pos.z);
            let fluid_chunk = fluids.get(pos).unwrap();
            let entry = (pos.y, chunk, fluid_chunk);
            if let Some(vec) = chunks.get_mut(&k) {
                vec.push(entry);
            } else {
                chunks.insert(k, vec![entry]);
            }
        }

        let entries: HashMap<IVec2, MapCacheEntry> = chunks
            .iter_mut()
            .map(|(pos, vec)| {
                vec.sort_by(|a, b| b.0.cmp(&a.0));
                let mut e: MapCacheEntry = Default::default();
                for (_, chunk, fluid) in vec {
                    for tx in 0..CHUNK_SIZE {
                        for tz in 0..CHUNK_SIZE {
                            if e.tile[tx][tz] != 0 {
                                continue;
                            }
                            for ty in (0..CHUNK_SIZE).rev() {
                                let b = chunk.data[tx][ty][tz];
                                if b == 0 {
                                    let b = fluid.data[tx][ty][tz];
                                    if b == 0 {
                                        continue;
                                    } else {
                                        e.tile[tx][tz] = 240 + b;
                                        break;
                                    }
                                } else {
                                    e.tile[tx][tz] = b;
                                    break;
                                }
                            }
                        }
                    }
                }
                (*pos, e)
            })
            .collect();

        let (x_off, z_off) = cam_pos;
        BLOCKS.with(|blocks| {
            FLUIDS.with(|fluids| {
                let blocks = blocks.borrow();
                let fluids = fluids.borrow();
                let mut img = image::RgbaImage::new(size.0 as u32, size.1 as u32);
                for (pos, entry) in entries {
                    let x_start = (pos.x << CHUNK_BITS) - x_off;
                    let z_start = (pos.y << CHUNK_BITS) - z_off;
                    if !(0..size.0).contains(&x_start) || !(0..size.1).contains(&z_start) {
                        continue;
                    }
                    for (xo, b) in entry.tile.iter().enumerate() {
                        for (zo, b) in b.iter().enumerate() {
                            if *b == 0 {
                                continue;
                            }

                            let block = if *b >= 240 {
                                &fluids[*b as usize - 240]
                            } else {
                                &blocks[*b as usize]
                            };
                            let color = block.colors()[0];
                            let color = [color.r, color.g, color.b, color.a];
                            let x = x_start as u32 + xo as u32;
                            let z = z_start as u32 + zo as u32;
                            img.put_pixel(x, z, color.into());
                        }
                    }
                }
                img.into()
            })
        })
    }

    fn map_off(cam_pos: Vec3, size: (i32, i32)) -> (i32, i32) {
        let x_off = (cam_pos.x as i32 & !CHUNK_MASK) - size.0 / 2;
        let z_off = (cam_pos.z as i32 & !CHUNK_MASK) - size.1 / 2;
        (x_off, z_off)
    }

    fn gen_texture(&mut self, fe: &ClientState, game: &GameState) {
        let player_pos = game.player().pos();
        let map_off = Self::map_off(player_pos, MAP_SIZE);
        let cur_pos: IVec2 = map_off.into();
        if let Some((pos, _)) = &self.cache {
            if &cur_pos == pos {
                return;
            }
        }
        let img = self.render_image(game, map_off, MAP_SIZE);
        let tex = Texture::from_image(&fe.display, img).unwrap();
        self.cache = Some((cur_pos, tex));
    }

    pub fn draw(
        &mut self,
        frame: &mut glium::Frame,
        fe: &ClientState,
        game: &GameState,
        projection: &Mat4,
    ) -> Result<()> {
        if !fe.show_debug_info() {
            self.clear();
            return Ok(());
        }
        self.gen_texture(fe, game);
        let mut mesh = TextMesh::new(&fe.display).unwrap();
        let (window_width, _window_height) = fe.window_size();
        let pos = (window_width as i16 - 512, 0, 512, 512);
        let uv = (0, 0, 128, 128);
        let rgba: [u8; 4] = [255, 255, 255, 255];
        mesh.push_box(pos, uv, rgba);
        mesh.prepare(&fe.display);
        let tex = &self.cache.as_ref().unwrap().1;
        let cur_tex = tex.texture_nn();
        let mat_mvp = projection.to_cols_array_2d();

        frame.draw(
            mesh.buffer(),
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &fe.shaders.text,
            &uniform! {
                mat_mvp: mat_mvp,
                cur_tex: cur_tex,
            },
            &glium::DrawParameters {
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
        )?;

        {
            let player_pos = game.player().pos();
            let (x_off, z_off) = Self::map_off(player_pos, MAP_SIZE);
            let x = player_pos.x - x_off as f32;
            let z = player_pos.z - z_off as f32;
            let mut mesh = TextMesh::new(&fe.display).unwrap();
            mesh.push_glyph(pos.0 + x as i16 - 8, pos.1 + z as i16 - 8, 2, rgba, '@');
            mesh.prepare(&fe.display);
            let cur_tex = fe.textures.gui.texture_nn();

            frame.draw(
                mesh.buffer(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &fe.shaders.text,
                &uniform! {
                    mat_mvp: mat_mvp,
                    cur_tex: cur_tex,
                },
                &glium::DrawParameters {
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    blend: glium::draw_parameters::Blend {
                        color: glium::draw_parameters::BlendingFunction::Addition {
                            source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                            destination:
                                glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        alpha: glium::draw_parameters::BlendingFunction::Addition {
                            source: glium::draw_parameters::LinearBlendingFactor::One,
                            destination:
                                glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0),
                    },
                    ..Default::default()
                },
            )?;
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.cache = None;
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    args.reactor.add_sink(
        Message::ResetEverything,
        Box::new(|_: &Reactor<Message>, _msg: Message| {
            MAP_CACHE.with(|map| {
                map.borrow_mut().clear();
            });
        }),
    );

    if std::env::args().any(|a| a == "render-map") {
        args.render_reactor
            .pre_world_render
            .push(Box::new(|args: RenderPassArgs| {
                MAP_CACHE.with(|map| {
                    let _ = std::fs::remove_file("map.png");
                    {
                        let size: i32 = RENDER_SIZE / 2 / CHUNK_SIZE as i32;
                        let mut world = args.game.world_mut();
                        for x in -size..size {
                            for z in -size..size {
                                for y in -2..2 {
                                    let pos = IVec3::new(x, y, z);
                                    world.generate(args.reactor, pos);
                                }
                            }
                        }
                    }
                    let img = map.borrow_mut().render_image(
                        args.game,
                        (-RENDER_SIZE / 2, -RENDER_SIZE / 2),
                        (RENDER_SIZE, RENDER_SIZE),
                    );
                    img.save("map.png").unwrap();
                    std::process::exit(0);
                });
                args
            }));
    }

    args.render_reactor
        .hud_2d_render
        .push(Box::new(|args: RenderPassArgs| {
            MAP_CACHE.with(|map| {
                let _ = map
                    .borrow_mut()
                    .draw(args.frame, args.fe, args.game, &args.projection);
            });
            args
        }));

    args
}
