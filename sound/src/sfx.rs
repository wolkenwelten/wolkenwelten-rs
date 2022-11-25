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
use wolkenwelten_common::{Item, Message, Reactor};

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

pub struct SfxList<'a> {
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
        let player_pos = state.player_pos / 8.0;
        let emitter_pos = emitter_pos / 8.0;
        let left = player_pos + (Self::planar_direction(state.player_rot.x - 90.0) * 0.3);
        let right = player_pos + (Self::planar_direction(state.player_rot.x + 90.0) * 0.3);
        let emitter = emitter_pos.into();
        let sink = SpatialSink::try_new(stream_handle, emitter, left.into(), right.into()).unwrap();
        sink.set_volume(volume);
        sink.append(source);
        sink.detach();
    }

    /// This function creates a new closure that emits a specific sound whenever
    /// a particular message is emitted.
    fn add_fun(&mut self, sfx: Rc<RefCell<Sfx>>, msg: Message, volume: f32) {
        let state = self.state.clone();
        let stream = self.stream.clone();
        let f = move |_: &Reactor<Message>, msg: Message| {
            Self::play_spatial_sound(
                &stream.borrow().1,
                &sfx.borrow(),
                volume,
                &state.borrow(),
                msg.pos().unwrap_or_default(),
            )
        };
        self.reactor.add_sink(msg, Box::new(f));
    }

    fn sfx_new(bytes: &'static [u8]) -> Rc<RefCell<Sfx>> {
        Rc::new(RefCell::new(Sfx::from_bytes(bytes)))
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
                if let Message::CharacterPosRotVel(pos, _vel, rot) = msg {
                    let mut state = state.borrow_mut();
                    state.player_pos = pos;
                    state.player_rot = rot;
                }
            };
            s.reactor.add_sink(
                Message::CharacterPosRotVel(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
                Box::new(f),
            );
        }
        let jump = Self::sfx_new(include_bytes!("../../assets/sfx/jump.ogg"));
        let hook_fire = Self::sfx_new(include_bytes!("../../assets/sfx/hookFire.ogg"));
        let ungh = Self::sfx_new(include_bytes!("../../assets/sfx/ungh.ogg"));
        let step = Self::sfx_new(include_bytes!("../../assets/sfx/step.ogg"));
        let stomp = Self::sfx_new(include_bytes!("../../assets/sfx/stomp.ogg"));
        let bomb = Self::sfx_new(include_bytes!("../../assets/sfx/bomb.ogg"));
        let pock = Self::sfx_new(include_bytes!("../../assets/sfx/pock.ogg"));
        let tock = Self::sfx_new(include_bytes!("../../assets/sfx/tock.ogg"));

        s.add_fun(pock.clone(), Message::BlockPlace(IVec3::ZERO, 0), 0.3);
        s.add_fun(jump.clone(), Message::CharacterJump(Vec3::ZERO), 0.1);
        s.add_fun(hook_fire.clone(), Message::CharacterShoot(Vec3::ZERO), 0.4);
        s.add_fun(ungh.clone(), Message::CharacterDamage(Vec3::ZERO, 0), 0.3);
        s.add_fun(ungh.clone(), Message::CharacterDeath(Vec3::ZERO), 0.3);
        s.add_fun(step.clone(), Message::CharacterStep(Vec3::ZERO), 0.2);
        s.add_fun(stomp.clone(), Message::CharacterStomp(Vec3::ZERO), 0.2);
        s.add_fun(bomb.clone(), Message::EntityCollision(Vec3::ZERO), 0.2);
        s.add_fun(pock.clone(), Message::BlockPlace(IVec3::ZERO, 0), 0.3);
        s.add_fun(
            pock.clone(),
            Message::ItemDropPickup(Vec3::ZERO, Item::None),
            0.1,
        );
        s.add_fun(tock.clone(), Message::BlockBreak(IVec3::ZERO, 0), 0.3);
        s.add_fun(tock.clone(), Message::BlockMine(IVec3::ZERO, 0), 0.1);
    }
}
