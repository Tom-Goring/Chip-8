import { CPU } from "../pkg/web_emulator";
import { memory } from "../pkg/web_emulator_bg";

const PIXEL_SIZE = 18;
const ON_COLOUR = "#FFFFFF";
const OFF_COLOUR = "#000000";

// These must match `Cell::Alive` and `Cell::Dead` in `src/lib.rs`.
const OFF = 0;
const ON = 1;

const chip8 = CPU.new();
const width = chip8.width();
const height = chip8.height();

let buffer = new ArrayBuffer(16);
var keys = new Uint8Array(buffer);

for (var i = 0; i < 16; i++) {
	keys[i] = 0;
}

// Initialize the canvas with room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("chip8-canvas");
canvas.height = (PIXEL_SIZE + 1) * height + 1;
canvas.width = (PIXEL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

document.getElementById("files").addEventListener("change", handleFiles);

function handleFiles() {
	const fileList = this.files;
	const file = fileList[0];
	var reader = new FileReader();
	reader.onload = function() {
		var arrayBuffer = reader.result;
		var gameData = new Uint8Array(arrayBuffer);
		chip8.load(gameData);
	};
	reader.readAsArrayBuffer(file);
}

const resetButton = document.getElementById("reset");

resetButton.addEventListener("click", event => {
	chip8.trigger_reset();
});

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const pixelsPtr = chip8.pixels();
  const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

	  ctx.fillStyle = pixels[idx] === OFF ? OFF_COLOUR : ON_COLOUR;

      ctx.fillRect(
        col * (PIXEL_SIZE),
        row * (PIXEL_SIZE),
        PIXEL_SIZE,
        PIXEL_SIZE
      );
    }
  }

  ctx.stroke();
};

canvas.addEventListener("keydown", handleKeyDown, true);
canvas.addEventListener("keyup", handleKeyUp, true);

function handleKeyDown(e) {
	switch (e.keyCode) {
		case 49:
			keys[0] = 1;
			break;
		case 50:
			keys[1] = 1;
			break;
		case 51:
			keys[2] = 1;
			break;
		case 52:
			keys[3] = 1;
			break;
		case 81:
			keys[4] = 1;
			break;
		case 87:
			keys[5] = 1;
			break;
		case 69:
			keys[6] = 1;
			break;
		case 82:
			keys[7] = 1;
			break;
		case 65:
			keys[8] = 1;
			break;
		case 83:
			keys[9] = 1;
			break;
		case 68:
			keys[10] = 1;
			break;
		case 70:
			keys[11] = 1;
			break;
		case 90:
			keys[12] = 1;
			break;
		case 88:
			keys[13] = 1;
			break;
		case 67:
			keys[14] = 1;
			break;
		case 86:
			keys[15] = 1;
			break;
		default:
			break;
	}
}

function handleKeyUp(e) {
	switch (e.keyCode) {
		case 49:
			keys[0] = 0;
			break;
		case 50:
			keys[1] = 0;
			break;
		case 51:
			keys[2] = 0;
			break;
		case 52:
			keys[3] = 0;
			break;
		case 81:
			keys[4] = 0;
			break;
		case 87:
			keys[5] = 0;
			break;
		case 69:
			keys[6] = 0;
			break;
		case 82:
			keys[7] = 0;
			break;
		case 65:
			keys[8] = 0;
			break;
		case 83:
			keys[9] = 0;
			break;
		case 68:
			keys[10] = 0;
			break;
		case 70:
			keys[11] = 0;
			break;
		case 90:
			keys[12] = 0;
			break;
		case 88:
			keys[13] = 0;
			break;
		case 67:
			keys[14] = 0;
			break;
		case 86:
			keys[15] = 0;
			break;
	}
}

setInterval(function() {
	chip8.set_key_array(keys);
	chip8.tick();
}, 2);

function draw() {
	drawCells();
	requestAnimationFrame(draw);
}

requestAnimationFrame(draw);