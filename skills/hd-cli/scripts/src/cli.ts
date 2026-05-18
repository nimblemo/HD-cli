import { die } from './utils';

function help() {
  return [
    'hd-cli-skill (scripts via npx)',
    '',
    'Usage:',
    '  hd-cli-skill <command> [options]',
    '',
    'Commands:',
    '  install        Download latest hd-cli release into cache and print active binary path',
    '  run            Run hd-cli (raw args or --structured)',
    '  compact        Convert hd-cli JSON output into compact JSON (stdin or --input)',
    '  nlm-query      Run NotebookLM query via notebooklm-cli (nlm)',
    '',
    'Examples:',
    '  hd-cli-skill install',
    '  hd-cli-skill run --structured --date 1990-05-15 --time 14:30 --utc +3 --lang ru',
    '  hd-cli-skill compact --input full.json',
    '  hd-cli-skill nlm-query --request-text \"...\"',
    '',
  ].join('\n');
}

export async function main(argv: string[]) {
  const [cmd, ...rest] = argv;
  if (!cmd || cmd === '--help' || cmd === '-h' || cmd === 'help') {
    process.stdout.write(help());
    return;
  }

  if (cmd === 'install') return await import('./install').then((m) => m.main(rest));
  if (cmd === 'run') return await import('./hd_cli_run').then((m) => m.main(rest));
  if (cmd === 'compact') return await import('./hd_compact').then((m) => m.main(rest));
  if (cmd === 'nlm-query') return await import('./nlm_query').then((m) => m.main(rest));

  die(`Unknown command: ${cmd}\n\n${help()}`);
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((e) => die((e as any)?.stack || (e as any)?.message || String(e)));
}

