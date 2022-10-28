use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::{path::Path, sync::Arc};

use anyhow::Context;
use swc::{
    self,
    config::{Config, JscConfig, Options},
    try_with_handler, BoolConfig, HandlerOpts,
};
use swc_common::{Globals, SourceMap, GLOBALS};
use swc_ecma_parser::{Syntax, TsConfig};

fn main() {
    let globals = Globals::new();
    GLOBALS.set(&globals, || {
        let cm = Arc::<SourceMap>::default();

        let c = swc::Compiler::new(cm.clone());

        let output = try_with_handler(
            cm.clone(),
            HandlerOpts {
                ..Default::default()
            },
            |handler| {
                println!("cargo:rerun-if-changed=../modules/");
                let fm = cm
                    .load_file(Path::new("../modules/main.ts"))
                    .expect("failed to load file");

                let opts: Options = Options {
                    config: Config {
                        jsc: JscConfig {
                            syntax: Some(Syntax::Typescript(TsConfig::default())),
                            ..Default::default()
                        },
                        minify: BoolConfig::new(Some(true)),
                        ..Default::default()
                    },
                    ..Options::default()
                };

                c.process_js_file(fm, handler, &opts)
                    .context("failed to minify")
            },
        )
        .unwrap();

        fs::create_dir_all("../target/dist/").unwrap();
        let mut fh = File::create("../target/dist/main.js").unwrap();
        fh.write_all(output.code.as_bytes()).unwrap();
    });
}
