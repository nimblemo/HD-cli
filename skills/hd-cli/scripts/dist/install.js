#!/usr/bin/env node
const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const { parseArgs, die, ensureDir } = require('./utils');
const { getPlatform } = require('./platform');
const { fetchLatestRelease, selectAsset, selectSha256SumsAsset } = require('./github_release');

function getDefaultCacheRoot() {
  const home = os.homedir();
  return path.join(home, '.cache', 'hd-cli');
}

function sha256Hex(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function parseSha256Sums(text) {
  const map = new Map();
  for (const line of String(text).split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const m = trimmed.match(/^([a-fA-F0-9]{64})\s+(.+)$/);
    if (!m) continue;
    map.set(m[2].trim(), m[1].toLowerCase());
  }
  return map;
}

function listFilesRecursive(dir) {
  const out = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const e of entries) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) out.push(...listFilesRecursive(p));
    else out.push(p);
  }
  return out;
}

function findBinary(extractDir, isWindows) {
  const want = isWindows ? 'hd-cli.exe' : 'hd-cli';
  for (const f of listFilesRecursive(extractDir)) {
    if (path.basename(f) === want) return f;
  }
  return null;
}

function extractArchive({ archivePath, destDir, osName }) {
  ensureDir(destDir);
  if (osName === 'windows') {
    const r = spawnSync('unzip', ['-o', archivePath, '-d', destDir], { encoding: 'utf8' });
    if (r.status !== 0) die(r.stderr || r.stdout || 'Failed to unzip archive');
    return;
  }

  const r = spawnSync('tar', ['-xzf', archivePath, '-C', destDir], { encoding: 'utf8' });
  if (r.status !== 0) die(r.stderr || r.stdout || 'Failed to extract tar.gz archive');
}

function linkOrCopyCurrent({ cacheRoot, version, binPath, osName }) {
  const currentDir = path.join(cacheRoot, 'current');
  ensureDir(currentDir);
  const currentBin = path.join(currentDir, osName === 'windows' ? 'hd-cli.exe' : 'hd-cli');
  try {
    fs.rmSync(currentBin, { force: true });
  } catch {}

  if (osName === 'windows') {
    fs.copyFileSync(binPath, currentBin);
    return currentBin;
  }

  fs.symlinkSync(binPath, currentBin);
  return currentBin;
}

async function main(argv) {
  const { flags } = parseArgs(argv);
  const repo = String(flags.repo || 'nimblemo/Human-Design-cli');
  const cacheRoot = String(flags['cache-root'] || getDefaultCacheRoot());

  const { os: osName, arch } = getPlatform();
  const rel = await fetchLatestRelease(repo);

  if (!rel.ok) {
    const msg =
      rel.reason === 'not_found'
        ? `No GitHub releases found for ${repo}`
        : `Cannot fetch latest release for ${repo} (status ${rel.status})`;
    die(msg);
  }

  const version = String(rel.tag || '').replace(/^v/, '');
  if (!version) die('Latest release has no tag_name');

  const assetSel = selectAsset({ version, os: osName, arch, assets: rel.assets });
  if (!assetSel.ok) {
    die(`No release asset found. Expected: ${assetSel.expected}`);
  }

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'hd-cli-'));
  const archivePath = path.join(tmpDir, assetSel.name);
  const archiveRes = await fetch(assetSel.url);
  if (!archiveRes.ok) die(`Failed to download asset: ${assetSel.url}`);
  const archiveBuf = Buffer.from(await archiveRes.arrayBuffer());
  fs.writeFileSync(archivePath, archiveBuf);

  const sumsSel = selectSha256SumsAsset(rel.assets);
  if (sumsSel.ok) {
    const sumsRes = await fetch(sumsSel.url);
    if (sumsRes.ok) {
      const sumsText = await sumsRes.text();
      const map = parseSha256Sums(sumsText);
      const expected = map.get(assetSel.name);
      if (expected) {
        const actual = sha256Hex(archiveBuf);
        if (actual !== expected) {
          die(`SHA256 mismatch for ${assetSel.name}. Expected ${expected}, got ${actual}`);
        }
      }
    }
  }

  const versionDir = path.join(cacheRoot, version);
  const extractDir = path.join(versionDir, 'extract');
  try {
    fs.rmSync(extractDir, { recursive: true, force: true });
  } catch {}

  extractArchive({ archivePath, destDir: extractDir, osName });

  const found = findBinary(extractDir, osName === 'windows');
  if (!found) die('Extracted archive does not contain hd-cli binary');

  ensureDir(versionDir);
  const binPath = path.join(versionDir, osName === 'windows' ? 'hd-cli.exe' : 'hd-cli');
  fs.copyFileSync(found, binPath);

  if (osName !== 'windows') {
    try {
      fs.chmodSync(binPath, 0o755);
    } catch {}
  }

  const current = linkOrCopyCurrent({ cacheRoot, version, binPath, osName });
  process.stdout.write(current + '\n');
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((e) => die(e?.stack || e?.message || String(e)));
}

module.exports = { main };

