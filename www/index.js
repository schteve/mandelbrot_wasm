import { Mandelbrot } from "mandelbrot_wasm";
import { memory } from "mandelbrot_wasm/mandelbrot_wasm_bg"; // Import the WebAssembly memory at the top of the file.

// Construct the plot generator
const width = 500;
const height = 500;
const mandelbrot = Mandelbrot.new(width, height);

// Give the canvas room for our plot
const canvas = document.getElementById("mandelbrot-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    fps.render();
    //drawPlot();
    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * width + column;
};

const rgbToHex = (rgb) => {
    var hex = Number(rgb).toString(16);
    if (hex.length < 2) {
        hex = "0" + hex;
    }
    return hex;
}

const getRGB = (value) => {
    const r = ((value & 0x03) >> 0) * (256 / 4);
    const g = ((value & 0x0C) >> 2) * (256 / 4);
    const b = ((value & 0x30) >> 4) * (256 / 4);
    return "#" + rgbToHex(r) + rgbToHex(g) + rgbToHex(b);
};

const drawPlot = () => {
    const plotPtr = mandelbrot.plot_data();
    const plot = new Uint8Array(memory.buffer, plotPtr, width * height);

    // var myImageData = ctx.createImageData(width, height);
    // Uint8ClampedArray.from(plot);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            var rgb;
            if (plot[idx] >= mandelbrot.max_iterations()) {
                rgb = "#000000";
            } else {
                rgb = getRGB(plot[idx]);
            }
            ctx.fillStyle = rgb;

            ctx.fillRect(col, row, 1, 1);
        }
    }
};

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};

mandelbrot.plot_generate();
drawPlot();
requestAnimationFrame(renderLoop);
