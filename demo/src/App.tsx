import { useEffect, useState } from "react";
import "./App.css";
import WabtModule from "./libwabt";
import { MainEditor } from "./MainEditor";
import { Wabt } from "./types";
import { WasmExecutor } from "./WasmExecutor";

// const defaultProgram = `__builtin fn print(content: str);
// __builtin fn printi(content: int);

// fn print_two_strings(a: str, b: str) {
// 	let c = a + b;

// 	print(c);
// }

// fn print_two_things(a: int, b: int) {
// 	let c = a + b;

// 	printi(c);
// }

// fn main() {
// 	print_two_things(3, 4);
// 	print_two_strings("abc", "def");
// }`;

const defaultProgram = `
  	;; boilerplate_entry.wat begin

(module
    ;; Imports from JavaScript namespace
    (import  "imports"  "print" (func  $print (param  i32))) ;; Import print function, print string
    (import  "imports"  "printi" (func  $printi (param  i32))) ;; Import printi function, print integer
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)

    ;; boilerplate_entry.wat end
	;; Global data section begin
	(data (i32.const 0) "def")
	(data (i32.const 4) "abc")

	;; Function definitions begin
	;; User function: print_two_things
	(func $__userf__9
		(param $_6_b i32)
		(param $_5_a i32)
		(local $_7_c i32)
		(local $_8_printi i32)
		local.get $_6_b
		local.get $_5_a
		i32.add
		local.set $_7_c
		local.get $_7_c
		;; Builtin call: printi
		call $printi

	)

	;; User function: print_two_strings
	(func $__userf__14
		(param $_11_b i32)
		(param $_10_a i32)
		(local $_13_print i32)
		(local $_12_c i32)
		local.get $_11_b
		local.get $_10_a
		i32.add
		local.set $_12_c
		local.get $_12_c
		;; Builtin call: print
		call $print

	)

	;; User function: main
	(func $__userf__15
		(local $_14_print_two_strings i32)
		(local $_9_print_two_things i32)
		i32.const 4
		i32.const 3
		;; Procedure call: print_two_things
		call $__userf__9
		i32.const 0
		i32.const 4
		;; Procedure call: print_two_strings
		call $__userf__14

	)


	(func $main

		call $__userf__15
	)

	(func $__main
		call $main	)
	(export "__main" (func $__main))
	
;; boilerplate_exit.wat begin
	)
	;; boilerplate_exit.wat end

`;

function AppWithWabt({ wabt }: { wabt: Wabt }) {
  const [features, setFeatures] = useState<Record<string, boolean>>({});
  const [value, setValue] = useState<string>(defaultProgram);

  useEffect(() => {
    setFeatures(
      Object.fromEntries(Object.entries(wabt.FEATURES).map(([f]) => [f, false]))
    );
  }, [wabt]);

  const onChange = (value: string) => {
    setValue(value);
  };

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        gap: "1rem",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: "1rem",
        }}
      >
        <div>
          <button>Hello world</button>
        </div>
        <MainEditor value={value} onChange={onChange} />
      </div>
      <div>
        <WasmExecutor features={features} value={value} wabt={wabt} />
      </div>
    </div>
  );
}

function App() {
  const [wabt, setWabt] = useState<Wabt>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    WabtModule().then((wabt: Wabt) => {
      setWabt(wabt);
      setIsLoading(false);
    });
  }, []);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (!wabt) {
    return <div>Failed to load Wabt</div>;
  }

  return <AppWithWabt wabt={wabt} />;
}

export default App;
