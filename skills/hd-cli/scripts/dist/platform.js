function getPlatform() {
  const p = process.platform;
  const a = process.arch;

  const os =
    p === 'win32' ? 'windows' : p === 'darwin' ? 'darwin' : p === 'linux' ? 'linux' : null;
  if (!os) {
    throw new Error(`Unsupported OS: ${p}`);
  }

  const arch = a === 'x64' ? 'x86_64' : null;
  if (!arch) {
    throw new Error(`Unsupported arch: ${a}`);
  }

  return { os, arch, key: `${os}-${arch}` };
}

module.exports = { getPlatform };

