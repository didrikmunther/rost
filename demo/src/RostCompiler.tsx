import { useEffect, useRef, useState } from "react";
import { Wabt } from "./types";
import { WasmExecutor } from "./WasmExecutor";
import { compile } from "./rost/rost_wasm";

type MainWasmObject = {
  __main: () => void;
};

const batch_size = 128;
const max_until_overflow = 1024;
const memory = new WebAssembly.Memory({ initial: 1 });

const useWasmWrapper = (
  wabt: Wabt,
  features: Record<string, boolean>,
  code: string
) => {
  const [rows, setRows] = useState<string[]>([]);
  const controller = useRef(new AbortController());

  useEffect(() => {
    controller.current.abort();
    const newController = new AbortController();
    controller.current = newController;

    setRows([]);

    const ignoreGuard =
      <T extends unknown[], R>(fn: (...args: T) => R) =>
      (...args: T) => {
        if (newController.signal.aborted) {
          return;
        }

        fn(...args);
      };

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

    const importObject = {
      imports: {
        print: ignoreGuard(print),
        printi: ignoreGuard(printi),
      },
      js: {
        mem: memory,
      },
    };

    const module = wabt.parseWat("test.wat", code, {});
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
      if (newController.signal.aborted) {
        return;
      }

      (obj.instance.exports as MainWasmObject).__main();
    });
  }, [code, features, wabt]);

  return rows;
};

export const WasmCompiler = ({
  wabt,
  features,
  compiled,
}: {
  wabt: Wabt;
  features: Record<string, boolean>;
  compiled: string;
}) => {
  const rows = useWasmWrapper(wabt, features, compiled);

  return <WasmExecutor rows={rows} />;
};

export function RostCompiler({
  code,
  wabt,
  features,
}: {
  code: string;
  wabt: Wabt;
  features: Record<string, boolean>;
}) {
  const [compiled, setCompiled] = useState<string | undefined>();
  const [error, setError] = useState<string | undefined>();

  const onExecute = async () => {
    try {
      setError(undefined);
      setCompiled(compile(code));
    } catch (e) {
      setCompiled(undefined);
      setError(e as string);
    }
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
        >
          Compile
        </button>
      </div>
      <div>
        {error && (
          <div
            style={{
              color: "red",
            }}
          >
            {error}
          </div>
        )}
        {compiled && (
          <WasmCompiler wabt={wabt} features={features} compiled={compiled} />
        )}
      </div>
    </div>
  );
}
