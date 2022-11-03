// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use deno_core::{op, Extension, JsRuntime, RuntimeOptions};

pub struct Runtime {
    runtime: JsRuntime,
}

#[op]
fn op_sum(nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    Ok(sum)
}

impl Default for Runtime {
    fn default() -> Self {
        let ext = Extension::builder().ops(vec![op_sum::decl()]).build();

        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });

        runtime
            .execute_script("<init>", include_str!("../../modules/main.js"))
            .unwrap();

        Self { runtime }
    }
}

impl Runtime {
    pub fn eval(&mut self, script_text: &str) {
        self.runtime.execute_script("<usage>", script_text).unwrap();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, millis: u64) {
        let _ = self.runtime.run_event_loop(false);
        let code = format!("WolkenWelten.tick({});", millis);
        self.runtime.execute_script("<usage>", &code).unwrap();
    }
}
