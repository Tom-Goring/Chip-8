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

// Initialize the canvas with room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("chip8-canvas");
canvas.height = (PIXEL_SIZE + 1) * height + 1;
canvas.width = (PIXEL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

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
  

const renderLoop = () => {
  drawCells();

  chip8.tick();

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
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

requestAnimationFrame(renderLoop);
pause();