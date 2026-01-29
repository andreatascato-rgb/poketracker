/**
 * Esegue cargo (check/build/test). Stessa logica di run-tauri.mjs per RUST_ROOT.
 * Se RUST_ROOT è in .env usa quella cartella; altrimenti usa l'installazione Rust dell'utente.
 * Uso: node scripts/cargo-check.mjs [check|build|test|...]
 */
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { readFileSync } from 'fs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');
try {
  const envPath = join(rootDir, '.env');
  const content = readFileSync(envPath, 'utf8');
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

const args = process.argv.slice(2);
const cmd = args[0] ?? 'check';
const rest = args.slice(1);
const cwd = join(__dirname, '..', 'src-tauri');

const child = spawn('cargo', [cmd, ...rest], {
  stdio: 'inherit',
  env,
  cwd,
});

child.on('exit', (code, signal) => {
  process.exit(code ?? (signal ? 1 : 0));
});
