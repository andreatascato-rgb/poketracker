/**
 * Genera icone app da SVG Pokéball: PNG 1024, icone Tauri, favicon.
 * Eseguire da root: node scripts/generate-icon.mjs
 */
import { readFile, writeFile } from 'fs/promises';
import { copyFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';
import { Resvg } from '@resvg/resvg-js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, '..');
const svgPath = join(root, 'src-tauri', 'icons', 'pokeball.svg');
const appIconPath = join(root, 'app-icon.png');
const faviconDest = join(root, 'static', 'favicon.png');
const icons128 = join(root, 'src-tauri', 'icons', '128x128.png');

async function main() {
  const svg = await readFile(svgPath);
  const resvg = new Resvg(svg, {
    fitTo: { mode: 'width', value: 1024 },
  });
  const png = resvg.render();
  const pngBuffer = png.asPng();
  await writeFile(appIconPath, pngBuffer);
  console.log('Generated app-icon.png (1024x1024)');

  execSync('npm run tauri icon app-icon.png', {
    cwd: root,
    stdio: 'inherit',
  });

  copyFileSync(icons128, faviconDest);
  console.log('Updated static/favicon.png from 128x128');
  console.log('');
  console.log('Per vedere le nuove icone:');
  console.log('  1. Chiudi completamente l\'app PokeTracker (e "npm run tauri dev" se attivo)');
  console.log('  2. npm run tauri build');
  console.log('  3. Avvia src-tauri\\target\\release\\poketracker.exe');
  console.log('  Favicon (browser): Ctrl+Shift+R o svuota cache. In app.html usa ?v=2.');
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
