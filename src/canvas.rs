use std::f64::consts::PI;

use physsol::{vec::*, map::*};

extern {
    fn js_canvas_size(ptr: *mut i32);
    fn js_canvas_set_transform(m00:f64,m01:f64,m10:f64,m11:f64,x:f64,y:f64);

    fn js_canvas_fill_style(r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_stroke_style(r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_line_width(w:f64);

    fn js_canvas_clear_rect(x:f64,y:f64,w:f64,h:f64);
    fn js_canvas_fill_rect(x:f64,y:f64,w:f64,h:f64);
    #[allow(dead_code)]
    fn js_canvas_stroke_rect(x:f64,y:f64,w:f64,h:f64);

    fn js_canvas_begin_path();
    fn js_canvas_close_path();
    fn js_canvas_fill();
    fn js_canvas_stroke();
    
    fn js_canvas_arc(x:f64,y:f64,r:f64,sa:f64,ea:f64);
    fn js_canvas_move_to(x:f64,y:f64);
    fn js_canvas_line_to(x:f64,y:f64);
    fn js_canvas_bezier_curve_to(x1:f64,y1:f64,x2:f64,y2:f64,x:f64,y:f64);
    fn js_canvas_quadratic_curve_to(x1:f64,y1:f64,x:f64,y:f64);
    fn js_canvas_ellipse(x:f64,y:f64,rx:f64,ry:f64,rot:f64,sa:f64,ea:f64);
    fn js_canvas_rect(x:f64,y:f64,w:f64,h:f64);
}

pub type Color = Vec4f64;

pub struct Canvas {
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
        pos: Vec2f64,
        rad: f64,
        angle: Vec2f64,
    },
    Circle {
        pos: Vec2f64,
        rad: f64,
    },
    MoveTo { pos: Vec2f64 },
    LineTo { pos: Vec2f64 },
    BezierTo {
        cp1: Vec2f64,
        cp2: Vec2f64,
        pos: Vec2f64,
    },
    QuadraticTo {
        cp1: Vec2f64,
        pos: Vec2f64,
    },
    Ellipse {
        pos: Vec2f64,
        rad: Vec2f64,
        rot: f64,
        angle: Vec2f64,
    },
    Rect {
        pos: Vec2f64,
        size: Vec2f64,
    },
    Close,
    List {
        paths: Vec<Path>
    }
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { map: Affine2::new() }
    }

    pub fn size(&self) -> Vec2i32 {
        let mut buf: [i32;2] = [0, 0];
        unsafe { js_canvas_size(buf.as_mut_ptr()); }
        Vec2i32::from_arr([buf[0], buf[1]])
    }

    pub fn transform(&mut self, map: Affine2<f64>) {
        unsafe { 
            js_canvas_set_transform(
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
            js_canvas_clear_rect(0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }
    pub fn fill(&mut self, c: Color) {
        let map = self.map.clone();
        self.transform(Affine2::new());
        let sizef = self.size().map(|v| v as f64);
        unsafe { 
            js_canvas_fill_style(c[0],c[1],c[2],c[3]);
            js_canvas_fill_rect(0.0, 0.0, sizef[0], sizef[1]);
        }
        self.transform(map);
    }

    fn draw_path(&mut self, path: &Path) {
        match *path {
            Path::Arc {pos, rad, angle} => unsafe {
                js_canvas_arc(pos[0], pos[1], rad, angle[0], angle[1]);
            },
            Path::Circle {pos, rad} => unsafe {
                js_canvas_arc(pos[0], pos[1], rad, 0.0, 2.0*PI);
            },
            Path::MoveTo {pos} => unsafe {
                js_canvas_move_to(pos[0], pos[1]);
            },
            Path::LineTo {pos} => unsafe {
                js_canvas_line_to(pos[0], pos[1]);
            },
            Path::BezierTo {cp1, cp2, pos} => unsafe {
                js_canvas_bezier_curve_to(cp1[0], cp1[1], cp2[0], cp2[1], pos[0], pos[1]);
            },
            Path::QuadraticTo {cp1, pos} => unsafe {
                js_canvas_quadratic_curve_to(cp1[0], cp1[1], pos[0], pos[1]);
            },
            Path::Ellipse {pos, rad, rot, angle} => unsafe {
                js_canvas_ellipse(pos[0], pos[1], rad[0], rad[1], rot, angle[0], angle[1]); },
            Path::Rect {pos, size} => unsafe {
                js_canvas_rect(pos[0], pos[1], size[0], size[1]);
            },
            Path::Close {} => unsafe {
                js_canvas_close_path();
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
                    js_canvas_fill_style(c[0],c[1],c[2],c[3]);
                    js_canvas_fill();
                },
                Method::Stroke { color: c, width: w } => {
                    js_canvas_stroke_style(c[0],c[1],c[2],c[3]);
                    js_canvas_line_width(w);
                    js_canvas_stroke();
                }
            }
        }
    }

    pub fn draw(&mut self, path: &Path, method: &Method) {
        unsafe { js_canvas_begin_path(); }
        self.draw_path(path);
        self.apply_method(method);
    }
}
