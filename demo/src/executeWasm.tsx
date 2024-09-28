type MainWasmObject = {
  __main: () => void;
};

const batch_size = 128;
const max_until_overflow = 1024;

export async function executeWasm(wasm: Uint8Array) {
  const memory = new WebAssembly.Memory({ initial: 1 });

  function printi(integer: number) {
    console.log(integer);
  }

  function print(offset: number) {
    let string = "";
    let new_offset = offset;
    let finished = false;

    while (!finished && new_offset - offset < max_until_overflow) {
      const new_bytes = new Uint8Array(memory.buffer, new_offset, batch_size);
      const new_string = new TextDecoder("utf8").decode(new_bytes);

      for (const char of new_string) {
        if (char == "\0") {
          finished = true;
          break;
        }

        string += char;
      }

      new_offset += batch_size;
    }

    // process.stdout.write(string);

    console.log(string);
  }

  const importObject = {
    imports: {
      print,
      printi,
    },
    js: {
      mem: memory,
    },
  };

  WebAssembly.instantiate(wasm, importObject).then((obj) => {
    (obj.instance.exports as MainWasmObject).__main();
  });
}
