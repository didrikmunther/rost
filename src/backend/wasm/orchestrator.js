const fs = require("fs");

const args = process.argv.slice(2);
if (args.length <= 0) {
  console.error("Usage: node boilerplate.js <name>");
  process.exit(1);
}

const wasmFile = args[0];

let memory = new WebAssembly.Memory({ initial: 1 });

let batch_size = 128;
let max_until_overflow = 1024;

function print(offset) {
  let string = "";
  let new_offset = offset;
  let finished = false;

  while (!finished && new_offset - offset < max_until_overflow) {
    let new_bytes = new Uint8Array(memory.buffer, new_offset, batch_size);
    let new_string = new TextDecoder("utf8").decode(new_bytes);

    for (let char of new_string) {
      if (char == "\0") {
        finished = true;
        break;
      }

      string += char;
    }

    new_offset += batch_size;
  }

  process.stdout.write(string);
}

var importObject = {
  imports: {
    print,
  },
  js: {
    mem: memory,
  },
};

const helloWasm = fs.readFileSync(wasmFile);
WebAssembly.instantiate(new Uint8Array(helloWasm), importObject).then((obj) => {
  obj.instance.exports.__main();
});
