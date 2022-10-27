/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
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
            .execute_script("<init>", include_str!("../../modules/preamble.js"))
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
