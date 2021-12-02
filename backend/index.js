import css from "./style.css";
const rust = import("./pkg/x_weld");
const ui = require("../frontend/UserInterface/terminal");


let terminal = ui.terminal();
terminal.log("hello world!");

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

const gl = canvas.getContext('webgl',{antialias: false});
const FPS = 120.0;
var frames = 0;

rust.then(m => {
  if(!gl){
    alert("WebGL initialisation has failed!");
    return;
  }

  const rustClient = new m.Client();
  const start = performance.now();

  function render(){
    window.requestAnimationFrame(render);
    const end = performance.now();
    let elapsed = (end - start)/1000;
    if((elapsed > frames * (1/FPS)))
    {

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
      rustClient.update(elapsed, canvas.height, canvas.width);
      rustClient.draw(canvas.height, canvas.width);
      frames++;
    }
  }

  render();
})
