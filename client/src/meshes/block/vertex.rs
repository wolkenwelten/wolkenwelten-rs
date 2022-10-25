use super::util;
use wolkenwelten_game::Side;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct BlockVertex {
    x: u8,
    y: u8,
    z: u8,
    texture_index: u8, // Right now we don't really use 256 distinct block faces, ~32 should suffice for a long time
    side_and_light: u8, // And another one here as well
}

impl BlockVertex {
    pub fn new(x: u8, y: u8, z: u8, texture_index: u8, side: u8, light: u8) -> Self {
        let side_and_light = side | (light << 4);
        Self {
            x,
            y,
            z,
            texture_index,
            side_and_light,
        }
    }

    pub fn add_front(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (w, h, d): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Front.into();
        let z = z + d;
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x + w,
            y,
            z,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn add_back(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (w, h, _): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Back.into();
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y,
            z,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn add_top(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (w, h, d): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Top.into();
        let y = y + h;
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y,
            z,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn add_bottom(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (w, _, d): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Bottom.into();
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x + w,
            y,
            z,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x + w,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn add_left(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (_, h, d): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Left.into();
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y + h,
            z + d,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn add_right(
        vertices: &mut Vec<Self>,
        (x, y, z): (u8, u8, u8),
        (w, h, d): (u8, u8, u8),
        texture_index: u8,
        light: u16,
    ) {
        let side: u8 = Side::Right.into();
        let x = x + w;
        vertices.push(Self::new(x, y, z, texture_index, side, (light & 0xF) as u8));
        vertices.push(Self::new(
            x,
            y + h,
            z,
            texture_index,
            side,
            ((light >> 4) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y + h,
            z + d,
            texture_index,
            side,
            ((light >> 8) & 0xF) as u8,
        ));
        vertices.push(Self::new(
            x,
            y,
            z + d,
            texture_index,
            side,
            ((light >> 12) & 0xF) as u8,
        ));
    }

    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>();
        unsafe {
            let offset = util::vertex_attrib_int_pointer(stride, 0, 0, gl::UNSIGNED_BYTE, 3, 3);
            let offset =
                util::vertex_attrib_int_pointer(stride, 1, offset, gl::UNSIGNED_BYTE, 1, 1);
            util::vertex_attrib_int_pointer(stride, 2, offset, gl::UNSIGNED_BYTE, 1, 1);
        }
    }
}
