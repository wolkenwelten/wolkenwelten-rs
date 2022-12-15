// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;
use glam::{IVec3, Mat4, Vec3};
use std::cell::RefCell;
use wolkenwelten_client::{ClientState, RenderInitArgs, RenderPassArgs};
use wolkenwelten_core::{BlockItem, Chungus, Entity, Item, Message, Reactor};

thread_local! {
    pub static DROPS: RefCell<ItemDropList> = RefCell::new(ItemDropList::new());
}

const ITEM_DROP_PICKUP_RANGE: f32 = 1.5;

#[derive(Clone, Default, Debug)]
pub struct ItemDrop {
    item: Item,
    ent: Entity,
}

fn item_drop_draw(
    frame: &mut glium::Frame,
    fe: &ClientState,
    entity: &ItemDrop,
    view: &Mat4,
    projection: &Mat4,
) -> Result<()> {
    let rot = entity.rot();
    let pos = entity.pos();
    let model = Mat4::from_scale(Vec3::new(1.0 / 32.0, 1.0 / 32.0, 1.0 / 32.0));
    let model = Mat4::from_rotation_x(rot.x.to_radians()) * model;
    let model = Mat4::from_rotation_y(rot.y.to_radians()) * model;
    let model = Mat4::from_translation(pos) * model;
    let vp = projection.mul_mat4(view);
    let mvp = vp.mul_mat4(&model);

    match entity.item() {
        Item::Block(bi) => fe.meshes.blocks[bi.block as usize].draw(
            frame,
            &fe.textures.blocks_raw,
            &fe.shaders.mesh,
            &mvp,
        ),
        Item::None => Ok(()),
        /*
        _ => fe.meshes
            .bag
            .draw(frame, fe.block_indeces(), &fe.shaders.block, &mvp, 1.0)
        */
    }
}

impl ItemDrop {
    pub fn new(pos: Vec3, item: Item) -> Self {
        let mut ent = Entity::new();
        ent.set_pos(pos);
        Self { item, ent }
    }
    #[inline]
    pub fn pos(&self) -> Vec3 {
        self.ent.pos()
    }
    #[inline]
    pub fn rot(&self) -> Vec3 {
        self.ent.rot()
    }
    #[inline]
    pub fn item(&self) -> Item {
        self.item
    }

    #[inline]
    pub fn set_vel(&mut self, vel: Vec3) {
        self.ent.set_vel(vel);
    }

    #[inline]
    pub fn tick(&mut self, world: &Chungus) -> bool {
        self.ent.set_rot(self.ent.rot() + Vec3::new(0.0, 0.2, 0.0));
        self.ent.tick(world)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ItemDropList {
    drops: Vec<ItemDrop>,
}

impl ItemDropList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.drops.clear();
    }

    pub fn add_from_block_break(&mut self, pos: IVec3, block: u8) {
        let pos = pos.as_vec3() + Vec3::new(0.5, 0.5, 0.5);
        let item = BlockItem::new(block, 1).into();
        self.drops.push(ItemDrop::new(pos, item));
    }

    pub fn add(&mut self, pos: Vec3, vel: Vec3, item: Item) {
        let mut drop = ItemDrop::new(pos, item);
        drop.set_vel(vel);
        self.drops.push(drop);
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<ItemDrop> {
        self.drops.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<ItemDrop> {
        self.drops.iter_mut()
    }

    pub fn tick_all(&mut self, reactor: &Reactor<Message>, player_pos: Vec3, world: &Chungus) {
        self.drops.retain_mut(|d| {
            d.tick(world);
            let dist = d.pos() - player_pos;
            let dd = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;
            if dd > (256.0 * 256.0) {
                false
            } else if dd < ITEM_DROP_PICKUP_RANGE * ITEM_DROP_PICKUP_RANGE {
                reactor.dispatch(Message::ItemDropPickup {
                    pos: d.pos(),
                    item: d.item(),
                });
                false
            } else {
                true
            }
        });
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    {
        let player = args.game.player_rc();
        let world = args.game.world_rc();
        let f = move |reactor: &Reactor<Message>, _msg: Message| {
            let player_pos = player.borrow().pos();
            DROPS.with(|drops| {
                drops
                    .borrow_mut()
                    .tick_all(reactor, player_pos, &world.borrow());
            });
        };
        args.reactor
            .add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
    }
    {
        let f = move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::BlockBreak { pos, block } = msg {
                DROPS.with(|drops| drops.borrow_mut().add_from_block_break(pos, block));
            }
        };
        args.reactor.add_sink(
            Message::BlockBreak {
                pos: IVec3::ZERO,
                block: 0,
            },
            Box::new(f),
        );
    }
    {
        let f = move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::ItemDropNew { pos, item } = msg {
                DROPS.with(|drops| {
                    drops.borrow_mut().add(pos, Vec3::ZERO, item);
                });
            }
        };
        args.reactor.add_sink(
            Message::ItemDropNew {
                pos: Vec3::ZERO,
                item: Item::None,
            },
            Box::new(f),
        );
    }
    {
        let f = move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::Explosion { pos, power } = msg {
                let p = power * power;
                DROPS.with(|drops| {
                    drops
                        .borrow_mut()
                        .iter_mut()
                        .filter(|m| (pos - m.pos()).length_squared() < p)
                        .for_each(|m| {
                            let d = pos - m.pos();
                            let mut dir = d.normalize() * d * 0.2;
                            dir.y = -0.5;
                            m.set_vel(dir * -0.04);
                        });
                });
            }
        };
        args.reactor.add_sink(
            Message::Explosion {
                pos: Vec3::ZERO,
                power: 0.0,
            },
            Box::new(f),
        );
    }

    args.reactor.add_sink(
        Message::CharacterDropItem {
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            item: Item::None,
        },
        Box::new(move |_reactor: &Reactor<Message>, msg: Message| {
            if let Message::CharacterDropItem { pos, vel, item } = msg {
                DROPS.with(|drops| {
                    drops.borrow_mut().add(pos, vel, item);
                });
            }
        }),
    );

    args.render_reactor.entity_provider.push(Box::new(move |v| {
        DROPS.with(|drops| {
            for e in drops.borrow().iter() {
                v.push(e.ent.clone());
            }
        });
    }));

    args.reactor.add_sink(
        Message::ResetEverything,
        Box::new(move |_: &Reactor<Message>, _msg: Message| {
            DROPS.with(|drops| {
                drops.borrow_mut().clear();
            });
        }),
    );

    args.render_reactor
        .world_render
        .push(Box::new(move |args: RenderPassArgs| {
            DROPS.with(|drops| {
                for entity in drops.borrow().iter() {
                    let _ =
                        item_drop_draw(args.frame, args.fe, entity, &args.view, &args.projection);
                }
            });
            args
        }));

    args
}
