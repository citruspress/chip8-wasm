import * as wasm from "wasm";

const key_map = {
    '1': 1, '2': 2, '3': 3, '4': 12,
    'q': 4, 'w': 5, 'e': 6, 'r': 13,
    'a': 7, 's': 8, 'd': 9, 'f': 14,
    'z': 10, 'x': 0, 'c': 11, 'v': 15
};

var key_state = 0;

window.addEventListener("keydown", event => {
    if (key_map[event.key]) {
        key_state = key_state | 1 << (key_map[event.key]);
    }

    wasm.on_key_state_changed(key_state);
});

window.addEventListener("keyup", event => {
    if (key_map[event.key]) {
        key_state = key_state ^ 1 << (key_map[event.key]);
    }

    wasm.on_key_state_changed(key_state);
});

fetch('/roms/tetris.rom')
    .then(response => response.arrayBuffer())
    .then(buffer => {
        wasm.load(new Uint8Array(buffer));
        wasm.start();
    })
    .catch(err => console.error(err));
