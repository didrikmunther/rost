export type BinaryOutput = {
  buffer: number[];
  log: string;
};

export type WabtModule = {
  resolveNames: () => void;
  validate: (features: object) => void;
  toBinary: (options: object) => BinaryOutput;
};

export type Wabt = {
  parseWat: (filename: string, wat: string, features: object) => WabtModule;
  FEATURES: Record<string, boolean>;
};