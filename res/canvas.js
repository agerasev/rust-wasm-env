class CanvasModule {
    constructor() {
        let mkcol = (r,g,b,a) => "rgba(" + 255*r + "," + 255*g + "," + 255*b + "," + a + ")"

        this.canvas = null;
        this.context = null;

        this.resize = () => {
            let width = 
                window.innerWidth || 
                document.documentElement.clientWidth || 
                document.body.clientWidth;
            let height = 
                window.innerHeight ||
                document.documentElement.clientHeight ||
                document.body.clientHeight;

            this.canvas.width = width;
            this.canvas.height = height;

            console.log("[info] resize: " + width + " x " + height);
        };

        this.init = () => {
            this.canvas = document.getElementById("screen");
            this.context = this.canvas.getContext("2d");

            this.resize();
            window.addEventListener("resize", this.resize);
        };

        this.exports = {
            "size": {
                "func": (ptr) => {
                    let view = new Uint32Array(WASM.exports.memory.buffer, ptr, 2);
                    view[0] = this.canvas.width;
                    view[1] = this.canvas.height;
                },
                "args": ["usize"],
            },

            "set_transform": {
                "func": (m00, m01, m10, m11, x, y) => {
                    this.context.setTransform(m00, m01, m10, m11, x, y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
            },

            "fill_style": {
                "func": (r,g,b,a) => {
                    this.context.fillStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "stroke_style": {
                "func": (r,g,b,a) => {
                    this.context.strokeStyle = mkcol(r,g,b,a);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "line_width": {
                "func": (w) => {
                    this.context.lineWidth = w;
                },
                "args": ["f64"],
            },

            "clear_rect": {
                "func": (x,y,w,h) => {
                    this.context.clearRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "fill_rect": {
                "func": (x,y,w,h) => {
                    this.context.fillRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "stroke_rect": {
                "func": (x,y,w,h) => {
                    this.context.strokeRect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },

            "begin_path": {
                "func": () => {
                    this.context.beginPath();
                },
                "args": [],
            },
            "close_path": {
                "func": () => {
                    this.context.closePath();
                },
                "args": [],
            },
            "fill": {
                "func": () => {
                    this.context.fill();
                },
                "args": [],
            },
            "stroke": {
                "func": () => {
                    this.context.stroke();
                },
                "args": [],
            },
            
            "arc": {
                "func": (x,y,r,sa,ea) => {
                    this.context.arc(x,y,r,sa,ea);
                },
                "args": ["f64","f64","f64","f64","f64"],
            },
            "move_to": {
                "func": (x,y) => {
                    this.context.moveTo(x,y);
                },
                "args": ["f64","f64"],
            },
            "line_to": {
                "func": (x,y) => {
                    this.context.lineTo(x,y);
                },
                "args": ["f64","f64"],
            },
            "bezier_curve_to": {
                "func": (x1,y1,x2,y2,x,y) => {
                    this.context.bezierCurveTo(x1,y1,x2,y2,x,y);
                },
                "args": ["f64","f64","f64","f64","f64","f64"],
            },
            "quadratic_curve_to": {
                "func": (x1,y1,x,y) => {
                    this.context.quadraticCurveTo(x1,y1,x,y);
                },
                "args": ["f64","f64","f64","f64"],
            },
            "ellipse": {
                "func": (x,y,rx,ry,rot,sa,ea) => {
                    this.context.ellipse(x,y,rx,ry,rot,sa,ea,0);
                },
                "args": ["f64","f64","f64","f64","f64","f64","f64"],
            },
            "rect": {
                "func": (x,y,w,h) => {
                    this.context.rect(x,y,w,h);
                },
                "args": ["f64","f64","f64","f64"],
            },
        }
    }
};

MODULES["canvas"] = new CanvasModule();
