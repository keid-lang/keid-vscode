import * as libkls from '../libkls/pkg';
import * as fs from 'fs';

export interface Position {
    character: number;
    line: number;
}

function toPoint(value: string, offset: number): Position {
    const previous = value.substring(0, offset);
    const lineNumber = (previous.match(/\r?\n/g) ?? []).length;
    const lastLine = previous.substring(previous.lastIndexOf('\n'));
    const characterNumber = lastLine.length;

    console.log(`offset = ${offset}`);
    console.log(`previous = '${previous}'`);
    console.log(`lastLine = '${lastLine}'`);

    return {
        line: lineNumber,
        character: characterNumber,
    };
}

export class KlsContext {
    pointer: number;

    constructor() {
        this.pointer = libkls.libkls_create_context();
    }

    get error(): string | null {
        console.log('last error = ' + libkls.libkls_get_last_error(this.pointer));
        return libkls.libkls_get_last_error(this.pointer);
    }

    includeFile(path: string, contents: string) {
        if (libkls.libkls_include_file(this.pointer, path, contents)) {
            throw new Error(this.error);
        }
    }

    compile(): {
        message: string;
        range: {
            start: Position;
            end: Position;
        };
    }[] {
        const errors = libkls.libkls_compile(this.pointer);
        console.log(errors);

        if (errors) {
            const results = [];
            for (const error of errors) {
                const contents = fs.readFileSync(new URL(error.file), 'utf8');
                const start = toPoint(contents, error.range.start);
                const end = toPoint(contents, error.range.end);

                results.push({
                    message: error.message,
                    range: {
                        start: {
                            character: start.character - 1,
                            line: start.line,
                        },
                        end,
                    },
                });
            }

            return results;
        }

        return [];
    }

    dispose() {
        libkls.libkls_dispose_context(this.pointer);
    }
}
