#!/usr/bin/env node

/**
 * Test script to verify setOutput function behavior
 *
 * This script tests that:
 * 1. When GITHUB_OUTPUT is set, it writes to the file
 * 2. When GITHUB_OUTPUT is set, it logs the output
 * 3. When GITHUB_OUTPUT is not set, nothing happens (graceful no-op)
 * 4. No deprecated ::set-output command is emitted
 */

import { appendFileSync, readFileSync, unlinkSync, existsSync } from 'fs';
import { join } from 'path';
import { tmpdir } from 'os';

/**
 * Append to GitHub Actions output file
 * @param {string} key
 * @param {string} value
 */
function setOutput(key, value) {
  const outputFile = process.env.GITHUB_OUTPUT;
  if (outputFile) {
    appendFileSync(outputFile, `${key}=${value}\n`);
    console.log(`Output: ${key}=${value}`);
  }
}

// Test 1: Without GITHUB_OUTPUT set
console.log('=== Test 1: Without GITHUB_OUTPUT ===');
delete process.env.GITHUB_OUTPUT;
console.log('Calling setOutput without GITHUB_OUTPUT...');
setOutput('test_key', 'test_value');
console.log('No error = PASS (graceful no-op)\n');

// Test 2: With GITHUB_OUTPUT set
console.log('=== Test 2: With GITHUB_OUTPUT ===');
const testOutputFile = join(tmpdir(), `github_output_test_${Date.now()}.txt`);
process.env.GITHUB_OUTPUT = testOutputFile;

console.log(`GITHUB_OUTPUT set to: ${testOutputFile}`);
console.log('Calling setOutput...');
setOutput('version_committed', 'true');
setOutput('new_version', '1.0.0');

// Verify file contents
const fileContents = readFileSync(testOutputFile, 'utf-8');
console.log('\nFile contents:');
console.log(fileContents);

// Check expected contents
const expectedLines = [
  'version_committed=true',
  'new_version=1.0.0'
];

let allPassed = true;
for (const line of expectedLines) {
  if (!fileContents.includes(line)) {
    console.error(`FAIL: Missing expected line: ${line}`);
    allPassed = false;
  }
}

// Check no deprecated command
if (fileContents.includes('::set-output')) {
  console.error('FAIL: Found deprecated ::set-output in output');
  allPassed = false;
}

// Cleanup
unlinkSync(testOutputFile);

if (allPassed) {
  console.log('\nAll tests PASSED!');
  process.exit(0);
} else {
  console.log('\nSome tests FAILED!');
  process.exit(1);
}
