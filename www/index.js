import * as wasm from "wasm";

fetch('https://github.com/dmatlack/chip8/blob/master/roms/demos/Maze%20(alt)%20%5BDavid%20Winter,%20199x%5D.ch8?raw=true')
    .then(response => response.arrayBuffer())
    .then(buffer => {
        wasm.load(new Uint8Array(buffer));
        wasm.start();
    })
    .catch(err => console.error(err));
