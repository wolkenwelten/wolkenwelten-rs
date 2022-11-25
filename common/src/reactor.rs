// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::mem::Discriminant;

pub struct Reactor<T> {
    handler: HashMap<Discriminant<T>, Vec<Box<dyn Fn(&Self, T)>>>,
    defer_queue: RefCell<Vec<T>>,
    defer_active: RefCell<bool>,
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
            return self.dispatch_raw(msg);
        } else {
            return self.dispatch_defer(msg);
        }
    }

    #[inline]
    pub fn defer(&self, msg: T) {
        let defer_active = *self.defer_active.borrow();
        if defer_active {
            self.defer_queue.borrow_mut().push(msg);
        } else {
            // If we are not currently dispatching a msg then we can safely dispatch the message immediatly
            return self.dispatch(msg);
        }
    }

    pub fn add_sink(&mut self, msg: T, f: Box<dyn Fn(&Self, T)>) {
        if let Some(handler) = self.handler.get_mut(&mem::discriminant(&msg)) {
            handler.push(f);
        } else {
            let mut handler: Vec<Box<dyn Fn(&Self, T)>> = vec![];
            handler.push(f);
            self.handler.insert(mem::discriminant(&msg), handler);
        }
    }
}
