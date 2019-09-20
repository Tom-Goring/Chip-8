import { CPU } from "../pkg/web_emulator";
import { memory } from "../pkg/web_emulator_bg";

const CELL_SIZE = 18;
const GRID_COLOUR = "#CCCCCC";
const OFF_COLOUR = "#FFFFFF";
const ON_COLOUR = "#000000";

// These must match `Cell::Alive` and `Cell::Dead` in `src/lib.rs`.
const OFF = 0;
const ON = 1;

const chip8 = CPU.new();
const width = chip8.width();
const height = chip8.height();

// Initialize the canvas with room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("chip8-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

const renderLoop = () => {
  drawGrid();
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

const drawGrid = () => {
  ctx.beginPath();
  ctx.lineWidth = 1 / window.devicePixelRatio;
  ctx.strokeStyle = GRID_COLOUR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

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
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

requestAnimationFrame(renderLoop);
