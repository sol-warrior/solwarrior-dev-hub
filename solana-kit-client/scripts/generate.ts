import { createFromRoot } from 'codama';
import { rootNodeFromAnchor, type AnchorIdl } from '@codama/nodes-from-anchor';
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js';
import path from 'path';
import idl from '../idl.json' with {type: "json"};

const root = createFromRoot(
    rootNodeFromAnchor(idl as AnchorIdl)
);

const outputDir = path.join(process.cwd(), 'src', 'generated');

root.accept(
    renderJavaScriptVisitor(outputDir)
);

console.log('✅ Solana Kit client generated');
