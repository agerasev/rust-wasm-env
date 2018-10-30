use std::f64::consts::PI;

use vecmat::{vec::*, map::*};

extern {
    fn js_canvas_create() -> u32;

    fn js_canvas_size(id_: u32, ptr: *mut i32);
    fn js_canvas_set_transform(id_:u32,m00:f64,m01:f64,m10:f64,m11:f64,x:f64,y:f64);

    fn js_canvas_fill_style(id_:u32,r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_stroke_style(id_:u32,r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_line_width(id_:u32,w:f64);

    fn js_canvas_clear_rect(id_:u32,x:f64,y:f64,w:f64,h:f64);
    fn js_canvas_fill_rect(id_:u32,x:f64,y:f64,w:f64,h:f64);
    #[allow(dead_code)]
    fn js_canvas_stroke_rect(id_:u32,x:f64,y:f64,w:f64,h:f64);

    fn js_canvas_begin_path(id_:u32);
    fn js_canvas_close_path(id_:u32);
    fn js_canvas_fill(id_:u32);
    fn js_canvas_stroke(id_:u32);
    
    fn js_canvas_arc(id_:u32,x:f64,y:f64,r:f64,sa:f64,ea:f64);
    fn js_canvas_move_to(id_:u32,x:f64,y:f64);
    fn js_canvas_line_to(id_:u32,x:f64,y:f64);
    fn js_canvas_bezier_curve_to(id_:u32,x1:f64,y1:f64,x2:f64,y2:f64,x:f64,y:f64);
    fn js_canvas_quadratic_curve_to(id_:u32,x1:f64,y1:f64,x:f64,y:f64);
    fn js_canvas_ellipse(id_:u32,x:f64,y:f64,rx:f64,ry:f64,rot:f64,sa:f64,ea:f64);
    fn js_canvas_rect(id_:u32,x:f64,y:f64,w:f64,h:f64);
}

pub type Color = Vec4<f64>;

pub struct Canvas {
    id_: u32,
    pub map: Affine2<f64>,
}

#[derive(Debug, Clone)]
pub enum Method {
    Fill {
        color: Color
    },
    Stroke {
        color: Color,
        width: f64
    },
}

#[derive(Debug, Clone)]
pub enum Path {
    Arc {
        pos: Vec2<f64>,
        rad: f64,
        angle: Vec2<f64>,
    },
    Circle {
        pos: Vec2<f64>,
        rad: f64,
    },
    MoveTo { pos: Vec2<f64> },
    LineTo { pos: Vec2<f64> },
    BezierTo {
        cp1: Vec2<f64>,
        cp2: Vec2<f64>,
        pos: Vec2<f64>,
    },
    QuadraticTo {
        cp1: Vec2<f64>,
        pos: Vec2<f64>,
    },
    Ellipse {
        pos: Vec2<f64>,
        rad: Vec2<f64>,
        rot: f64,
        angle: Vec2<f64>,
    },
    Rect {
        pos: Vec2<f64>,
        size: Vec2<f64>,
    },
    Close,
    List {
        paths: Vec<Path>
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe { ::drop_object(self.id_); }
    }
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            id_: unsafe { js_canvas_create() },
            map: Affine2::new()
        }
    }

    pub fn id(&self) -> u32 {
        self.id_
    }

    pub fn size(&self) -> Vec2<i32> {
        let mut buf: [i32; 2] = [0, 0];
        unsafe { js_canvas_size(self.id_, buf.as_mut_ptr()); }
        Vec2::from(buf[0], buf[1])
    }

    pub fn transform(&mut self, map: Affine2<f64>) {
        unsafe { 
            js_canvas_set_transform(
                self.id_, 
                map.linear[(0,0)],
                map.linear[(0,1)],
                map.linear[(1,0)],
                map.linear[(1,1)],
                map.shift[0],
                map.shift[1],
            );
        }
        self.map = map;
    }

    pub fn clear(&mut self) {
        let map = self.map.clone();
        self.transform(Affine2::new());
        let sizef = self.size().map(|v| v as f64);
        unsafe {
            js_canvas_clear_rect(self.id_, 0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }
    pub fn fill(&mut self, c: Color) {
        let map = self.map.clone();
        self.transform(Affine2::new());
        let sizef = self.size().map(|v| v as f64);
        unsafe { 
            js_canvas_fill_style(self.id_, c[0],c[1],c[2],c[3]);
            js_canvas_fill_rect(self.id_, 0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }

    fn draw_path(&mut self, path: &Path) {
        match *path {
            Path::Arc {pos, rad, angle} => unsafe {
                js_canvas_arc(self.id_, pos[0], pos[1], rad, angle[0], angle[1]);
            },
            Path::Circle {pos, rad} => unsafe {
                js_canvas_arc(self.id_, pos[0], pos[1], rad, 0.0, 2.0*PI);
            },
            Path::MoveTo {pos} => unsafe {
                js_canvas_move_to(self.id_, pos[0], pos[1]);
            },
            Path::LineTo {pos} => unsafe {
                js_canvas_line_to(self.id_, pos[0], pos[1]);
            },
            Path::BezierTo {cp1, cp2, pos} => unsafe {
                js_canvas_bezier_curve_to(self.id_, cp1[0], cp1[1], cp2[0], cp2[1], pos[0], pos[1]);
            },
            Path::QuadraticTo {cp1, pos} => unsafe {
                js_canvas_quadratic_curve_to(self.id_, cp1[0], cp1[1], pos[0], pos[1]);
            },
            Path::Ellipse {pos, rad, rot, angle} => unsafe {
                js_canvas_ellipse(self.id_, pos[0], pos[1], rad[0], rad[1], rot, angle[0], angle[1]); },
            Path::Rect {pos, size} => unsafe {
                js_canvas_rect(self.id_, pos[0], pos[1], size[0], size[1]);
            },
            Path::Close {} => unsafe {
                js_canvas_close_path(self.id_, );
            },
            Path::List {ref paths} => {
                for subpath in paths.iter() {
                    self.draw_path(subpath);
                }
            },
        }
    }

    fn apply_method(&mut self, method: &Method) {
        unsafe {
            match *method {
                Method::Fill { color: c } => {
                    js_canvas_fill_style(self.id_, c[0], c[1], c[2], c[3]);
                    js_canvas_fill(self.id_, );
                },
                Method::Stroke { color: c, width: w } => {
                    js_canvas_stroke_style(self.id_, c[0], c[1], c[2], c[3]);
                    js_canvas_line_width(self.id_, w);
                    js_canvas_stroke(self.id_, );
                }
            }
        }
    }

    pub fn draw(&mut self, path: &Path, method: &Method) {
        unsafe { js_canvas_begin_path(self.id_, ); }
        self.draw_path(path);
        self.apply_method(method);
    }
}
