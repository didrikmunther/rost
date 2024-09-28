import Editor from "@monaco-editor/react";

export function MainEditor({
  value,
  onChange,
}: {
  value: string;
  onChange: (value: string) => void;
}) {
  return (
    <>
      <Editor
        height="82vh"
        defaultLanguage="rost"
        value={value}
        onChange={(v) => onChange(v ?? "")}
      />
    </>
  );
}
