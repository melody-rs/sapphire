use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
    pub alpha: f64,
}

impl Color {
    pub const WHITE: Self = Self {
        red: 255.0,
        blue: 255.0,
        green: 255.0,
        alpha: 255.0,
    };

    pub const BLACK: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 255.0,
    };

    pub const GREY: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 128.0,
    };

    pub const TRANSPARENT: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 0.0,
    };
}

#[derive(Clone, Copy, PartialEq, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Tone {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
    pub gray: f64,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Default)]
pub struct Table {
    xsize: usize,
    ysize: usize,
    zsize: usize,
    data: Vec<i16>,
}

impl Table {
    pub fn new(xsize: usize, ysize: usize, zsize: usize) -> Self {
        let data = vec![0; xsize * ysize * zsize];
        Self {
            xsize,
            ysize,
            zsize,
            data,
        }
    }

    pub fn new_data(xsize: usize, ysize: usize, zsize: usize, data: Vec<i16>) -> Self {
        assert_eq!(xsize * ysize * zsize, data.len());

        Self {
            xsize,
            ysize,
            zsize,
            data,
        }
    }

    pub fn xsize(&self) -> usize {
        self.xsize
    }

    pub fn ysize(&self) -> usize {
        self.ysize
    }

    pub fn zsize(&self) -> usize {
        self.zsize
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn data(&self) -> &[i16] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [i16] {
        &mut self.data
    }

    pub fn resize(&mut self, xsize: usize, ysize: usize, zsize: usize) {
        let mut new_data = vec![0; xsize * ysize];

        // A naive for loop like this is optimized to a handful of memcpys.
        for z in 0..self.zsize.min(zsize) {
            for y in 0..self.ysize.min(ysize) {
                for x in 0..self.xsize.min(xsize) {
                    new_data[(xsize * ysize * z) + (xsize * y) + x] = self[(x, y, z)]
                }
            }
        }

        self.xsize = xsize;
        self.ysize = ysize;
        self.zsize = zsize;

        self.data = new_data;
    }

    fn index_for(&self, (x, y, z): (usize, usize, usize)) -> usize {
        x + (y * self.xsize + (z * self.xsize * self.ysize))
    }

    pub fn get(&self, index: (usize, usize, usize)) -> Option<&i16> {
        let index = self.index_for(index);
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: (usize, usize, usize)) -> Option<&mut i16> {
        let index = self.index_for(index);
        self.data.get_mut(index)
    }
}

impl Index<usize> for Table {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.xsize);

        let index = self.index_for((index, 0, 0));
        &self.data[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.xsize);

        let index = self.index_for((index, 0, 0));
        &mut self.data[index]
    }
}

impl Index<(usize, usize)> for Table {
    type Output = i16;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);

        let index = self.index_for((x, y, 0));
        &self[index]
    }
}

impl IndexMut<(usize, usize)> for Table {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);

        let index = self.index_for((x, y, 0));
        &mut self.data[index]
    }
}

impl Index<(usize, usize, usize)> for Table {
    type Output = i16;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < self.xsize);
        debug_assert!(index.1 < self.ysize);
        debug_assert!(index.2 < self.zsize);

        let index = self.index_for(index);
        &self.data[index]
    }
}

impl IndexMut<(usize, usize, usize)> for Table {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < self.xsize);
        debug_assert!(index.1 < self.ysize);
        debug_assert!(index.2 < self.zsize);

        let index = self.index_for(index);
        &mut self.data[index]
    }
}
