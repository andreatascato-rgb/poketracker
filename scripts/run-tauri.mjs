/**
 * Avvia il CLI Tauri.
 * Se in .env è impostato RUST_ROOT, usa quella cartella per CARGO_HOME/RUSTUP_HOME (Rust custom).
 * Altrimenti usa l'installazione Rust dell'utente (es. C:\Users\<user>\.cargo).
 */
import { spawn } from 'child_process';
import { readFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');
try {
  const content = readFileSync(join(rootDir, '.env'), 'utf8');
  for (const line of content.split('\n')) {
    const m = line.match(/^\s*RUST_ROOT\s*=\s*(.+)\s*$/);
    if (m) process.env.RUST_ROOT = m[1].trim().replace(/^["']|["']$/g, '');
  }
} catch (_) {}

const env = { ...process.env };
if (process.env.RUST_ROOT) {
  const RUST_ROOT = process.env.RUST_ROOT;
  const CARGO_BIN = `${RUST_ROOT}\\.cargo\\bin`;
  env.CARGO_HOME = `${RUST_ROOT}\\.cargo`;
  env.RUSTUP_HOME = `${RUST_ROOT}\\.rustup`;
  env.PATH = `${CARGO_BIN};${process.env.PATH || ''}`;
}

let args = process.argv.slice(2);
if (args.length === 0) args = ['dev'];
const child = spawn('npx', ['tauri', ...args], {
  stdio: 'inherit',
  shell: true,
  env,
});

child.on('exit', (code, signal) => {
  process.exit(code ?? (signal ? 1 : 0));
});
