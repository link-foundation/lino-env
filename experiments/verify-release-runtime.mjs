#!/usr/bin/env node

// Import release tools exactly as the release scripts do, then run a harmless command.
// This catches dependency/runtime incompatibilities before a release can mutate anything.
import assert from 'node:assert/strict';

const response = await fetch('https://unpkg.com/use-m/use.js');
assert.ok(response.ok, `Failed to load use-m: HTTP ${response.status}`);
const { use } = eval(await response.text());
const { $ } = await use('command-stream');
const { makeConfig } = await use('lino-arguments');
assert.equal(typeof makeConfig, 'function');
const result = await $`printf release-runtime-ok`.run({ capture: true });
assert.equal(result.stdout.trim(), 'release-runtime-ok');
console.log(`Release tools work on ${process.version}`);
