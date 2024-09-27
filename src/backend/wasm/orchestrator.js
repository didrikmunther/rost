const fs = require("fs");

const args = process.argv.slice(2);
if (args.length <= 0) {
  console.error("Usage: node boilerplate.js <name>");
  process.exit(1);
}

const wasmFile = args[0];

var memory = new WebAssembly.Memory({ initial: 1 });

function print(offset, length) {
  var bytes = new Uint8Array(memory.buffer, offset, length);
  var string = new TextDecoder("utf8").decode(bytes);
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
  obj.instance.exports.main();
});
