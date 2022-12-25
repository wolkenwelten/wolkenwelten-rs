// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use v8::{ContextScope, HandleScope};
use wolkenwelten_client::{start_client, RenderInit};
use wolkenwelten_core::{Chungus, GameState, Message, Reactor};

mod io;
mod world;

thread_local! {
    pub static WORLD: RefCell<Option<Rc<RefCell<Chungus>>>> = RefCell::new(None);
    static MSG_QUEUE: RefCell<Vec<Message>> = RefCell::new(vec![]);
    static JS_INIT_VEC: RefCell<Vec<String>> = RefCell::new(vec![]);
}

pub fn push_init_code(code: &str) {
    JS_INIT_VEC.with(|v| {
        v.borrow_mut().push(code.to_string());
    })
}

fn eval(scope: &mut ContextScope<HandleScope>, source: &str) {
    let code = v8::String::new(scope, source).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let _result = script.run(scope).unwrap();
}

pub fn defun(
    scope: &mut ContextScope<HandleScope>,
    obj: &v8::Local<v8::ObjectTemplate>,
    key: &str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
) {
    let key = v8::String::new(scope, key).unwrap();
    let value = v8::FunctionTemplate::new(scope, callback);
    obj.set(key.into(), value.into());
}

pub fn add_string(
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
            io::init(&mut scope.borrow_mut(), &wwc);
            world::init(&mut scope.borrow_mut(), &wwc);
            {
                let key = v8::String::new(&mut scope.borrow_mut(), "WWC").unwrap();
                let global = context.global(&mut scope.borrow_mut());
                let wwc = wwc.new_instance(&mut scope.borrow_mut()).unwrap();
                global.set(&mut scope.borrow_mut(), key.into(), wwc.into());
            }
        }

        JS_INIT_VEC.with(|v| {
            let scope = &mut scope.borrow_mut();
            v.borrow_mut().drain(0..).for_each(|source| {
                eval(scope, &source);
            });
        });

        start_client(game_state, reactor, render_init_fun);
    }
}
