// polyfill so deques.js will work
import "./textencoderdecoder.js";
import init, { MusicBox } from "../pkg/deques.js";

// random-noise-processor.js
class CheeseProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.port.onmessage = (event) => this.onmessage(event.data);
        this.player = null;
        this.initialized = false;
    }

    onmessage(event) {
        if (event.type === "send-wasm-module") {
          init(WebAssembly.compile(event.wasmBytes)).then(() => {
            this.port.postMessage({ type: 'wasm-module-loaded' });
          });
        } else if (event.type === 'init-detector') {
          const { sampleRate, noteAtten, isTTAF: isTTFAF, noteFreq } = event;

          this.player = MusicBox.new(sampleRate, noteAtten, isTTFAF, noteFreq);
          this.initialized = true;
          console.log("Player has initialized!");
        }
      };

    process (inputs, outputs, parameters) {
      const output = outputs[0];
      if (this.initialized == false) {
        return true; // just skip this batch of samples, wait for wasm to load
      }
      if (output.length == 0) {
        return false;
      }
      let continuing = this.player.process(output[0]);
      for (let c = 1; c < output.length; c++) {
        for (let i = 0; i < output[0].length; i++) {
            output[c][i] = output[0][i];
        }
      }
      return continuing;
    }
}

registerProcessor('cheese-processor', CheeseProcessor);
