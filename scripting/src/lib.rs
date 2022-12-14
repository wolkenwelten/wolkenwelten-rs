// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glam::{IVec3, Vec3};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use v8::{ContextScope, HandleScope};
use wolkenwelten_client::{start_client, RenderInit};
use wolkenwelten_core::{Chungus, GameState, Message, Reactor, SfxId, GAME_LOG};

thread_local! {
    static WORLD: RefCell<Option<Rc<RefCell<Chungus>>>> = RefCell::new(None);
    static MSG_QUEUE: RefCell<Vec<Message>> = RefCell::new(vec![]);
}

fn eval(scope: &mut ContextScope<HandleScope>, source: &str) {
    let code = v8::String::new(scope, source).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let _result = script.run(scope).unwrap();
}

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

fn add_fun(
    scope: &mut ContextScope<HandleScope>,
    obj: &v8::Local<v8::ObjectTemplate>,
    key: &str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
) {
    let key = v8::String::new(scope, key).unwrap();
    let value = v8::FunctionTemplate::new(scope, callback);
    obj.set(key.into(), value.into());
}

fn add_string(
    scope: &mut ContextScope<HandleScope>,
    obj: &v8::Local<v8::ObjectTemplate>,
    key: &str,
    val: &str,
) {
    let key = v8::String::new(scope, key).unwrap();
    let value = v8::String::new(scope, val).unwrap();
    obj.set(key.into(), value.into());
}

pub fn start_runtime(
    game_state: GameState,
    mut reactor: Reactor<Message>,
    render_init_fun: Vec<RenderInit>,
) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let world = game_state.world_rc();
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
        eval(
            &mut scope.borrow_mut(),
            include_str!("../target/dist/stdlib.js"),
        );

        {
            let clock = Instant::now();
            let scope = scope.clone();
            let f = move |reactor: &Reactor<Message>, _msg: Message| {
                let mut log = reactor.log_mut();
                let msgs: Vec<Message> = log.iter().copied().collect();
                let json = serde_json::to_string(&msgs);
                log.clear();
                if let Ok(json) = json {
                    let millis = clock.elapsed().as_millis() as u64;
                    let code = format!("WolkenWelten.tick({}, {});", millis, json);
                    eval(&mut scope.borrow_mut(), code.as_str());
                }
                MSG_QUEUE.with(move |f| {
                    f.borrow_mut().drain(0..).for_each(|m| reactor.defer(m));
                });
            };
            reactor.add_sink(Message::GameTick { ticks: 0 }, Box::new(f));
        }

        {
            let wwc = v8::ObjectTemplate::new(&mut scope.borrow_mut());
            add_string(
                &mut scope.borrow_mut(),
                &wwc,
                "VERSION",
                env!("CARGO_PKG_VERSION"),
            );
            add_fun(&mut scope.borrow_mut(), &wwc, "eprint", fun_eprint);
            add_fun(&mut scope.borrow_mut(), &wwc, "print", fun_print);
            add_fun(&mut scope.borrow_mut(), &wwc, "getBlock", fun_get_block);
            add_fun(&mut scope.borrow_mut(), &wwc, "setBlock", fun_set_block);
            add_fun(&mut scope.borrow_mut(), &wwc, "sfxPlay", fun_sfx_play);
            add_fun(&mut scope.borrow_mut(), &wwc, "game_log", fun_log);
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
        start_client(game_state, reactor, render_init_fun);
    }
}
