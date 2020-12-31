const main = () => {
  const ws = new WebSocket("ws://127.0.0.1:9001");
  ws.addEventListener("open", (e) => {
    console.log("open");
  });
  ws.addEventListener("message", (e) => {
    console.log("message");
  });
  ws.addEventListener("error", (e) => {
    console.log("error");
  });
  ws.addEventListener("close", (e) => {
    console.log("close");
  });
};

main();
