import { RayTracer } from "raytracer";
import { memory } from "raytracer/raytracer_bg";

(function () {
  const WIDTH = 256;
  const HEIGHT = 256;
  const CHANNELS = 4;

  const canvas = document.getElementById("canvas");
  canvas.width = WIDTH;
  canvas.height = HEIGHT;

  const ctx = canvas.getContext("2d");

  const rayTracer = RayTracer.new(WIDTH, HEIGHT);

  let screenPtr;
  let screen;
  let imgData = ctx.createImageData(WIDTH, HEIGHT);

  function drawImage() {
    if (screenPtr === undefined) {
      screenPtr = rayTracer.screen();
    }

    if (screen === undefined) {
      screen = new Uint8Array(
        memory.buffer,
        screenPtr,
        WIDTH * HEIGHT * CHANNELS
      );
    }

    imgData.data.set(screen);

    ctx.putImageData(imgData, 0, 0, 0, 0, WIDTH, HEIGHT);
  }

  function render() {
    requestAnimationFrame(render);

    rayTracer.render();

    drawImage();
  }

  requestAnimationFrame(render);
})();
