import { Mandelbrot } from "mandelbrot_wasm";
import { memory } from "mandelbrot_wasm/mandelbrot_wasm_bg"; // Import the WebAssembly memory at the top of the file.

// Construct the plot generator
const width = 500;
const height = 500;
const mandelbrot = Mandelbrot.new(width, height);
var plotDirty = true;

// Give the canvas room for our plot
const canvas = document.getElementById("mandelbrot-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    fps.render();
    if (plotDirty == true) {
        plotDirty = false;
        mandelbrot.plot_generate();
        drawPlot();
    }

    requestAnimationFrame(renderLoop);
};

const drawPlot = () => {
    const plotRgbaPtr = mandelbrot.plot_rgba();
    const plotRgba = new Uint8ClampedArray(memory.buffer, plotRgbaPtr, width * height * 4); // 4 bytes per RGBA
    const imageData = new ImageData(plotRgba, width);
    ctx.putImageData(imageData, 0, 0);
};

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasX = (event.clientX - boundingRect.left) * scaleX;
    const canvasY = (event.clientY - boundingRect.top) * scaleY;

    mandelbrot.view_center(canvasX, canvasY);
    plotDirty = true;
});

canvas.addEventListener("wheel", event => {
    const zoomFactor = 1.0 + event.deltaY / 1000;
    mandelbrot.view_zoom(zoomFactor);
    plotDirty = true;

    return false; // Don't propagate the event, this avoids the page scrolling
});

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

var maxIterationsSlider = document.getElementById("max-iterations-slider");
var maxIterationsValue = document.getElementById("max-iterations-value");
maxIterationsValue.innerHTML = maxIterationsSlider.value; // Display the default slider value

// Update the current slider value (each time you drag the slider handle)
maxIterationsSlider.addEventListener("input", event => {
    maxIterationsValue.innerHTML = event.target.value;
    mandelbrot.max_iterations_set(event.target.value);
    plotDirty = true;
});

// Idea: add a toggle button to choose the color scheme.
// Idea: refactor the rust code to be based on the view center instead of left / top.
// Idea: set max iterations from js on load and don't rely on html default value matching rust's.
// Idea: use f64 instead of f32 to get more resolution! Need to increase max_iterations though.
// Idea: pull dirty flag into rust since it knows best when something changes.

requestAnimationFrame(renderLoop);
