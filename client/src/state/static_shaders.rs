// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.

#[derive(Debug)]
pub struct ShaderList {
    pub block: glium::Program,
    pub mesh: glium::Program,
    pub text: glium::Program,
    pub particle: glium::Program,
}

const VERT_PREFIX: &str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
    r"#version 300 es
precision mediump float;
precision mediump int;
#line 0"
} else if cfg!(target_os = "macos") {
    r"#version 330 core
precision mediump float;
precision mediump int;
#line 0"
} else {
    r"#version 130
#line 0"
};

const FRAG_PREFIX: &str = if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
    r"#version 300 es
precision mediump float;
precision mediump int;
precision mediump sampler2DArray;
#line 0"
} else if cfg!(target_os = "macos") {
    r"#version 330 core
precision mediump float;
precision mediump int;
precision mediump sampler2DArray;
#line 0"
} else {
    r"#version 130
#line 0"
};

impl ShaderList {
    fn new_program(
        display: &glium::Display,
        vert: &str,
        frag: &str,
    ) -> Result<glium::Program, glium::ProgramCreationError> {
        let vert = format!("{}\n{}", VERT_PREFIX, vert);
        let frag = format!("{}\n{}", FRAG_PREFIX, frag);
        glium::Program::from_source(display, &vert, &frag, None)
    }

    fn new_point_program(
        display: &glium::Display,
        vert: &str,
        frag: &str,
    ) -> Result<glium::Program, glium::ProgramCreationError> {
        let vert = format!("{}\n{}", VERT_PREFIX, vert);
        let frag = format!("{}\n{}", FRAG_PREFIX, frag);
        glium::Program::new(
            display,
            glium::program::ProgramCreationInput::SourceCode {
                vertex_shader: &vert,
                fragment_shader: &frag,
                geometry_shader: None,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                transform_feedback_varyings: None,
                outputs_srgb: false,
                uses_point_size: !cfg!(any(target_arch = "aarch64", target_arch = "armv7")), // Work around the GLES bug
            },
        )
    }

    pub fn new(display: &glium::Display) -> Result<Self, glium::ProgramCreationError> {
        let mesh = Self::new_program(
            display,
            include_str!("../shaders/mesh.vert"),
            include_str!("../shaders/mesh.frag"),
        )?;
        let text = Self::new_program(
            display,
            include_str!("../shaders/text.vert"),
            include_str!("../shaders/text.frag"),
        )?;
        let block = Self::new_program(
            display,
            include_str!("../shaders/block.vert"),
            include_str!("../shaders/block.frag"),
        )?;
        let particle = Self::new_point_program(
            display,
            include_str!("../shaders/particle.vert"),
            include_str!("../shaders/particle.frag"),
        )?;

        Ok(Self {
            block,
            mesh,
            particle,
            text,
        })
    }
}
