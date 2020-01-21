import { Universe, Cell } from 'wasm-gol';
import { memory } from 'wasm-gol/wasm_gol_bg';

const CELL_PX_SIZE = 11;
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const pre = document.getElementById('gol-pre');
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('gol-canvas');
canvas.height = (CELL_PX_SIZE + 1) * height + 1;
canvas.width = (CELL_PX_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function drawGrid() {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_PX_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_PX_SIZE + 1) + 1, (CELL_PX_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_PX_SIZE + 1) + 1);
    ctx.lineTo((CELL_PX_SIZE + 1) * width + 1, j * (CELL_PX_SIZE + 1) + 1);
  }

  ctx.stroke();
}

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  ctx.beginPath();
  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      const idx = universe.get_index(i, j);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
      ctx.fillRect(
        j * (CELL_PX_SIZE + 1) + 1,
        i * (CELL_PX_SIZE + 1) + 1,
        CELL_PX_SIZE,
        CELL_PX_SIZE
      );
    }
  }
  ctx.stroke();
}

const gameLoop2 = () => {
  pre.textContent = universe.render();
  universe.tick();
  requestAnimationFrame(gameLoop2);
};

// gameLoop2();

const gameLoop1 = () => {
  universe.tick();
  drawGrid();
  drawCells();
  requestAnimationFrame(gameLoop1);
};

gameLoop1();
