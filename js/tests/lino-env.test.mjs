import { test, assert } from 'test-anywhere';
import { LinoEnv, readLinoEnv, writeLinoEnv } from '../src/lino-env.mjs';
import { unlinkSync, existsSync, writeFileSync } from 'node:fs';

const TEST_FILE = '.test.lenv';

function cleanup() {
  if (existsSync(TEST_FILE)) {
    unlinkSync(TEST_FILE);
  }
}

// Basic reading and writing
test('should create and write a simple .lenv file', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('GITHUB_TOKEN', 'gh_test123');
  env.set('TELEGRAM_TOKEN', '054test456');
  env.write();

  assert.ok(existsSync(TEST_FILE), 'File should exist');
  cleanup();
});

test('should read a .lenv file', () => {
  cleanup();
  // First create a file
  const env1 = new LinoEnv(TEST_FILE);
  env1.set('GITHUB_TOKEN', 'gh_test123');
  env1.set('TELEGRAM_TOKEN', '054test456');
  env1.write();

  // Then read it
  const env2 = new LinoEnv(TEST_FILE);
  env2.read();

  assert.equal(env2.get('GITHUB_TOKEN'), 'gh_test123');
  assert.equal(env2.get('TELEGRAM_TOKEN'), '054test456');
  cleanup();
});

// get() method
test('should return undefined for non-existent reference', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  assert.equal(env.get('NON_EXISTENT'), undefined);
  cleanup();
});

// set() method
test('should set a reference to a value', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('API_KEY', 'new_value');

  assert.equal(env.get('API_KEY'), 'new_value');
  cleanup();
});

// Duplicate keys: last value wins (rewrite semantics)
test('should use last value when duplicate keys exist in file', () => {
  cleanup();
  // Write a file with duplicate keys manually
  writeFileSync(TEST_FILE, 'A: value1\nA: value2\n', 'utf-8');

  const env = new LinoEnv(TEST_FILE);
  env.read();

  assert.equal(env.get('A'), 'value2');
  cleanup();
});

test('should overwrite when setting same key twice', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY', 'value1');
  env.set('KEY', 'value2');

  assert.equal(env.get('KEY'), 'value2');
  cleanup();
});

// has() method
test('should return true for existing reference', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY', 'value');

  assert.equal(env.has('KEY'), true);
  cleanup();
});

test('should return false for non-existent reference', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  assert.equal(env.has('NON_EXISTENT'), false);
  cleanup();
});

// delete() method
test('should delete a reference', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY', 'value1');
  env.delete('KEY');

  assert.equal(env.has('KEY'), false);
  assert.equal(env.get('KEY'), undefined);
  cleanup();
});

// keys() method
test('should return all keys', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY1', 'value1');
  env.set('KEY2', 'value2');
  env.set('KEY3', 'value3');

  const keys = env.keys();
  assert.ok(keys.includes('KEY1'), 'Should contain KEY1');
  assert.ok(keys.includes('KEY2'), 'Should contain KEY2');
  assert.ok(keys.includes('KEY3'), 'Should contain KEY3');
  assert.equal(keys.length, 3);
  cleanup();
});

// toObject() method
test('should convert to object', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY1', 'value1');
  env.set('KEY2', 'value2');

  const obj = env.toObject();
  assert.deepEqual(obj, {
    KEY1: 'value1',
    KEY2: 'value2',
  });
  cleanup();
});

// Persistence
test('should persist values across write/read', () => {
  cleanup();
  const env1 = new LinoEnv(TEST_FILE);
  env1.set('KEY', 'value');
  env1.write();

  const env2 = new LinoEnv(TEST_FILE);
  env2.read();

  assert.equal(env2.get('KEY'), 'value');
  cleanup();
});

test('should handle empty lines and whitespace', () => {
  cleanup();
  const env1 = new LinoEnv(TEST_FILE);
  env1.set('KEY1', 'value1');
  env1.set('KEY2', 'value2');
  env1.write();

  const env2 = new LinoEnv(TEST_FILE);
  env2.read();

  assert.equal(env2.get('KEY1'), 'value1');
  assert.equal(env2.get('KEY2'), 'value2');
  cleanup();
});

// Convenience functions
test('readLinoEnv should read and return LinoEnv instance', () => {
  cleanup();
  writeLinoEnv(TEST_FILE, {
    GITHUB_TOKEN: 'gh_test',
    TELEGRAM_TOKEN: '054test',
  });

  const env = readLinoEnv(TEST_FILE);
  assert.equal(env.get('GITHUB_TOKEN'), 'gh_test');
  assert.equal(env.get('TELEGRAM_TOKEN'), '054test');
  cleanup();
});

test('writeLinoEnv should create file with data', () => {
  cleanup();
  writeLinoEnv(TEST_FILE, {
    API_KEY: 'test_key',
    SECRET: 'test_secret',
  });

  const env = readLinoEnv(TEST_FILE);
  assert.equal(env.get('API_KEY'), 'test_key');
  assert.equal(env.get('SECRET'), 'test_secret');
  cleanup();
});

// Format compliance
test('should use ": " as separator', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('GITHUB_TOKEN', 'gh_test123');
  env.write();

  const env2 = readLinoEnv(TEST_FILE);
  assert.equal(env2.get('GITHUB_TOKEN'), 'gh_test123');
  cleanup();
});

test('should handle values with colons', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('URL', 'https://example.com:8080');
  env.write();

  const env2 = readLinoEnv(TEST_FILE);
  assert.equal(env2.get('URL'), 'https://example.com:8080');
  cleanup();
});

test('should handle values with spaces', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('MESSAGE', 'Hello World');
  env.write();

  const env2 = readLinoEnv(TEST_FILE);
  assert.equal(env2.get('MESSAGE'), 'Hello World');
  cleanup();
});

// Edge cases
test('should handle non-existent file on read', () => {
  cleanup();
  const env = new LinoEnv('non-existent-file.lenv');
  env.read();

  assert.equal(env.get('ANY_KEY'), undefined);
  assert.equal(env.keys().length, 0);
  cleanup();
});

test('should handle empty values', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('EMPTY_KEY', '');
  env.write();

  const env2 = readLinoEnv(TEST_FILE);
  assert.equal(env2.get('EMPTY_KEY'), '');
  cleanup();
});

// Write does not produce duplicate lines
test('should write one line per key', () => {
  cleanup();
  const env = new LinoEnv(TEST_FILE);
  env.set('KEY', 'value1');
  env.set('KEY', 'value2');
  env.write();

  const env2 = new LinoEnv(TEST_FILE);
  env2.read();
  assert.equal(env2.get('KEY'), 'value2');
  assert.equal(env2.keys().length, 1);
  cleanup();
});
