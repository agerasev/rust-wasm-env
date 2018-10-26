use std::f64::consts::PI;
use std::sync::Once;

use vecmat::{vec::*, map::*};

extern {
    fn js_canvas_create() -> u32;
    fn js_canvas_set_screen(id: u32);

    fn js_canvas_size(id: u32, ptr: *mut i32);
    fn js_canvas_set_transform(id:u32,m00:f64,m01:f64,m10:f64,m11:f64,x:f64,y:f64);

    fn js_canvas_fill_style(id:u32,r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_stroke_style(id:u32,r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_line_width(id:u32,w:f64);

    fn js_canvas_clear_rect(id:u32,x:f64,y:f64,w:f64,h:f64);
    fn js_canvas_fill_rect(id:u32,x:f64,y:f64,w:f64,h:f64);
    #[allow(dead_code)]
    fn js_canvas_stroke_rect(id:u32,x:f64,y:f64,w:f64,h:f64);

    fn js_canvas_begin_path(id:u32);
    fn js_canvas_close_path(id:u32);
    fn js_canvas_fill(id:u32);
    fn js_canvas_stroke(id:u32);
    
    fn js_canvas_arc(id:u32,x:f64,y:f64,r:f64,sa:f64,ea:f64);
    fn js_canvas_move_to(id:u32,x:f64,y:f64);
    fn js_canvas_line_to(id:u32,x:f64,y:f64);
    fn js_canvas_bezier_curve_to(id:u32,x1:f64,y1:f64,x2:f64,y2:f64,x:f64,y:f64);
    fn js_canvas_quadratic_curve_to(id:u32,x1:f64,y1:f64,x:f64,y:f64);
    fn js_canvas_ellipse(id:u32,x:f64,y:f64,rx:f64,ry:f64,rot:f64,sa:f64,ea:f64);
    fn js_canvas_rect(id:u32,x:f64,y:f64,w:f64,h:f64);
}

static MOD_CHECK: Once = Once::new();

pub type Color = Vec4<f64>;

pub struct Canvas {
    id: u32,
    map: Affine2<f64>,
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
        unsafe { ::drop_object(self.id); }
    }
}

impl Canvas {
    pub fn new() -> Self {
        MOD_CHECK.call_once(|| {
            if !::mod_check("canvas") {
                panic!("js module 'canvas' is not loaded");
            }
        });
        Canvas {
            id: unsafe { js_canvas_create() },
            map: Affine2::new()
        }
    }

    pub fn set_as_screen(&mut self) {
        unsafe { js_canvas_set_screen(self.id); }
    }

    pub fn size(&self) -> Vec2<i32> {
        let mut buf: [i32; 2] = [0, 0];
        unsafe { js_canvas_size(self.id, buf.as_mut_ptr()); }
        Vec2::from(buf[0], buf[1])
    }

    pub fn transform(&mut self, map: Affine2<f64>) {
        unsafe { 
            js_canvas_set_transform(
                self.id, 
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
            js_canvas_clear_rect(self.id, 0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }
    pub fn fill(&mut self, c: Color) {
        let map = self.map.clone();
        self.transform(Affine2::new());
        let sizef = self.size().map(|v| v as f64);
        unsafe { 
            js_canvas_fill_style(self.id, c[0],c[1],c[2],c[3]);
            js_canvas_fill_rect(self.id, 0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }

    fn draw_path(&mut self, path: &Path) {
        match *path {
            Path::Arc {pos, rad, angle} => unsafe {
                js_canvas_arc(self.id, pos[0], pos[1], rad, angle[0], angle[1]);
            },
            Path::Circle {pos, rad} => unsafe {
                js_canvas_arc(self.id, pos[0], pos[1], rad, 0.0, 2.0*PI);
            },
            Path::MoveTo {pos} => unsafe {
                js_canvas_move_to(self.id, pos[0], pos[1]);
            },
            Path::LineTo {pos} => unsafe {
                js_canvas_line_to(self.id, pos[0], pos[1]);
            },
            Path::BezierTo {cp1, cp2, pos} => unsafe {
                js_canvas_bezier_curve_to(self.id, cp1[0], cp1[1], cp2[0], cp2[1], pos[0], pos[1]);
            },
            Path::QuadraticTo {cp1, pos} => unsafe {
                js_canvas_quadratic_curve_to(self.id, cp1[0], cp1[1], pos[0], pos[1]);
            },
            Path::Ellipse {pos, rad, rot, angle} => unsafe {
                js_canvas_ellipse(self.id, pos[0], pos[1], rad[0], rad[1], rot, angle[0], angle[1]); },
            Path::Rect {pos, size} => unsafe {
                js_canvas_rect(self.id, pos[0], pos[1], size[0], size[1]);
            },
            Path::Close {} => unsafe {
                js_canvas_close_path(self.id, );
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
                    js_canvas_fill_style(self.id, c[0], c[1], c[2], c[3]);
                    js_canvas_fill(self.id, );
                },
                Method::Stroke { color: c, width: w } => {
                    js_canvas_stroke_style(self.id, c[0], c[1], c[2], c[3]);
                    js_canvas_line_width(self.id, w);
                    js_canvas_stroke(self.id, );
                }
            }
        }
    }

    pub fn draw(&mut self, path: &Path, method: &Method) {
        unsafe { js_canvas_begin_path(self.id, ); }
        self.draw_path(path);
        self.apply_method(method);
    }
}
