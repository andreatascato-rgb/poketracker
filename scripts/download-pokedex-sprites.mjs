/**
 * Scarica sprite Pokémon gen 1-4 (id 1-493) da PokeAPI/sprites in static/pokedex-sprites/.
 * Uso: node scripts/download-pokedex-sprites.mjs (da root)
 * Vedi docs/project/pokedex-sprites.md.
 */
import { mkdir, writeFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, '..');
const outDir = join(root, 'static', 'pokedex-sprites');

const BASE_URL = 'https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon';
const MIN_ID = 1;
const MAX_ID = 493; // Gen 1-4
const DELAY_MS = 50; // tra una richiesta e l'altra per non sovraccaricare

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

async function downloadOne(id) {
  const url = `${BASE_URL}/${id}.png`;
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} per ${url}`);
  }
  const buf = Buffer.from(await res.arrayBuffer());
  const path = join(outDir, `${id}.png`);
  await writeFile(path, buf);
  return path;
}

async function main() {
  await mkdir(outDir, { recursive: true });
  console.log(`Download sprite ${MIN_ID}-${MAX_ID} in ${outDir}`);

  let ok = 0;
  let fail = 0;
  for (let id = MIN_ID; id <= MAX_ID; id++) {
    try {
      await downloadOne(id);
      ok++;
      if (id % 50 === 0) console.log(`  ${id}/${MAX_ID}`);
    } catch (e) {
      fail++;
      console.error(`  Errore id ${id}:`, e.message);
    }
    await sleep(DELAY_MS);
  }

  console.log(`Fatto: ${ok} ok, ${fail} errori.`);
  if (fail > 0) process.exit(1);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
