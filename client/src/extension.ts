import * as path from 'path';
import { ExtensionContext, StatusBarAlignment, StatusBarItem, window, workspace } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;
let serverStatus: StatusBarItem;

export function activate(context: ExtensionContext) {
    const serverModule = context.asAbsolutePath(path.join('server', 'out', 'server.js'));
    const serverOptions: ServerOptions = {
        run: {
            module: serverModule,
            transport: TransportKind.ipc,
        },
        debug: {
            module: serverModule,
            transport: TransportKind.ipc,
        }
    };
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{
            scheme: 'file',
            language: 'keid'
        }],
        synchronize: {
            fileEvents: [
                workspace.createFileSystemWatcher('**/*.keid'),
                workspace.createFileSystemWatcher('**/clust.yaml')
            ]
        }
    };

    serverStatus = window.createStatusBarItem(StatusBarAlignment.Left, 0);
    serverStatus.text = 'KLS: Starting...';
    serverStatus.show();

    client = new LanguageClient('kls', 'Keid Language Server', serverOptions, clientOptions);
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
