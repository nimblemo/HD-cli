async function fetchLatestRelease(repo) {
  const url = `https://api.github.com/repos/${repo}/releases/latest`;
  const res = await fetch(url, {
    headers: {
      'User-Agent': 'hd-cli-skill-scripts',
      Accept: 'application/vnd.github+json',
    },
  });

  if (res.status === 404) {
    return { ok: false, reason: 'not_found', status: 404 };
  }
  if (res.status === 401 || res.status === 403) {
    return { ok: false, reason: 'forbidden', status: res.status };
  }
  if (!res.ok) {
    return { ok: false, reason: 'http_error', status: res.status };
  }

  const data = await res.json();
  const tag = data?.tag_name;
  const assets = Array.isArray(data?.assets) ? data.assets : [];
  return { ok: true, tag, assets };
}

function selectAsset({ version, os, arch, assets }) {
  const base =
    os === 'windows'
      ? `hd-cli-v${version}-${os}-${arch}.zip`
      : `hd-cli-v${version}-${os}-${arch}.tar.gz`;
  const a = assets.find((x) => x && x.name === base);
  return a ? { ok: true, name: a.name, url: a.browser_download_url } : { ok: false, expected: base };
}

function selectSha256SumsAsset(assets) {
  const a = assets.find((x) => x && x.name === 'SHA256SUMS');
  return a ? { ok: true, url: a.browser_download_url } : { ok: false };
}

module.exports = {
  fetchLatestRelease,
  selectAsset,
  selectSha256SumsAsset,
};

