#!/usr/bin/env node

const { execFileSync } = require('child_process');
const { join } = require('path');

const root = join(__dirname, '..');
const npmDir = join(root, 'npm');
const expectedFiles = [
  'bin/agent-desktop.js',
  'package.json',
  'scripts/postinstall.js',
];

const output = execFileSync('npm', ['pack', '--dry-run', '--json'], {
  cwd: npmDir,
  encoding: 'utf8',
  env: {
    ...process.env,
    npm_config_cache: process.env.npm_config_cache || '/tmp/agent-desktop-npm-cache',
  },
});

const pack = JSON.parse(output)[0];
const actualFiles = pack.files.map((file) => file.path).sort();
const expected = [...expectedFiles].sort();

if (JSON.stringify(actualFiles) !== JSON.stringify(expected)) {
  throw new Error(`Unexpected npm package contents: ${actualFiles.join(', ')}`);
}

if (pack.bundled && pack.bundled.length > 0) {
  throw new Error(`npm package unexpectedly bundles dependencies: ${pack.bundled.join(', ')}`);
}

if (pack.unpackedSize > 25_000) {
  throw new Error(`npm package is unexpectedly large: ${pack.unpackedSize} bytes`);
}

console.log(`OK: npm package contains ${actualFiles.length} files, ${pack.unpackedSize} bytes unpacked`);
