import * as wasm from "gameboy";

const emu = wasm.get_emulator("hackfest.gb");
const ctx = display.getContext('2d');

let frame_start_time = null;
let req = null;

const rom = fetch('hackfest.gb').then(rom => {
  rom.blob().then(blob => {
    var reader = new FileReader();
    reader.addEventListener("loadend", function() {
      const data = new Uint8Array(reader.result);
      wasm.load_cartridge_data(emu, data);
      wasm.reset(emu);
      console.log("ROM loaded");
      req = window.requestAnimationFrame(step_emulator);
    });
    reader.readAsArrayBuffer(blob);
  });
});

let prev_ts = 0;
const step_emulator = ts => {
  const draw = wasm.run_until_redraw(emu);
  if(draw) {
    const displayBuffer = wasm.display_buffer(emu);
    ctx.putImageData(new ImageData(Uint8ClampedArray.from(displayBuffer), 160, 144), 0, 0);
  }
  req = window.requestAnimationFrame(step_emulator);
};

document.getElementById('stop').addEventListener('click', () => window.cancelAnimationFrame(req));
