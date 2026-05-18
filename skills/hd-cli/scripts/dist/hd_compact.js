#!/usr/bin/env node
const fs = require('node:fs');

const { parseArgs, die } = require('./utils');

function compactPlanetPosition(p) {
  return {
    planet: p.planet,
    index: p.index,
    longitude: p.longitude,
    degree: p.degree,
    zodiac_sign: p.zodiac_sign,
    gate: p.gate,
    line: p.line,
    color: p.color,
    tone: p.tone,
    base: p.base,
  };
}

function compactChannel(c) {
  return { key: c.key, name: c.name };
}

function compactCenter(c) {
  return { name: c.name, defined: c.defined };
}

function compactHdChart(chart) {
  const out = {
    birth_date: chart.birth_date,
    birth_time: chart.birth_time,
    utc_offset: chart.utc_offset,
    type: chart.type || chart.hd_type,
    profile: chart.profile,
    authority: chart.authority,
    strategy: chart.strategy,
    incarnation_cross: chart.incarnation_cross,
    personality: Array.isArray(chart.personality) ? chart.personality.map(compactPlanetPosition) : [],
    design: Array.isArray(chart.design) ? chart.design.map(compactPlanetPosition) : [],
    channels: Array.isArray(chart.channels) ? chart.channels.map(compactChannel) : [],
    centers: Array.isArray(chart.centers) ? chart.centers.map(compactCenter) : [],
  };
  return out;
}

async function readStdin() {
  return await new Promise((resolve, reject) => {
    let data = '';
    process.stdin.setEncoding('utf8');
    process.stdin.on('data', (c) => (data += c));
    process.stdin.on('end', () => resolve(data));
    process.stdin.on('error', reject);
  });
}

async function main(argv) {
  const { flags } = parseArgs(argv);
  const inputPath = flags.input ? String(flags.input) : null;
  const raw = inputPath ? fs.readFileSync(inputPath, 'utf8') : await readStdin();
  if (!raw.trim()) die('No input JSON provided (use --input <file> or stdin)');
  const parsed = JSON.parse(raw);
  const compact = compactHdChart(parsed);
  process.stdout.write(JSON.stringify(compact, null, 2) + '\n');
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((e) => die(e?.stack || e?.message || String(e)));
}

module.exports = { compactHdChart, main };

