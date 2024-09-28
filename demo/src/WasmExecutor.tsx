export function WasmExecutor({ rows }: { rows: string[] }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: "1rem",
      }}
    >
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
