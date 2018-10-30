let WASM = null;
let BUFFER = null;

let MODULES = {};
let MOD_COUNTER = 1;

let OBJECTS = {};
let ID_COUNTER = 1;

let add_object = (obj) => {
    let id = ID_COUNTER;
    OBJECTS[id] = obj;
    ID_COUNTER += 1;
    return id;
};

let del_object = (id) => {
    if (OBJECTS.hasOwnProperty(id)) {
        delete OBJECTS[id];
        return 0;
    } else {
        return 1;
    }
}

/*
let term_out = function () {
    let str = "";
    for (let i = 0; i < arguments.length; ++i) {
        if (i != 0) { str += ' '; }
        str += String(arguments[i]);
    }
    str += '\n';
    document.getElementById("term").innerText += str;
};

(function () {
    console._log = console.log;
    console.log = () => {
        term_out(arguments);
        console._log.apply(console, arguments);
    };
    console._error = console.error;
    console.error = () => {
        term_out(arguments);
        console._error.apply(console, arguments);
    };
})();
*/
