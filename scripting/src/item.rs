// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use crate::defun;
use v8::{ContextScope, HandleScope, Local, ObjectTemplate};
use wolkenwelten_core::ScriptedItemList;

fn fun_item_get_icon(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        if let Some(icon) = ScriptedItemList::get_icon(id as u32) {
            retval.set_uint32(icon.into());
            return;
        }
    }
    retval.set_undefined();
}

fn fun_item_get_mesh(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        if let Some(icon) = ScriptedItemList::get_mesh(id as u32) {
            retval.set_uint32(icon.into());
            return;
        }
    }
    retval.set_undefined();
}

fn fun_item_get_amount(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        if let Some(icon) = ScriptedItemList::get_amount(id as u32) {
            retval.set_uint32(icon.into());
            return;
        }
    }
    retval.set_undefined();
}

fn fun_item_set_icon(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        let id = id as u32;
        if let Some(icon) = args.get(1).number_value(scope) {
            ScriptedItemList::set_icon(id, icon as u16);
        }
    }
}

fn fun_item_set_mesh(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        let id = id as u32;
        if let Some(mesh) = args.get(1).number_value(scope) {
            ScriptedItemList::set_mesh(id, mesh as u16);
        }
    }
}

fn fun_item_set_amount(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    if let Some(id) = args.get(0).number_value(scope) {
        let id = id as u32;
        if let Some(amount) = args.get(1).number_value(scope) {
            ScriptedItemList::set_amount(id, amount as u16);
        }
    }
}

pub fn use_item(
    id: u32,
    action: u8,
) -> bool {
    println!("use_item({}, {})", id, action);
    true
}

pub fn init(scope: &mut ContextScope<HandleScope>, obj: &Local<ObjectTemplate>) {
    defun(scope, obj, "itemGetIcon", fun_item_get_icon);
    defun(scope, obj, "itemGetMesh", fun_item_get_mesh);
    defun(scope, obj, "itemGetAmount", fun_item_get_amount);

    defun(scope, obj, "itemSetIcon", fun_item_set_icon);
    defun(scope, obj, "itemSetMesh", fun_item_set_mesh);
    defun(scope, obj, "itemSetAmount", fun_item_set_amount);
}

