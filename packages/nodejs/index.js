// ByteRAG Native Node.js bindings
const fs = require('fs');
const path = require('path');

function findBinding() {
    // 1. Direct search in current dir
    const isNode = f => f.endsWith('.node');
    const localNodes = fs.existsSync(__dirname) ? fs.readdirSync(__dirname).filter(isNode) : [];
    if (localNodes.length > 0) {
        return path.join(__dirname, localNodes[0]);
    }

    // 2. Search target/debug or target/release
    const targetDirs = [
        path.join(__dirname, '../../target/debug'),
        path.join(__dirname, '../../target/release')
    ];
    for (const tDir of targetDirs) {
        if (fs.existsSync(tDir)) {
            const files = fs.readdirSync(tDir).filter(f => f.includes('byterag_node') || f.includes('byterag-node'));
            for (const f of files) {
                if (f.endsWith('.node') || f.endsWith('.so') || f.endsWith('.dylib') || f.endsWith('.dll')) {
                    try {
                        return path.join(tDir, f);
                    } catch (_) {}
                }
            }
        }
    }

    const { platform, arch } = process;
    const candidates = [
        `byterag-node.${platform}-${arch}.node`,
        `byterag-node.${platform}-${arch}-gnu.node`,
        `byterag-node.${platform}-${arch}-msvc.node`,
        `byterag-node.node`,
        `dbx-native.node`,
        `index.node`
    ];

    for (const cand of candidates) {
        const p = path.join(__dirname, cand);
        if (fs.existsSync(p)) return p;
    }

    return null;
}

const bindingPath = findBinding();
if (!bindingPath) {
    throw new Error(`Could not find byterag-node native binding for ${process.platform}-${process.arch}`);
}

module.exports = require(bindingPath);
