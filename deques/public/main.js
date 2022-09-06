
class CheeseNode extends AudioWorkletNode {
    init(wasmBytes, sampleRate, noteAtten, isTTAF, noteFreq) {
        this.sampleRate = sampleRate;
        this.noteAtten = noteAtten;
        this.isTTAF = isTTAF;
        this.noteFreq = noteFreq;

        // Listen to messages sent from the audio processor.
        this.port.onmessage = (event) => this.onmessage(event.data);
        this.port.postMessage({
        type: "send-wasm-module",
        wasmBytes,
        });
    }
    onprocessorerror(err) {
        console.log(`error from AudioWorkletProcessor.process(): ${err}`);
    };

    onmessage(event) {
        if (event.type === 'wasm-module-loaded') {
        // The Wasm module was successfully sent to the PitchProcessor running on the
        // AudioWorklet thread and compiled.
        this.port.postMessage({
            type: "init-detector",
            sampleRate: this.sampleRate,
            noteAtten: this.noteAtten,
            isTTAF: this.isTTAF,
            noteFreq: this.noteFreq
        });
        }
    }
}

export default async function setup(buttons) {
    const response = await window.fetch("../pkg/deques_bg.wasm");
    const wasmBytes = await response.arrayBuffer();

    let handlerFactory = (att, isTTAF, hz, callback) => {
        return async () => {
            const audioContext = new AudioContext();
            await audioContext.audioWorklet.addModule('./public/processor.js');
            let cheese = new CheeseNode(audioContext, 'cheese-processor');
            cheese.init(wasmBytes, audioContext.sampleRate, att, isTTAF, hz);
            cheese.connect(audioContext.destination);
            console.log("should be noisy");
            callback();
        }
    }

    for(let a = 0; a < buttons.length; a++) {
        let b = buttons[a];
        document.getElementById(b[0]).onclick = 
            handlerFactory(b[1], b[2], b[3], b[4]);
    }
}