#!/usr/bin/env zx
import 'zx/globals';
import { createFromRoot } from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js';
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust';

const workingDirectory = (await $`pwd`.quiet()).toString().trim();

// Instanciate Codama.
const Codama = createFromRoot(
  rootNodeFromAnchor(require(path.join(workingDirectory, 'idl', 'myproject.json')))
);

// Render JavaScript.
const jsClient = path.join(__dirname, '..', 'clients', 'js');
Codama.accept(
  renderJavaScriptVisitor(path.join(jsClient, 'src', 'generated'), {
    prettier: require(path.join(jsClient, '.prettierrc')),
  })
);

// Render Rust.
const rustClient = path.join(__dirname, '..', 'clients', 'rust');
Codama.accept(
  renderRustVisitor(path.join(rustClient, 'src', 'generated'), {
    formatCode: true,
    crateFolder: rustClient,
    deleteFolderBeforeRendering: true,
  })
);
