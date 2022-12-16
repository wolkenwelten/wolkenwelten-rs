// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use anyhow::Result;

/// A list of all the built-in shaders.
#[derive(Debug)]
pub struct ShaderList {
    pub block: glium::Program,
    pub mesh: glium::Program,
    pub text: glium::Program,
    pub voxel: glium::Program,
}

/// This is the prefix that has to be prepended to all vertex shaders,
/// Care must be taken that all shaders are valid in all 3 distinct GLSL
/// versions (300 es, 330 core, 130).
///
/// Because of this one should not start the shader with a #version statement
/// since this needs to be different depending on the target platform.
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

/// This is the prefix that has to be prepended to all fragment shaders,
/// Care must be taken that all shaders are valid in all 3 distinct GLSL
/// versions (300 es, 330 core, 130).
///
/// Because of this one should not start the shader with a #version statement
/// since this needs to be different depending on the target platform.
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
    /// Create a new shader program from 2 vert/frag source slices where the appropriate #version prefix
    /// will be automatically prepended
    pub fn new_program(display: &glium::Display, vert: &str, frag: &str) -> Result<glium::Program> {
        let vert = format!("{}\n{}", VERT_PREFIX, vert);
        let frag = format!("{}\n{}", FRAG_PREFIX, frag);
        Ok(glium::Program::from_source(display, &vert, &frag, None)?)
    }

    /// Create a new shader program from 2 vert/frag source slices where the appropriate #version prefix
    /// will be automatically prepended
    ///
    /// This also sets the uses_point_size boolean so that GL_VERTEX_PROGRAM_POINT_SIZE is enabled before
    /// doing the actual draw calls.
    pub fn new_point_program(
        display: &glium::Display,
        vert: &str,
        frag: &str,
    ) -> Result<glium::Program> {
        let vert = format!("{}\n{}", VERT_PREFIX, vert);
        let frag = format!("{}\n{}", FRAG_PREFIX, frag);
        Ok(glium::Program::new(
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
        )?)
    }

    /// Initialize a new shader list and compile/link all the shaders.
    pub fn new(display: &glium::Display) -> Result<Self> {
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
        let voxel = Self::new_program(
            display,
            include_str!("../shaders/voxel.vert"),
            include_str!("../shaders/voxel.frag"),
        )?;

        Ok(Self {
            block,
            mesh,
            text,
            voxel,
        })
    }
}
