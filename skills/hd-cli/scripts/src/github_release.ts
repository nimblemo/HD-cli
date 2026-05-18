export type LatestReleaseError =
  | { ok: false; reason: 'not_found'; status: 404 }
  | { ok: false; reason: 'forbidden'; status: 401 | 403 }
  | { ok: false; reason: 'http_error'; status: number };

export type LatestReleaseOk = { ok: true; tag: string; assets: any[] };

export type LatestReleaseResult = LatestReleaseError | LatestReleaseOk;

export async function fetchLatestRelease(repo: string): Promise<LatestReleaseResult> {
  const url = `https://api.github.com/repos/${repo}/releases/latest`;
  const res = await fetch(url, {
    headers: {
      'User-Agent': 'hd-cli-skill-scripts',
      Accept: 'application/vnd.github+json',
    },
  });

  if (res.status === 404) return { ok: false, reason: 'not_found', status: 404 };
  if (res.status === 401 || res.status === 403) {
    return { ok: false, reason: 'forbidden', status: res.status };
  }
  if (!res.ok) return { ok: false, reason: 'http_error', status: res.status };

  const data: any = await res.json();
  const tag = String(data?.tag_name || '');
  const assets = Array.isArray(data?.assets) ? data.assets : [];
  return { ok: true, tag, assets };
}

export function selectAsset(opts: { version: string; os: string; arch: string; assets: any[] }) {
  const base =
    opts.os === 'windows'
      ? `hd-cli-v${opts.version}-${opts.os}-${opts.arch}.zip`
      : `hd-cli-v${opts.version}-${opts.os}-${opts.arch}.tar.gz`;
  const a = opts.assets.find((x) => x && x.name === base);
  return a ? { ok: true as const, name: a.name as string, url: a.browser_download_url as string } : { ok: false as const, expected: base };
}

export function selectSha256SumsAsset(assets: any[]) {
  const a = assets.find((x) => x && x.name === 'SHA256SUMS');
  return a ? { ok: true as const, url: a.browser_download_url as string } : { ok: false as const };
}

