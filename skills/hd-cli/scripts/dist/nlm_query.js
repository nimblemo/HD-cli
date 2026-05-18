#!/usr/bin/env node
const { spawnSync } = require('node:child_process');

const { parseArgs, die } = require('./utils');

function checkLogin() {
  const r = spawnSync('nlm', ['login', '--check'], { encoding: 'utf8' });
  return r.status === 0;
}

function queryNotebook({ notebookId, requestText, conversationId }) {
  const argv = ['notebook', 'query', notebookId, requestText];
  if (conversationId) argv.push('--conversation-id', conversationId);
  const r = spawnSync('nlm', argv, { encoding: 'utf8' });
  return { exitCode: r.status ?? 1, stdout: r.stdout || '', stderr: r.stderr || '' };
}

async function main(argv) {
  const { flags } = parseArgs(argv);
  const notebookId = String(flags['notebook-id'] || 'c5dd30c7-da41-49a5-a0ce-74ed7ad7ce1b');
  const requestText = flags['request-text'] ? String(flags['request-text']) : null;
  const conversationId = flags['conversation-id'] ? String(flags['conversation-id']) : null;

  if (!requestText) die('nlm-query requires --request-text "<text>"');
  if (!checkLogin()) die('NotebookLM не авторизован. Выполни: nlm login');

  const r = queryNotebook({ notebookId, requestText, conversationId });
  if (r.exitCode !== 0) die(r.stderr || r.stdout || `nlm exited with ${r.exitCode}`, r.exitCode);
  process.stdout.write(r.stdout);
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((e) => die(e?.stack || e?.message || String(e)));
}

module.exports = { checkLogin, queryNotebook, main };

