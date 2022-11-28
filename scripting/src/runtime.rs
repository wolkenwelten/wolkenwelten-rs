// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::IVec3;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use v8::{ContextScope, HandleScope};
use wolkenwelten_client_winit::start_app;
use wolkenwelten_common::{Message, Reactor};
use wolkenwelten_game::{Chungus, GameState};

thread_local! {
    static WORLD: RefCell<Option<Rc<RefCell<Chungus>>>> = RefCell::new(None);
}

fn eval(scope: &mut ContextScope<HandleScope>, source: &str) {
    let code = v8::String::new(scope, source).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let _result = script.run(scope).unwrap();
}

#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type
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

fn fun_get_block(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    WORLD.with(|world| {
        let world = &*world.borrow();
        let world = world.as_ref().unwrap();
        let x = args.get(0).int32_value(scope);
        let y = args.get(1).int32_value(scope);
        let z = args.get(2).int32_value(scope);
        match (x, y, z) {
            (Some(x), Some(y), Some(z)) => {
                let pos = IVec3::new(x, y, z);
                let world = world.borrow_mut().get_block(pos);
                if let Some(b) = world {
                    retval.set_int32(b as i32);
                    return;
                }
            }
            _ => (),
        }
        retval.set_null();
    });
}

fn fun_set_block(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    WORLD.with(|world| {
        let world = &*world.borrow();
        let world = world.as_ref().unwrap();
        let x = args.get(0).int32_value(scope);
        let y = args.get(1).int32_value(scope);
        let z = args.get(2).int32_value(scope);
        let b = args.get(3).int32_value(scope);
        match (x, y, z, b) {
            (Some(x), Some(y), Some(z), Some(block)) => {
                let pos = IVec3::new(x, y, z);
                world.borrow_mut().set_block(pos, block as u8);
            }
            _ => (),
        }
    });
}

pub fn start_runtime(game_state: GameState, mut reactor: Reactor<Message>) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let world = game_state.world_ref();
        WORLD.with(move |f| {
            f.replace(Some(world));
        });
    }

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    {
        // These unsafe blocks are necessary to trick the borrow checker, I really dislike this but it seems
        // that it is necessary because rusty_v8 mostly just exposes the C++ API
        let mut handle_scope =
            v8::HandleScope::new(unsafe { &mut *(&mut isolate as *mut v8::OwnedIsolate) });
        let context = v8::Context::new(&mut handle_scope);
        let scope = Rc::new(RefCell::new(v8::ContextScope::new(
            unsafe { &mut *(&mut handle_scope as *mut HandleScope<()>) },
            context,
        )));
        eval(&mut scope.borrow_mut(), include_str!("./preamble.js"));

        {
            let clock = Instant::now();
            let scope = scope.clone();
            let f = move |_reactor: &Reactor<Message>, _msg: Message| {
                let millis = clock.elapsed().as_millis() as u64;
                let code = format!("WolkenWelten.tick({});", millis);
                eval(&mut scope.borrow_mut(), code.as_str());
            };
            reactor.add_sink(Message::GameTick(0), Box::new(f));
        }

        {
            let wwc = v8::ObjectTemplate::new(&mut scope.borrow_mut());
            {
                let key = v8::String::new(&mut scope.borrow_mut(), "print").unwrap();
                let value = v8::FunctionTemplate::new(&mut scope.borrow_mut(), fun_print);
                wwc.set(key.into(), value.into());
            }
            {
                let key = v8::String::new(&mut scope.borrow_mut(), "get_block").unwrap();
                let value = v8::FunctionTemplate::new(&mut scope.borrow_mut(), fun_get_block);
                wwc.set(key.into(), value.into());
            }
            {
                let key = v8::String::new(&mut scope.borrow_mut(), "set_block").unwrap();
                let value = v8::FunctionTemplate::new(&mut scope.borrow_mut(), fun_set_block);
                wwc.set(key.into(), value.into());
            }
            {
                let key = v8::String::new(&mut scope.borrow_mut(), "WWC").unwrap();
                let global = context.global(&mut scope.borrow_mut());
                let wwc = wwc.new_instance(&mut scope.borrow_mut()).unwrap();
                global.set(&mut scope.borrow_mut(), key.into(), wwc.into());
            }
        }

        eval(
            &mut scope.borrow_mut(),
            include_str!("../../modules/main.js"),
        );
        start_app(game_state, reactor);
    }
}
