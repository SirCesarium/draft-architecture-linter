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
  // Path to the sweet-lsp binary
  const serverModule = context.asAbsolutePath(
    path.join('..', '..', 'target', 'debug', 'sweet-lsp')
  );

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

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
