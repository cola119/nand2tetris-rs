type MyIncomingMessage = {
  register_index: number;
  x: number;
  y: number;
  input: string;
};

const drawCanvas = (
  ctx: CanvasRenderingContext2D,
  { x, y, input }: MyIncomingMessage
) => {
  const imageData = ctx.createImageData(16, 1);
  for (let i = 0; i < imageData.data.length; i += 4) {
    const digit = i / 4;
    const isFill = input[digit] === "1";
    imageData.data[i] = 0;
    imageData.data[i + 1] = 0;
    imageData.data[i + 2] = 0;
    imageData.data[i + 3] = isFill ? 255 : 0;
  }
  ctx.putImageData(imageData, x, y);
};

const main = () => {
  const canvas = document.getElementById(
    "my_canvas"
  ) as HTMLCanvasElement | null;
  if (!canvas) return;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const ws = new WebSocket("ws://127.0.0.1:9001");
  ws.addEventListener("open", (e) => {
    console.log("open");
  });
  ws.addEventListener("message", (e) => {
    const parsed = JSON.parse(e.data) as MyIncomingMessage;
    console.log("message", parsed);
    drawCanvas(ctx, parsed);
  });
  ws.addEventListener("error", (e) => {
    console.log("error");
  });
  ws.addEventListener("close", (e) => {
    console.log("close");
  });
};

window.addEventListener("DOMContentLoaded", (event) => {
  main();
});
