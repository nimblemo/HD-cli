const fs = require('node:fs');
const path = require('node:path');

function parseArgs(argv) {
  const args = [];
  const flags = {};

  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (!a.startsWith('--')) {
      args.push(a);
      continue;
    }

    const eq = a.indexOf('=');
    if (eq !== -1) {
      const k = a.slice(2, eq);
      const v = a.slice(eq + 1);
      flags[k] = v;
      continue;
    }

    const k = a.slice(2);
    const next = argv[i + 1];
    if (next && !next.startsWith('--')) {
      flags[k] = next;
      i++;
      continue;
    }

    flags[k] = true;
  }

  return { args, flags };
}

function die(message, code = 1) {
  process.stderr.write(String(message).trimEnd() + '\n');
  process.exit(code);
}

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function writeFileAtomic(filePath, content) {
  const dir = path.dirname(filePath);
  ensureDir(dir);
  const tmp = path.join(dir, `.tmp-${path.basename(filePath)}-${process.pid}-${Date.now()}`);
  fs.writeFileSync(tmp, content);
  fs.renameSync(tmp, filePath);
}

module.exports = {
  parseArgs,
  die,
  ensureDir,
  writeFileAtomic,
};

