#!/usr/bin/env node
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const { parseArgs, die } = require('./utils');

function getDefaultCacheRoot() {
  return path.join(os.homedir(), '.cache', 'hd-cli');
}

function resolveHdCliBin(flags) {
  if (process.env.HD_CLI_BIN) return process.env.HD_CLI_BIN;
  if (flags['hd-cli-bin']) return String(flags['hd-cli-bin']);

  const cacheRoot = String(flags['cache-root'] || getDefaultCacheRoot());
  const current = path.join(cacheRoot, 'current', process.platform === 'win32' ? 'hd-cli.exe' : 'hd-cli');
  if (fs.existsSync(current)) return current;

  return 'hd-cli';
}

function runHdCli({ bin, argv }) {
  const r = spawnSync(bin, argv, { encoding: 'utf8' });
  return { exitCode: r.status ?? 1, stdout: r.stdout || '', stderr: r.stderr || '' };
}

async function main(argv) {
  const { args, flags } = parseArgs(argv);
  const bin = resolveHdCliBin(flags);

  if (flags.structured) {
    const date = flags.date ? String(flags.date) : null;
    const time = flags.time ? String(flags.time) : null;
    const utc = flags.utc ? String(flags.utc) : null;
    const lang = flags.lang ? String(flags.lang) : null;
    const short = Boolean(flags.short);
    const format = flags.format ? String(flags.format) : 'json';

    if (!date || !time || !utc) die('structured requires --date, --time, --utc');
    const call = ['--date', date, '--time', time, '--utc', utc, '--format', format];
    if (lang) call.push('--lang', lang);
    if (short) call.push('--short');

    const r = runHdCli({ bin, argv: call });
    if (r.exitCode !== 0) die(r.stderr || r.stdout || `hd-cli exited with ${r.exitCode}`, r.exitCode);
    process.stdout.write(r.stdout);
    return;
  }

  if (args.length === 0) die('Usage: hd-cli-skill-run [--structured ...] -- <hd-cli args...>');

  const r = runHdCli({ bin, argv: args });
  if (r.exitCode !== 0) {
    process.stderr.write(r.stderr || '');
    process.exit(r.exitCode);
  }
  process.stdout.write(r.stdout);
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((e) => die(e?.stack || e?.message || String(e)));
}

module.exports = { resolveHdCliBin, runHdCli, main };

