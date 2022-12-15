// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::{defun, WORLD};
use glam::IVec3;
use v8::{ContextScope, HandleScope, Local, ObjectTemplate};

fn fun_get_block(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let x = args.get(0).int32_value(scope);
    let y = args.get(1).int32_value(scope);
    let z = args.get(2).int32_value(scope);
    if let (Some(x), Some(y), Some(z)) = (x, y, z) {
        let pos = IVec3::new(x, y, z);
        WORLD.with(|world| {
            let world = &*world.borrow();
            let world = world.as_ref().unwrap();
            let world = world.borrow_mut().get_block(pos);
            if let Some(b) = world {
                retval.set_int32(b as i32);
            } else {
                retval.set_null();
            }
        });
    }
}

fn fun_set_block(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let x = args.get(0).int32_value(scope);
    let y = args.get(1).int32_value(scope);
    let z = args.get(2).int32_value(scope);
    let b = args.get(3).int32_value(scope);
    if let (Some(x), Some(y), Some(z), Some(block)) = (x, y, z, b) {
        let pos = IVec3::new(x, y, z);
        WORLD.with(|world| {
            let world = &*world.borrow();
            let world = world.as_ref().unwrap();
            world.borrow_mut().set_block(pos, block as u8);
        });
    }
}

pub fn init(scope: &mut ContextScope<HandleScope>, obj: &Local<ObjectTemplate>) {
    defun(scope, obj, "getBlock", fun_get_block);
    defun(scope, obj, "setBlock", fun_set_block);
}
