class CanvasModule {
    constructor() {
        let mkcol = (r,g,b,a) => "rgba(" + 255*r + "," + 255*g + "," + 255*b + "," + a + ")"

        this.exports = {
            "create": {
                "func": () => {
                    let obj = {};
                    obj.canvas = document.createElement("canvas");
                    obj.context = obj.canvas.getContext("2d");
                    return add_object(obj);
                },
                "args": [],
                "ret": "void",
            },

            "size": {
                "func": (id, ptr) => {
                    let canvas = OBJECTS[id].canvas;
                    let view = new Uint32Array(WASM.exports.memory.buffer, ptr, 2);
                    view[0] = canvas.width;
                    view[1] = canvas.height;
                },
                "args": ["usize"],
                "ret": "void",
            },

            "set_transform": {
                "func": (id,m00,m01,m10,m11,x,y) => {
                    OBJECTS[id].context.setTransform(m00, m01, m10, m11, x, y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
                "ret": "void",
            },

            "fill_style": {
                "func": (id,r,g,b,a) => {
                    OBJECTS[id].context.fillStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
            "stroke_style": {
                "func": (id,r,g,b,a) => {
                    OBJECTS[id].context.strokeStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
            "line_width": {
                "func": (id,w) => {
                    OBJECTS[id].context.lineWidth = w;
                },
                "args": ["f64"],
                "ret": "void",
            },

            "clear_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.clearRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
            "fill_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.fillRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
            "stroke_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.strokeRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },

            "begin_path": {
                "func": (id,) => {
                    OBJECTS[id].context.beginPath();
                },
                "args": [],
                "ret": "void",
            },
            "close_path": {
                "func": (id,) => {
                    OBJECTS[id].context.closePath();
                },
                "args": [],
                "ret": "void",
            },
            "fill": {
                "func": (id,) => {
                    OBJECTS[id].context.fill();
                },
                "args": [],
                "ret": "void",
            },
            "stroke": {
                "func": (id,) => {
                    OBJECTS[id].context.stroke();
                },
                "args": [],
                "ret": "void",
            },
            
            "arc": {
                "func": (id,x,y,r,sa,ea) => {
                    OBJECTS[id].context.arc(x,y,r,sa,ea);
                },
                "args": ["f64","f64","f64","f64","f64"],
                "ret": "void",
            },
            "move_to": {
                "func": (id,x,y) => {
                    OBJECTS[id].context.moveTo(x,y);
                },
                "args": ["f64","f64"],
                "ret": "void",
            },
            "line_to": {
                "func": (id,x,y) => {
                    OBJECTS[id].context.lineTo(x,y);
                },
                "args": ["f64","f64"],
                "ret": "void",
            },
            "bezier_curve_to": {
                "func": (id,x1,y1,x2,y2,x,y) => {
                    OBJECTS[id].context.bezierCurveTo(x1,y1,x2,y2,x,y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
                "ret": "void",
            },
            "quadratic_curve_to": {
                "func": (id,x1,y1,x,y) => {
                    OBJECTS[id].context.quadraticCurveTo(x1,y1,x,y);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
            "ellipse": {
                "func": (id,x,y,rx,ry,rot,sa,ea) => {
                    OBJECTS[id].context.ellipse(x,y,rx,ry,rot,sa,ea,0);
                },
                "args": ["f64","f64","f64","f64","f64","f64","f64"],
                "ret": "void",
            },
            "rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.rect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
                "ret": "void",
            },
        }
    }
    init() {

    }
};

MODULES["canvas"] = new CanvasModule();
