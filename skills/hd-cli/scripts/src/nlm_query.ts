import { spawnSync } from 'node:child_process';
import { die, parseArgs } from './utils';

export function checkLogin() {
  const r = spawnSync('nlm', ['login', '--check'], { encoding: 'utf8' });
  return r.status === 0;
}

export function queryNotebook(opts: { notebookId: string; requestText: string; conversationId?: string | null }) {
  const argv = ['notebook', 'query', opts.notebookId, opts.requestText];
  if (opts.conversationId) argv.push('--conversation-id', opts.conversationId);
  const r = spawnSync('nlm', argv, { encoding: 'utf8' });
  return { exitCode: r.status ?? 1, stdout: r.stdout || '', stderr: r.stderr || '' };
}

export async function main(argv: string[]) {
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
  main(process.argv.slice(2)).catch((e) => die((e as any)?.stack || (e as any)?.message || String(e)));
}

