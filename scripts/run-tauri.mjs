/**
 * Avvia il CLI Tauri con CARGO_HOME e RUSTUP_HOME.
 * Percorso Rust: RUST_ROOT da .env (copia .env.example in .env) oppure default C:\_Main\_app.
 * Usato quando Rust/Cargo sono in cartella custom (es. dopo aver spostato le cartelle).
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
const RUST_ROOT = process.env.RUST_ROOT ?? 'C:\\_Main\\_app';
const CARGO_BIN = `${RUST_ROOT}\\.cargo\\bin`;

const env = {
  ...process.env,
  CARGO_HOME: `${RUST_ROOT}\\.cargo`,
  RUSTUP_HOME: `${RUST_ROOT}\\.rustup`,
  PATH: `${CARGO_BIN};${process.env.PATH || ''}`,
};

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
