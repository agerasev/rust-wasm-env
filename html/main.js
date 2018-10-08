let wasm = null;
let last = null;
let done = true;

let event_data = new ArrayBuffer(0x100);

let handlers = {
    "timeout": (dt) => {
        let view = new DataView(event_data);
        view.setFloat64(0, dt);
        wasm.exports.handle(0x0101);
    },
    "step": (dt) => {
        let view = new DataView(event_data);
        view.setFloat64(0, dt, true);
        wasm.exports.handle(0x0102);
    },
    "render": () => {
        wasm.exports.handle(0x0103);
    },
}

let load_str = (ptr, len) => {
    const view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
    //const utf8dec = new TextDecoder("utf-8");
    //return utf8dec.decode(view);
    let str = "";
    for (let i = 0; i < view.length; i++) {
        str += String.fromCharCode(view[i]);
    }
    return str;
}

let env = {
    js_console: (type, ptr, len) => {
        let str = load_str(ptr, len);
        if (type == 1) {
            console.error(str);
        } else {
            console.log(str);
        }
    },
    js_timeout: (sec) => {
        setTimeout(() => {
            handlers["timeout"](parseFloat(sec));
        }, 1000*sec);
    },
    js_crypto_random: (ptr, len) => {
        let view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
        return window.crypto.getRandomValues(view);
    },
    js_get_event_data: (ptr, len) => {
        let dst = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
        let src = new Uint8Array(event_data);
        for (let i = 0; i < len; i++) {
            dst[i] = src[i];
        }
    }
};

let render = () => {
    if (!done) {
        let now = +new Date();
        let ms = now - last;
        last = now;
        handlers["step"](parseFloat(0.001*ms));
        handlers["render"]();
        window.requestAnimationFrame(render);
    }
};

let import_env = (env, im_env, prefix) => {
    prefix = !prefix ? "" : prefix;
    for (let key in im_env) {
        if (im_env.hasOwnProperty(key)) {
            env[prefix + key] = im_env[key];
        }
    }
    return env;
};

let load_wasm = (path, env, onload) => {
    fetch(path + "?_=" + Math.floor(Math.random()*0x80000000))
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {env: env}))
    .then(results => {
        onload(results.instance);
    });
};

let resize = () => {
    let width = 
        window.innerWidth || 
        document.documentElement.clientWidth || 
        document.body.clientWidth;
    let height = 
        window.innerHeight ||
        document.documentElement.clientHeight ||
        document.body.clientHeight;

    canvas_resize(width, height);
    if (wasm && done) {
        handlers["render"]();
    }

    console.log("[info] resize: " + width + " x " + height);
};

window.addEventListener("load", () => {
    canvas_init();
    resize();
    window.addEventListener("resize", resize);

    import_env(env, math_env, "");
    import_env(env, canvas_env, "js_canvas_");

    let pause_button = document.getElementById("pause");
    let start = () => {
        done = false;
        last = +new Date();
        window.requestAnimationFrame(render);
        console.log("[info] start animation");
        pause_button.innerText = "Pause";
    };
    let stop = () => {
        done = true;
        console.log("[info] stop animation");
        pause_button.innerText = "Resume";
    };
    pause_button.addEventListener("click", () => {
        if (done) {
            start();
        } else {
            stop();
        }
    });

    console.log("load wasm");

    load_wasm("../../main.wasm", env, instance => {
        wasm = instance;
        console.log("wasm init");
        wasm.exports.init();
        start();
    });

});
