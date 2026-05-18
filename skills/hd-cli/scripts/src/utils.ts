import fs from 'node:fs';
import path from 'node:path';

export type ParsedArgs = {
  args: string[];
  flags: Record<string, string | boolean>;
};

export function parseArgs(argv: string[]): ParsedArgs {
  const args: string[] = [];
  const flags: Record<string, string | boolean> = {};

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

export function die(message: unknown, code = 1): never {
  process.stderr.write(String(message).trimEnd() + '\n');
  process.exit(code);
}

export function ensureDir(dirPath: string) {
  fs.mkdirSync(dirPath, { recursive: true });
}

export function writeFileAtomic(filePath: string, content: string) {
  const dir = path.dirname(filePath);
  ensureDir(dir);
  const tmp = path.join(dir, `.tmp-${path.basename(filePath)}-${process.pid}-${Date.now()}`);
  fs.writeFileSync(tmp, content);
  fs.renameSync(tmp, filePath);
}

