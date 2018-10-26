class CanvasModule {
    constructor() {
        let mkcol = (r,g,b,a) => "rgba(" + 255*r + "," + 255*g + "," + 255*b + "," + a + ")"

        this.resize = () => {
            if (this.screen != null) {
                let width = 
                    window.innerWidth || 
                    document.documentElement.clientWidth || 
                    document.body.clientWidth;
                let height = 
                    window.innerHeight ||
                    document.documentElement.clientHeight ||
                    document.body.clientHeight;

                this.screen.canvas.width = width;
                this.screen.canvas.height = height;

                console.log("[info] resize: " + width + " x " + height);
            }
        };

        this.init = () => {
            window.addEventListener("resize", this.resize);
        };

        this.screen = null;

        this.exports = {
            "create": {
                "func": () => {
                    let obj = {};
                    obj.canvas = document.createElement("canvas");
                    obj.context = obj.canvas.getContext("2d");
                    return add_object(obj);
                },
                "args": [],
            },

            "set_screen": {
                "func": (id) => {
                    if (this.screen != null) {
                        this.screen.canvas.remove();
                        this.screen.canvas.classList.remove("screen");
                    }
                    if (id == 0) {
                        this.screen = null;
                    } else {
                        this.screen = OBJECTS[id];
                        this.screen.canvas.classList.add("screen");
                        document.body.appendChild(this.screen.canvas);
                        this.resize();
                    }
                },
                "args": ["u32"],
            },

            "size": {
                "func": (id, ptr) => {
                    let canvas = OBJECTS[id].canvas;
                    let view = new Uint32Array(WASM.exports.memory.buffer, ptr, 2);
                    view[0] = canvas.width;
                    view[1] = canvas.height;
                },
                "args": ["usize"],
            },

            "set_transform": {
                "func": (id,m00,m01,m10,m11,x,y) => {
                    OBJECTS[id].context.setTransform(m00, m01, m10, m11, x, y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
            },

            "fill_style": {
                "func": (id,r,g,b,a) => {
                    OBJECTS[id].context.fillStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "stroke_style": {
                "func": (id,r,g,b,a) => {
                    OBJECTS[id].context.strokeStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "line_width": {
                "func": (id,w) => {
                    OBJECTS[id].context.lineWidth = w;
                },
                "args": ["f64"],
            },

            "clear_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.clearRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "fill_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.fillRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "stroke_rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.strokeRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },

            "begin_path": {
                "func": (id,) => {
                    OBJECTS[id].context.beginPath();
                },
                "args": [],
            },
            "close_path": {
                "func": (id,) => {
                    OBJECTS[id].context.closePath();
                },
                "args": [],
            },
            "fill": {
                "func": (id,) => {
                    OBJECTS[id].context.fill();
                },
                "args": [],
            },
            "stroke": {
                "func": (id,) => {
                    OBJECTS[id].context.stroke();
                },
                "args": [],
            },
            
            "arc": {
                "func": (id,x,y,r,sa,ea) => {
                    OBJECTS[id].context.arc(x,y,r,sa,ea);
                },
                "args": ["f64","f64","f64","f64","f64"],
            },
            "move_to": {
                "func": (id,x,y) => {
                    OBJECTS[id].context.moveTo(x,y);
                },
                "args": ["f64","f64"],
            },
            "line_to": {
                "func": (id,x,y) => {
                    OBJECTS[id].context.lineTo(x,y);
                },
                "args": ["f64","f64"],
            },
            "bezier_curve_to": {
                "func": (id,x1,y1,x2,y2,x,y) => {
                    OBJECTS[id].context.bezierCurveTo(x1,y1,x2,y2,x,y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
            },
            "quadratic_curve_to": {
                "func": (id,x1,y1,x,y) => {
                    OBJECTS[id].context.quadraticCurveTo(x1,y1,x,y);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "ellipse": {
                "func": (id,x,y,rx,ry,rot,sa,ea) => {
                    OBJECTS[id].context.ellipse(x,y,rx,ry,rot,sa,ea,0);
                },
                "args": ["f64","f64","f64","f64","f64","f64","f64"],
            },
            "rect": {
                "func": (id,x,y,w,h) => {
                    OBJECTS[id].context.rect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
        }
    }
};

MODULES["canvas"] = new CanvasModule();
