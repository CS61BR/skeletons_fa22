import init, { Game } from "../pkg/game2048.js";

let seed_string;
let game;
let requested_frame = false;
let canvas;
let ctx;

export function set_canvas_size(w, h) {
  canvas.width = w;
  canvas.height = h;
}

export function request_animation_frame() {
  if (!requested_frame) {
    requested_frame = true;
    window.requestAnimationFrame(() => {
      requested_frame = false;
      game.draw_animation_frame();
    });
  }
}

export function draw_rectangle(x, y, w, h, color) {
  ctx.fillStyle = color;
  ctx.fillRect(x, y, w, h);
}

export function draw_text(text, x, y, color, font) {
  ctx.fillStyle = color;
  ctx.font = font;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(text, x, y);
}

export function set_score_text(text) {
  document.getElementById("score").innerText = text;
}

export function start() {
  init().then(() => {
      let wi = document.getElementById("width-inp");
      let hi = document.getElementById("height-inp")
      canvas = document.getElementById("main-canvas");
      ctx = canvas.getContext("2d");
      seed_string = "" + new Date().getTime() + Math.random();
      game = Game.new(parseInt(wi.value), parseInt(hi.value), seed_string);
      
      window.addEventListener("keydown", (e) => {
        game.handle_keypress(e.code);
      });

      let reset = () => {
        game.reset(parseInt(wi.value), parseInt(hi.value));
      };
      document.getElementById("new-game-btn").onclick = reset;
      document.getElementById("width-inp").onchange = reset;
      document.getElementById("height-inp").onchange = reset;
  });
}

