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
                uses_point_size: true,
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
