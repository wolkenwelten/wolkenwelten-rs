// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{defun, MSG_QUEUE};
use glam::Vec3;
use v8::{ContextScope, HandleScope, Local, ObjectTemplate};
use wolkenwelten_core::{Message, SfxId, GAME_LOG};

fn fun_log(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    GAME_LOG.with(|log| {
        log.borrow_mut().push(message);
    });
}

fn fun_print(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    print!("{}", message);
}

fn fun_eprint(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    eprint!("{}", message);
}

fn fun_sfx_play(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let x = args.get(0).number_value(scope);
    let y = args.get(1).number_value(scope);
    let z = args.get(2).number_value(scope);
    let volume = args.get(3).number_value(scope);
    let sfx = args.get(4).int32_value(scope);
    if let (Some(x), Some(y), Some(z), Some(volume), Some(sfx)) = (x, y, z, volume, sfx) {
        let pos = Vec3::new(x as f32, y as f32, z as f32);
        let sfx = match sfx {
            1 => SfxId::Jump,
            2 => SfxId::HookFire,
            3 => SfxId::Ungh,
            4 => SfxId::Step,
            5 => SfxId::Stomp,
            6 => SfxId::Bomb,
            7 => SfxId::Pock,
            8 => SfxId::Tock,
            _ => SfxId::Void,
        };
        let volume = volume as f32;
        let msg = Message::SfxPlay { pos, volume, sfx };
        MSG_QUEUE.with(|q| q.borrow_mut().push(msg));
    }
}

pub fn init(scope: &mut ContextScope<HandleScope>, obj: &Local<ObjectTemplate>) {
    defun(scope, obj, "eprint", fun_eprint);
    defun(scope, obj, "print", fun_print);
    defun(scope, obj, "sfxPlay", fun_sfx_play);
    defun(scope, obj, "game_log", fun_log);
}
