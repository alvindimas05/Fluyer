import * as fs from 'fs';

const version = process.env.APP_VERSION;
const repo = process.env.GITHUB_REPO;
const tag = 'v' + version;

async function fetchRelease() {
  const res = await fetch(`https://api.github.com/repos/${repo}/releases/tags/${tag}`, {
    headers: { 'Authorization': `token ${process.env.GITHUB_TOKEN}` }
  });
  return await res.json();
}

async function fetchSig(url: string) {
  const res = await fetch(url);
  return await res.text();
}

async function main() {
  if (!version || !repo || !process.env.GITHUB_TOKEN) {
    console.error('Missing required environment variables (APP_VERSION, GITHUB_REPO, GITHUB_TOKEN)');
    process.exit(1);
  }

  const release = await fetchRelease();
  if (!release.assets) {
    console.error('No assets found');
    process.exit(1);
  }
  const assets = release.assets;

  const getAssetUrl = (nameRegex: RegExp) => {
    const asset = assets.find((a: any) => nameRegex.test(a.name));
    return asset ? asset.browser_download_url : null;
  };

  const platforms: Record<string, any> = {};

  const addPlatform = async (platformKey: string, sigRegex: RegExp, urlRegex: RegExp) => {
    const sigUrl = getAssetUrl(sigRegex);
    const url = getAssetUrl(urlRegex);
    if (sigUrl && url) {
      platforms[platformKey] = { signature: await fetchSig(sigUrl), url };
    }
  };

  await addPlatform('windows-x86_64', /\.exe\.sig$/, /\.exe$/);
  await addPlatform('darwin-x86_64', /_x64\.app\.tar\.gz\.sig$/, /_x64\.app\.tar\.gz$/);
  await addPlatform('darwin-aarch64', /_aarch64\.app\.tar\.gz\.sig$/, /_aarch64\.app\.tar\.gz$/);
  await addPlatform('linux-x86_64', /\.AppImage\.sig$/, /\.AppImage$/);

  const updater = {
    version: tag,
    notes: 'Update ' + tag,
    pub_date: new Date().toISOString(),
    platforms
  };

  fs.writeFileSync('latest.json', JSON.stringify(updater, null, 2));
  console.log('Generated latest.json');
}

main();
