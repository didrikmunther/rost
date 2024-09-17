import * as path from "path";
import { workspace, ExtensionContext } from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  const rpcScriptModule = context.asAbsolutePath(path.join("..", "target", 'debug', 'lsp'));

  const serverOptions: ServerOptions = {
    command: rpcScriptModule,
    args: [],
    transport: {
      kind: TransportKind.socket,
      port: 8080,
    }
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ language: "plaintext" }],
    // synchronize: {
    //   // Notify the server about file changes to '.clientrc files contained in the workspace
    //   fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
    // },
  };

  console.log("Starting client");

  // Create the language client and start the client.
  client = new LanguageClient(
    "rostLanguageServer",
    "Rost LSP",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }

  console.log("Deactivating client");

  return client.stop();
}
