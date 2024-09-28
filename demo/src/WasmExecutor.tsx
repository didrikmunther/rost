import { useState } from "react";
import { Wabt } from "./types";

type MainWasmObject = {
  __main: () => void;
};

const batch_size = 128;
const max_until_overflow = 1024;

export function WasmExecutor({
  features,
  value,
  wabt,
}: {
  features: Record<string, boolean>;
  value: string;
  wabt: Wabt;
}) {
  const memory = new WebAssembly.Memory({ initial: 1 });
  const [rows, setRows] = useState<string[]>([]);
  const [isExecuting, setIsExecuting] = useState(false);

  const printi = (integer: number) => {
    setRows((prev) => [...prev, integer.toString()]);
  };

  const print = (offset: number) => {
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

    setRows((prev) => [...prev, string]);
  };

  const onExecute = async () => {
    if (isExecuting) {
      return;
    }

    setIsExecuting(true);
    setRows([]);

    const importObject = {
      imports: {
        print,
        printi,
      },
      js: {
        mem: memory,
      },
    };

    const module = wabt.parseWat("test.wat", value, {});
    module.resolveNames();
    module.validate(features);
    const binaryOutput = module.toBinary({
      log: true,
      write_debug_names: true,
    });

    WebAssembly.instantiate(
      new Uint8Array(binaryOutput.buffer),
      importObject
    ).then((obj) => {
      (obj.instance.exports as MainWasmObject).__main();
      setIsExecuting(false);
    });
  };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "1rem",
      }}
    >
      <div>
        <button
          style={{
            backgroundColor: "rgb(66 66 66)",
          }}
          onClick={onExecute}
          disabled={isExecuting}
        >
          {isExecuting ? "Executing..." : "Execute"}
        </button>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "0.5rem",
          justifyContent: "flex-start",
          width: "300px",
          padding: "0.5rem",
          borderRadius: "0.25rem",
          background: "rgb(131 131 131)",
        }}
      >
        {rows.map((row, index) => (
          <div
            style={{
              textAlign: "left",
              background: "rgb(66 66 66)",
              padding: "0.25rem",
              borderRadius: "0.25rem",
            }}
            key={index}
          >
            {row}
          </div>
        ))}
      </div>
    </div>
  );
}
