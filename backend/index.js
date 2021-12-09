import css from "../frontend/UserInterface/style.css";
const rust = import("./pkg/x_weld");
const ui = require("../frontend/UserInterface/terminal");

let sim = document.createElement("div");
document.body.appendChild(sim);
sim.setAttribute("id","sim");
sim.style.position = "fixed";
sim.style.top = "2.5%";
sim.style.left = "2.5%";
sim.style.width = "70%";
sim.style.height = "95%";

let canvas = document.createElement("canvas");
canvas.width = sim.clientWidth;
canvas.height = sim.clientHeight;
canvas.id = "canvas";
canvas.style.cursor="crosshair";
canvas.style.borderColor = "black";
canvas.style.borderStyle = "solid";
sim.appendChild(canvas);

let graphics = {
  start:0,
  fps: 120.,
  frames:0,

  reset: false,
  client: {},
};

var gl = canvas.getContext('webgl',{antialias: true});
rust.then(m => {
  if(!gl){
    alert("WebGL initialisation has failed!");
    return;
  }

graphics.client = new m.Client();
let terminal = ui.terminal();
terminal.log("hello world!");
terminal.bindGraphics(graphics);

graphics.start = performance.now();
  function render(){

    let now = performance.now();
    let elapsed = (now - graphics.start)/1000;
    graphics.client.update(elapsed, canvas.height, canvas.width);
    window.requestAnimationFrame(render);
    if((elapsed > graphics.frames * (1/graphics.fps))){

      /*
      if(window.innerHeight != canvas.height || window.innerWidth != canvas.width){

        canvas.height = window.innerHeight-700;
        canvas.clientHeight = window.innerHeight-700;
        canvas.style.height = window.innerHeight-700;

        canvas.width = window.innerWidth-700;
        canvas.clientWidth = window.innerWidth-700;
        canvas.style.width = window.innerWidth-700;

        gl.viewport(0,0, canvas.width, canvas.height);
      }
*/
      if (graphics.reset){
        graphics.client = new m.Client();
        graphics.reset = false;
      }
      graphics.client.draw(canvas.height, canvas.width);
      graphics.frames++;
    }
  }
  render();
})
