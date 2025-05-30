pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    // pub fn to_sprites_vertices(&self) -> [Vertex; 2] {
    //     [
    //         Vertex {
    //             x: self.x as i16,
    //             y: self.y as i16,
    //             ..Default::default()
    //         },
    //         Vertex {
    //             x: (self.x + self.w) as i16,
    //             y: (self.y + self.h) as i16,
    //             ..Default::default()
    //         },
    //     ]
    // }
}
