import * as path from 'node:path';
import { workspace, type ExtensionContext } from 'vscode';
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Path to the sweet-lsp binary (absolute path is safer)
  const serverModule = path.resolve(
    context.extensionPath,
    '..',
    '..',
    'target',
    'debug',
    'sweet-lsp'
  );

  console.log(`[Sweet] Searching for LSP binary at: ${serverModule}`);

  const serverOptions: ServerOptions = {
    run: {
      command: serverModule,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverModule,
      transport: TransportKind.stdio,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'rust' },
      { scheme: 'file', language: 'python' },
      { scheme: 'file', language: 'javascript' },
      { scheme: 'file', language: 'typescript' },
      { scheme: 'file', language: 'java' },
      { scheme: 'file', language: 'csharp' },
    ],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher('**/.swtrc'),
    },
  };

  client = new LanguageClient(
    'sweet',
    'Sweet LSP Server',
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start().catch((err) => {
    console.error(`[Sweet] Failed to start LSP client: ${err}`);
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
