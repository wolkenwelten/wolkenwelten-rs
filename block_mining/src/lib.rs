// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{IVec3, Mat4, Vec2};
use glium::{draw_parameters::PolygonOffset, uniform, Surface};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wolkenwelten_client::RenderInitArgs;
use wolkenwelten_client::{ClientState, Mesh, MeshVertex, Texture};
use wolkenwelten_common::{Message, Reactor};

#[derive(Clone, Copy, Debug, Default)]
struct BlockMining {
    pub _block: u8,
    pub damage: u16,
    pub block_health: u16,
}

#[derive(Debug, Default)]
struct BlockMiningMap {
    map: HashMap<IVec3, BlockMining>,
}

impl BlockMiningMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.map.retain(|_, p| {
            p.damage -= 1;
            p.damage > 0
        });
    }

    pub fn mine(&mut self, pos: IVec3, block: u8, dmg: u16, block_health: u16) -> bool {
        if let Some(m) = self.map.get_mut(&pos) {
            m.damage += dmg;
            if m.damage > m.block_health {
                m.damage = 1;
                return true;
            }
        } else {
            self.map.insert(
                pos,
                BlockMining {
                    damage: 2,
                    _block: block,
                    block_health,
                },
            );
        }
        false
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<IVec3, BlockMining> {
        self.map.iter()
    }
}

fn block_mining_draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    mining: &BlockMiningMap,
    mvp: &Mat4,
    mining_texture: &Texture,
) -> Result<()> {
    let mut vertices: Vec<MeshVertex> = vec![];
    let tex_scale = Vec2::new(1.0 / 8.0, -1.0);
    let step = 1.0 / 8.0;
    for (&pos, &m) in mining.iter() {
        let pos_off = pos.as_vec3();
        let i = ((m.damage as f32 / m.block_health as f32) * 8.0).floor();
        let tex_off = Vec2::new(i * step, 1.0);
        Mesh::add_block(&mut vertices, pos_off, tex_off, tex_scale);
    }
    let m = Mesh::from_vec(&fe.display, &vertices)?;
    let in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let mat_mvp = mvp.to_cols_array_2d();
    let cur_tex = mining_texture.texture_nn();

    frame.draw(
        m.buffer(),
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &fe.shaders.mesh,
        &uniform! {
            mat_mvp: mat_mvp,
            in_color: in_color,
            cur_tex: cur_tex,
        },
        &glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend {
                color: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::SourceAlpha,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::draw_parameters::BlendingFunction::Addition {
                    source: glium::draw_parameters::LinearBlendingFactor::One,
                    destination: glium::draw_parameters::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            depth: glium::draw_parameters::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                ..Default::default()
            },
            polygon_offset: PolygonOffset {
                factor: -8.0,
                units: -8.0,
                fill: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )?;
    Ok(())
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    let mining: Rc<RefCell<BlockMiningMap>> = Rc::new(RefCell::new(BlockMiningMap::new()));
    {
        let mining = mining.clone();
        let f = move |_reactor: &Reactor<Message>, _msg: Message| {
            mining.borrow_mut().tick();
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let player = args.game.player_ref();
        let world = args.game.world_ref();
        let mining = mining.clone();
        let f = move |reactor: &Reactor<Message>, msg: Message| {
            if let Message::GameTick { ticks } = msg {
                let player = player.borrow();
                if let Some((pos, block)) = player.mining() {
                    let mut world = world.borrow_mut();
                    let blocks = world.blocks.clone();
                    let bt = blocks.borrow();
                    if let Some(bt) = bt.get(block as usize) {
                        let mut mining = mining.borrow_mut();
                        if mining.mine(pos, block, 2, bt.block_health()) {
                            world.set_block(pos, 0);
                            reactor.defer(Message::BlockBreak { pos, block });
                        }
                    }
                    if (ticks & 0x7F) == 0 {
                        reactor.defer(Message::BlockMine { pos, block });
                    }
                }
            }
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let mining_texture = Texture::from_bytes(
            &args.fe.display,
            include_bytes!("../../assets/textures/mining.png"),
        )
        .expect("Couldn't load block mining texture");
        args.render_reactor
            .post_world_render
            .push(Box::new(move |args| {
                let mvp = args.projection * args.view;
                let _ =
                    block_mining_draw(args.frame, args.fe, &mining.borrow(), &mvp, &mining_texture);
                args
            }));
    }
    args
}
