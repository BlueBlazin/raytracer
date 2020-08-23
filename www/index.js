import { Scene } from "raytracer";
import { memory } from "raytracer/raytracer_bg";

(function () {
  const ASPECT_RATIO = 16.0 / 9.0;
  const WIDTH = 400;
  const HEIGHT = Math.floor(WIDTH / ASPECT_RATIO);
  const CHANNELS = 4;

  const canvas = document.getElementById("canvas");
  canvas.width = WIDTH;
  canvas.height = HEIGHT;

  const ctx = canvas.getContext("2d");

  const scene = Scene.new(WIDTH);

  let screenPtr;
  let screen;
  let imgData = ctx.createImageData(WIDTH, HEIGHT);

  function drawImage() {
    if (screenPtr === undefined) {
      screenPtr = scene.screen();
    }

    if (screen === undefined) {
      screen = new Uint8Array(
        memory.buffer,
        screenPtr,
        WIDTH * HEIGHT * CHANNELS
      );

      console.log(screen);
    }

    imgData.data.set(screen);

    ctx.putImageData(imgData, 0, 0, 0, 0, WIDTH, HEIGHT);
  }

  function render() {
    requestAnimationFrame(render);
    drawImage();
  }

  scene.render();

  requestAnimationFrame(render);
})();
