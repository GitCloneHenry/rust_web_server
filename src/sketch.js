let canvas = document.createElement("canvas");
let ctx = canvas.getContext("2d");

canvas.width = 400;
canvas.height = 400;

document.body.appendChild(canvas);

ctx.fillStyle = "rgb(255,255,0)";
ctx.fillRect(20,20,20,20);
