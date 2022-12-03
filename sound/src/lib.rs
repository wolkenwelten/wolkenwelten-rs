// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use core::f32::consts::PI;
use glam::{IVec3, Vec3};
use rodio::{
    buffer::SamplesBuffer, source::Buffered, Decoder, OutputStream, OutputStreamHandle, Source,
    SpatialSink,
};
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;
use wolkenwelten_client::RenderInitArgs;
use wolkenwelten_core::{Item, Message, Reactor, SfxId};

struct Sfx {
    buf: Buffered<SamplesBuffer<i16>>,
}

impl Sfx {
    pub fn from_bytes(bytes: &'static [u8]) -> Self {
        let dec = Decoder::new(Cursor::new(bytes)).unwrap();
        let sample_rate = dec.sample_rate();
        let channels = dec.channels();
        let buf: Vec<i16> = dec.collect();
        let buf = SamplesBuffer::new(channels, sample_rate, buf).buffered();
        Self { buf }
    }

    pub fn get_buf(&self) -> Buffered<SamplesBuffer<i16>> {
        self.buf.clone()
    }
}

#[derive(Default, Debug)]
struct State {
    pub player_pos: glam::Vec3,
    pub player_rot: glam::Vec3,
}

struct SfxList<'a> {
    stream: Rc<RefCell<(OutputStream, OutputStreamHandle)>>,
    state: Rc<RefCell<State>>,
    reactor: &'a mut Reactor<Message>,
}

impl<'a> SfxList<'a> {
    fn planar_direction(deg: f32) -> Vec3 {
        Vec3::new(
            ((deg - 90.0) * PI / 180.0).cos(),
            0.0,
            ((deg - 90.0) * PI / 180.0).sin(),
        )
    }

    /// Low level function that does the actual playing of sound
    fn play_spatial_sound(
        stream_handle: &OutputStreamHandle,
        sfx: &Sfx,
        volume: f32,
        state: &State,
        emitter_pos: Vec3,
    ) {
        let source = sfx.get_buf();
        let player_pos = state.player_pos / 16.0;
        let emitter_pos = emitter_pos / 16.0;
        let left = player_pos + (Self::planar_direction(state.player_rot.x - 90.0) * 0.3);
        let right = player_pos + (Self::planar_direction(state.player_rot.x + 90.0) * 0.3);
        let emitter = emitter_pos.into();
        let sink = SpatialSink::try_new(stream_handle, emitter, left.into(), right.into()).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }

    fn add_relay(&mut self, sfx: SfxId, msg: Message, volume: f32) {
        self.reactor.add_sink(
            msg,
            Box::new(move |reactor: &Reactor<Message>, msg: Message| {
                reactor.dispatch(Message::SfxPlay {
                    pos: msg.pos().unwrap_or_default(),
                    volume,
                    sfx,
                });
            }),
        );
    }

    fn sfx_new(bytes: &'static [u8]) -> RefCell<Sfx> {
        RefCell::new(Sfx::from_bytes(bytes))
    }

    pub fn add_handler(reactor: &'a mut Reactor<Message>) {
        // First we set up our struct so that we can more conveniently pass all the arguments necessary
        let mut s = {
            let stream = rodio::OutputStream::try_default().unwrap();
            let state = Default::default();
            Self {
                stream: Rc::new(RefCell::new(stream)),
                state: Rc::new(RefCell::new(state)),
                reactor,
            }
        };

        // This handler is a bit special in that it caches the players position that
        // the game is regularly emitting, that way we have the last known position of
        // the player handy.
        {
            let state = s.state.clone();
            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::CharacterPosRotVel { pos, rot, .. } = msg {
                    let mut state = state.borrow_mut();
                    state.player_pos = pos;
                    state.player_rot = rot;
                }
            };
            s.reactor.add_sink(
                Message::CharacterPosRotVel {
                    pos: Vec3::ZERO,
                    rot: Vec3::ZERO,
                    vel: Vec3::ZERO,
                },
                Box::new(f),
            );
        }

        s.add_relay(SfxId::Jump, Message::CharacterJump { pos: Vec3::ZERO }, 0.1);
        s.add_relay(
            SfxId::Pock,
            Message::BlockPlace {
                pos: IVec3::ZERO,
                block: 0,
            },
            0.3,
        );
        s.add_relay(
            SfxId::HookFire,
            Message::CharacterShoot { pos: Vec3::ZERO },
            0.4,
        );
        s.add_relay(
            SfxId::Ungh,
            Message::CharacterDamage {
                pos: Vec3::ZERO,
                damage: 0,
            },
            0.3,
        );
        s.add_relay(
            SfxId::Ungh,
            Message::CharacterDeath { pos: Vec3::ZERO },
            0.3,
        );
        s.add_relay(SfxId::Step, Message::CharacterStep { pos: Vec3::ZERO }, 0.2);
        s.add_relay(
            SfxId::Stomp,
            Message::CharacterStomp { pos: Vec3::ZERO },
            0.2,
        );
        s.add_relay(
            SfxId::Bomb,
            Message::Explosion {
                pos: Vec3::ZERO,
                power: 0.0,
            },
            1.0,
        );
        s.add_relay(
            SfxId::Pock,
            Message::BlockPlace {
                pos: IVec3::ZERO,
                block: 0,
            },
            0.3,
        );
        s.add_relay(
            SfxId::Pock,
            Message::ItemDropPickup {
                pos: Vec3::ZERO,
                item: Item::None,
            },
            0.1,
        );
        s.add_relay(
            SfxId::Tock,
            Message::BlockBreak {
                pos: IVec3::ZERO,
                block: 0,
            },
            0.3,
        );
        s.add_relay(
            SfxId::Tock,
            Message::BlockMine {
                pos: IVec3::ZERO,
                block: 0,
            },
            0.1,
        );

        {
            let state = s.state.clone();
            let stream = s.stream.clone();
            let jump = Self::sfx_new(include_bytes!("../../assets/sfx/jump.ogg"));
            let hook_fire = Self::sfx_new(include_bytes!("../../assets/sfx/hookFire.ogg"));
            let ungh = Self::sfx_new(include_bytes!("../../assets/sfx/ungh.ogg"));
            let step = Self::sfx_new(include_bytes!("../../assets/sfx/step.ogg"));
            let stomp = Self::sfx_new(include_bytes!("../../assets/sfx/stomp.ogg"));
            let bomb = Self::sfx_new(include_bytes!("../../assets/sfx/bomb.ogg"));
            let pock = Self::sfx_new(include_bytes!("../../assets/sfx/pock.ogg"));
            let tock = Self::sfx_new(include_bytes!("../../assets/sfx/tock.ogg"));

            let f = move |_: &Reactor<Message>, msg: Message| {
                if let Message::SfxPlay {
                    pos: emitter_pos,
                    volume,
                    sfx,
                } = msg
                {
                    let sfx = match sfx {
                        SfxId::Jump => jump.borrow(),
                        SfxId::HookFire => hook_fire.borrow(),
                        SfxId::Ungh => ungh.borrow(),
                        SfxId::Step => step.borrow(),
                        SfxId::Stomp => stomp.borrow(),
                        SfxId::Bomb => bomb.borrow(),
                        SfxId::Pock => pock.borrow(),
                        SfxId::Tock => tock.borrow(),
                        _ => return,
                    };
                    Self::play_spatial_sound(
                        &stream.borrow().1,
                        &sfx,
                        volume,
                        &state.borrow(),
                        emitter_pos,
                    )
                }
            };
            s.reactor.add_sink(
                Message::SfxPlay {
                    pos: Vec3::ZERO,
                    volume: 0.0,
                    sfx: SfxId::Jump,
                },
                Box::new(f),
            );
        }
    }
}

pub fn init(args: RenderInitArgs) -> RenderInitArgs {
    SfxList::add_handler(args.reactor);
    args
}
