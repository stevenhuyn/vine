#!/bin/bash

# Compiling the VSCode extension
cd lsp/client
npm i
npx vsce package --skip-license --allow-missing-repository --out ./dist/vine-vscode.vsix