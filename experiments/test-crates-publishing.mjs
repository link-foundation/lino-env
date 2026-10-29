#!/usr/bin/env node

// Exercise the actual workflow shell with a fake cargo; never contact a registry.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

const workflow = readFileSync(
  new URL('../.github/workflows/rust.yml', import.meta.url),
  'utf8'
);

// Only extract these named jobs and their literal run blocks, without a YAML dependency.
for (const jobName of ['auto-release', 'manual-release']) {
  const job = workflow.match(
    new RegExp(`^  ${jobName}:\\n[\\s\\S]*?(?=^  [a-z]|$(?![\\s\\S]))`, 'm')
  )?.[0];
  assert.ok(job, `Missing ${jobName} job`);
  const publish = job.match(
    /      - name: Publish to Crates.io\n[\s\S]*?(?=      - name:)/
  )?.[0];
  assert.ok(publish, `Missing ${jobName} publish step`);
  const script = publish
    .match(/        run: \|\n([\s\S]*?)\n        env:/)?.[1]
    .replace(/^          /gm, '');
  assert.ok(script, `Missing ${jobName} publish script`);

  test(`${jobName}: trusted publishing and fallback configuration`, () => {
    assert.match(job, /^      id-token: write$/m);
    const auth = job.match(
      /      - name: Authenticate with crates.io[^\n]*\n[\s\S]*?(?=      - name:)/
    )?.[0];
    assert.ok(auth);
    assert.match(auth, /uses: rust-lang\/crates-io-auth-action@[a-f0-9]{40}\n/);
    assert.match(auth, /^        id: prm_auth$/m);
    assert.match(auth, /^        continue-on-error: true$/m);
    assert.equal(
      auth.match(/^        if: (.*)$/m)?.[1],
      publish.match(/^        if: (.*)$/m)?.[1]
    );
    assert.match(
      publish,
      /CARGO_REGISTRY_TOKEN: \$\{\{ steps\.prm_auth\.outputs\.token \|\| secrets\.CARGO_REGISTRY_TOKEN \|\| secrets\.CARGO_TOKEN \}\}/
    );
    assert.doesNotMatch(script, /--token|secrets\./);
  });

  for (const [name, message, code, expectedStatus, expectedOutput] of [
    ['success', 'Published successfully', 0, 0, 'success'],
    [
      'already uploaded',
      'error: crate version is already uploaded',
      101,
      0,
      'already_exists',
    ],
    [
      'already exists',
      'error: crate version already exists',
      101,
      0,
      'already_exists',
    ],
    ['invalid token', 'error: authentication failed', 101, 1, 'failed'],
    [
      'missing token',
      'error: no token found, please run cargo login',
      101,
      1,
      'failed',
    ],
    [
      'network failure',
      'error: failed to connect to crates.io',
      101,
      1,
      'failed',
    ],
  ]) {
    test(`${jobName}: ${name}`, { timeout: 5000 }, () => {
      const directory = mkdtempSync(join(tmpdir(), 'lino-env-publish-'));
      try {
        writeFileSync(
          join(directory, 'Cargo.toml'),
          'name = "test-crate"\nversion = "1.2.3"\n'
        );
        writeFileSync(
          join(directory, 'cargo'),
          '#!/bin/sh\nprintf "%s\\n" "$MOCK_MESSAGE"\nexit "$MOCK_CODE"\n',
          { mode: 0o755 }
        );
        const output = join(directory, 'output');
        writeFileSync(output, '');
        // An unspecified Actions shell uses bash -e, without pipefail.
        const result = spawnSync('bash', ['-e', '-c', script], {
          cwd: directory,
          encoding: 'utf8',
          timeout: 4000,
          env: {
            PATH: `${directory}:${process.env.PATH}`,
            GITHUB_OUTPUT: output,
            MOCK_MESSAGE: message,
            MOCK_CODE: String(code),
            CARGO_REGISTRY_TOKEN: 'fake-test-token',
          },
        });
        assert.ifError(result.error);
        assert.equal(
          result.status,
          expectedStatus,
          result.stdout + result.stderr
        );
        assert.equal(
          readFileSync(output, 'utf8'),
          `publish_result=${expectedOutput}\n`
        );
        assert.ok(
          readFileSync(join(directory, 'publish_output.txt'), 'utf8').includes(
            message
          )
        );
      } finally {
        rmSync(directory, { recursive: true, force: true });
      }
    });
  }
}
