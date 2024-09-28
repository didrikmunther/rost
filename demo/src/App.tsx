import { useEffect, useState } from "react";
import "./App.css";
import WabtModule from "./libwabt";
import { MainEditor } from "./MainEditor";
import { Wabt } from "./types";
import { RostCompiler } from "./RostCompiler";

const defaultProgram = `__builtin fn print(content: str);
__builtin fn printi(content: int);

fn print_two_strings(a: str, b: str) {
	let c = a + b;

	print(c);
}

fn print_two_things(a: int, b: int) {
	let c = a + b;

	printi(c);
}

fn main() {
	print_two_things(3, 4);
	print_two_strings("abc", "def");
}`;

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
        <RostCompiler code={value} wabt={wabt} features={features} />
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
