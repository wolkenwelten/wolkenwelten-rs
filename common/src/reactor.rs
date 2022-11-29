// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::mem::Discriminant;

type ReactorHandler<T> = Box<dyn Fn(&Reactor<T>, T)>;
type ReactorHandlerMap<T> = HashMap<Discriminant<T>, Vec<ReactorHandler<T>>>;

pub struct Reactor<T> {
    handler: ReactorHandlerMap<T>,
    defer_queue: RefCell<Vec<T>>,
    defer_active: RefCell<bool>,
}

impl<T> Default for Reactor<T>
where
    T: Clone + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Reactor<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            handler: HashMap::new(),
            defer_queue: RefCell::new(vec![]),
            defer_active: RefCell::new(false),
        }
    }

    fn dispatch_raw(&self, msg: T) {
        if let Some(handler) = self.handler.get(&mem::discriminant(&msg)) {
            handler.iter().for_each(|f| f(self, msg));
        }
    }

    fn dispatch_defer(&self, msg: T) {
        self.defer_active.replace(true);
        self.dispatch(msg);
        loop {
            let q = {
                let mut q = self.defer_queue.borrow_mut();
                if q.len() == 0 {
                    break;
                }
                let r = q.clone();
                q.clear();
                r
            };
            q.iter().for_each(|m| self.dispatch_raw(*m));
        }
        self.defer_active.replace(false);
    }

    #[inline]
    pub fn dispatch(&self, msg: T) {
        let defer_active = *self.defer_active.borrow();
        if defer_active {
            self.dispatch_raw(msg)
        } else {
            self.dispatch_defer(msg)
        }
    }

    #[inline]
    pub fn defer(&self, msg: T) {
        let defer_active = *self.defer_active.borrow();
        if defer_active {
            self.defer_queue.borrow_mut().push(msg)
        } else {
            // If we are not currently dispatching a msg then we can safely dispatch the message immediatly
            self.dispatch(msg)
        }
    }

    pub fn add_sink(&mut self, msg: T, f: ReactorHandler<T>) {
        if let Some(handler) = self.handler.get_mut(&mem::discriminant(&msg)) {
            handler.push(f);
        } else {
            let handler: Vec<ReactorHandler<T>> = vec![f];
            self.handler.insert(mem::discriminant(&msg), handler);
        }
    }
}
