const assert = require('node:assert/strict');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const test = require('node:test');

test('hd-cli-skill runs via npx and shows help', () => {
  const pkgPath = path.resolve(__dirname, '..');
  const out = execFileSync(
    'npx',
    ['--yes', '--package', `file:${pkgPath}`, 'hd-cli-skill', '--help'],
    { cwd: path.resolve(__dirname, '..', '..', '..', '..'), encoding: 'utf8' }
  );

  assert.match(out, /hd-cli-skill/i);
});

test('hd-cli-skill-compact works via npx', () => {
  const pkgPath = path.resolve(__dirname, '..');
  const input = JSON.stringify({
    birth_date: '1990-01-01',
    birth_time: '00:00',
    utc_offset: 0,
    type: 'Generator',
    profile: '1/3',
    authority: 'Sacral',
    strategy: 'Wait',
    incarnation_cross: 'X',
    personality: [],
    design: [],
    channels: [],
    centers: [],
    type_description: 'should be removed',
  });

  const out = execFileSync(
    'npx',
    ['--yes', '--package', `file:${pkgPath}`, 'hd-cli-skill-compact'],
    { cwd: path.resolve(__dirname, '..', '..', '..', '..'), input, encoding: 'utf8' }
  );

  const parsed = JSON.parse(out);
  assert.equal(parsed.type_description, undefined);
  assert.equal(parsed.birth_date, '1990-01-01');
});
