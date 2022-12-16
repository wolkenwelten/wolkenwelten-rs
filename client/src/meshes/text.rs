// Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
// All rights reserved. AGPL-3.0+ license.
use glium;
use glium::implement_vertex;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex2D {
    pub pos: [i16; 2],
    pub tex: [i16; 2],
    pub color: [u8; 4],
}
implement_vertex!(Vertex2D, pos normalize(false), tex normalize(false), color normalize(true));

#[derive(Debug)]
pub struct TextMesh {
    buffer: glium::VertexBuffer<Vertex2D>,
    vertex_count: usize,

    finished: bool,
    vertices: Vec<Vertex2D>,
}

impl TextMesh {
    pub fn prepare(&mut self, display: &glium::Display) {
        if !self.finished {
            self.buffer = glium::VertexBuffer::dynamic(display, self.vertices.as_slice()).unwrap();
            self.vertex_count = self.vertices.len();
            self.vertices.clear();
        }
    }

    pub fn buffer(&self) -> &glium::VertexBuffer<Vertex2D> {
        &self.buffer
    }

    pub fn new(display: &glium::Display) -> Result<Self, glium::vertex::BufferCreationError> {
        let buffer = glium::VertexBuffer::empty(display, 0)?;
        Ok(Self {
            buffer,
            vertex_count: 0,
            finished: false,
            vertices: Vec::with_capacity(8),
        })
    }

    pub fn push_vertex(&mut self, x: i16, y: i16, u: i16, v: i16, color: [u8; 4]) -> &mut Self {
        self.vertices.push(Vertex2D {
            pos: [x, y],
            tex: [u, v],
            color,
        });
        self
    }

    pub fn push_box(
        &mut self,
        (x, y, w, h): (i16, i16, i16, i16),
        (u, v, uw, vh): (i16, i16, i16, i16),
        rgba: [u8; 4],
    ) -> &mut Self {
        self.push_vertex(x, y + h, u, v + vh, rgba)
            .push_vertex(x + w, y, u + uw, v, rgba)
            .push_vertex(x, y, u, v, rgba)
            .push_vertex(x + w, y, u + uw, v, rgba)
            .push_vertex(x, y + h, u, v + vh, rgba)
            .push_vertex(x + w, y + h, u + uw, v + vh, rgba)
    }

    pub fn push_heart(
        &mut self,
        x: i16,
        y: i16,
        size: i16,
        rgba: [u8; 4],
        fill_state: i16,
    ) -> &mut Self {
        let u = 128 - 20 + fill_state * 4;
        let v = 128 - 4;
        self.push_box((x, y, size, size), (u, v, 4, 4), rgba)
    }

    pub fn push_glyph(&mut self, x: i16, y: i16, size: i16, rgba: [u8; 4], c: char) -> &mut Self {
        let glyph_width: i16 = 8 * size;

        if x < -glyph_width {
            return self;
        }
        if y < -glyph_width {
            return self;
        }
        if c == '\0' {
            return self;
        }
        if c == ' ' {
            return self;
        }

        let cc = c as u8;
        let u = 32 + ((cc & 0xF) as i16 * size.min(2));
        let voff = if size == 1 { 128 - 16 } else { 128 };
        let v = voff - ((((cc >> 4) & 0xF) + 1) as i16 * size.min(2));

        self.push_box(
            (x, y, glyph_width, glyph_width),
            (u, v, size.min(2), size.min(2)),
            rgba,
        )
    }

    pub fn push_string(
        &mut self,
        x: i16,
        y: i16,
        size: i32,
        rgba: [u8; 4],
        text: &str,
    ) -> &mut Self {
        let glyph_width: i32 = 8 * size;
        text.chars().enumerate().for_each(|(i, c)| {
            let x: i16 = x + ((i as i32) * glyph_width) as i16;
            self.push_glyph(x, y, size as i16, rgba, c);
        });
        self
    }
}
