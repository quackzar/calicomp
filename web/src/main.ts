import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebLinksAddon } from '@xterm/addon-web-links';
import { CanvasAddon } from "@xterm/addon-canvas";
import { AttachAddon } from "@xterm/addon-attach";


const term = new Terminal({
  fontFamily: '"DejaVu Sans Mono", "Everson Mono", FreeMono, Menlo, Terminal, monospace, "Apple Symbols"',
});
const fitAddon = new FitAddon();
const socket = new WebSocket("ws://localhost:1111");


term.loadAddon(fitAddon);
term.loadAddon(new WebLinksAddon());
term.loadAddon(new CanvasAddon());

term.open(document.getElementById("terminal")!);

fitAddon.fit();

window.addEventListener('resize', () => {
  console.log('resize');
  fitAddon.fit();
});


socket.onopen = () => {
    socket.send(JSON.stringify({
        cols: term.cols,
        rows: term.rows,
    }));

    term.loadAddon(
        new AttachAddon(socket, {
            bidirectional: true,
        }),
    );

    term.onResize((event) => {
        const packet = new Uint8Array(3);

        packet[0] = 0x04;
        packet[1] = Math.min(event.cols, 255);
        packet[2] = Math.min(event.rows, 255);

        socket.send(packet);
    });

    term.focus();
};
