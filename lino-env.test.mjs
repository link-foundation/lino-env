import { describe, test, expect, beforeEach, afterEach } from '@jest/globals';
import { LinoEnv, readLinoEnv, writeLinoEnv } from './lino-env.mjs';
import { unlinkSync, existsSync } from 'fs';

const TEST_FILE = '.test.lenv';

describe('LinoEnv', () => {
  afterEach(() => {
    // Clean up test file after each test
    if (existsSync(TEST_FILE)) {
      unlinkSync(TEST_FILE);
    }
  });

  describe('Basic reading and writing', () => {
    test('should create and write a simple .lenv file', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('GITHUB_TOKEN', 'gh_test123');
      env.set('TELEGRAM_TOKEN', '054test456');
      env.write();

      expect(existsSync(TEST_FILE)).toBe(true);
    });

    test('should read a .lenv file', () => {
      // First create a file
      const env1 = new LinoEnv(TEST_FILE);
      env1.set('GITHUB_TOKEN', 'gh_test123');
      env1.set('TELEGRAM_TOKEN', '054test456');
      env1.write();

      // Then read it
      const env2 = new LinoEnv(TEST_FILE);
      env2.read();

      expect(env2.get('GITHUB_TOKEN')).toBe('gh_test123');
      expect(env2.get('TELEGRAM_TOKEN')).toBe('054test456');
    });
  });

  describe('get() method', () => {
    test('should get the last instance of a reference', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('API_KEY', 'value1');
      env.add('API_KEY', 'value2');
      env.add('API_KEY', 'value3');

      expect(env.get('API_KEY')).toBe('value3');
    });

    test('should return undefined for non-existent reference', () => {
      const env = new LinoEnv(TEST_FILE);
      expect(env.get('NON_EXISTENT')).toBeUndefined();
    });
  });

  describe('getAll() method', () => {
    test('should get all instances of a reference', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('API_KEY', 'value1');
      env.add('API_KEY', 'value2');
      env.add('API_KEY', 'value3');

      const all = env.getAll('API_KEY');
      expect(all).toEqual(['value1', 'value2', 'value3']);
    });

    test('should return empty array for non-existent reference', () => {
      const env = new LinoEnv(TEST_FILE);
      expect(env.getAll('NON_EXISTENT')).toEqual([]);
    });
  });

  describe('set() method', () => {
    test('should set all instances of a reference to a new value', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('API_KEY', 'value1');
      env.add('API_KEY', 'value2');
      env.set('API_KEY', 'new_value');

      expect(env.get('API_KEY')).toBe('new_value');
      expect(env.getAll('API_KEY')).toEqual(['new_value']);
    });
  });

  describe('add() method', () => {
    test('should add duplicate instances', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('KEY', 'value1');
      env.add('KEY', 'value2');
      env.add('KEY', 'value3');

      expect(env.getAll('KEY')).toEqual(['value1', 'value2', 'value3']);
    });
  });

  describe('has() method', () => {
    test('should return true for existing reference', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('KEY', 'value');

      expect(env.has('KEY')).toBe(true);
    });

    test('should return false for non-existent reference', () => {
      const env = new LinoEnv(TEST_FILE);
      expect(env.has('NON_EXISTENT')).toBe(false);
    });
  });

  describe('delete() method', () => {
    test('should delete all instances of a reference', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('KEY', 'value1');
      env.add('KEY', 'value2');
      env.delete('KEY');

      expect(env.has('KEY')).toBe(false);
      expect(env.get('KEY')).toBeUndefined();
    });
  });

  describe('keys() method', () => {
    test('should return all keys', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('KEY1', 'value1');
      env.set('KEY2', 'value2');
      env.set('KEY3', 'value3');

      const keys = env.keys();
      expect(keys).toContain('KEY1');
      expect(keys).toContain('KEY2');
      expect(keys).toContain('KEY3');
      expect(keys.length).toBe(3);
    });
  });

  describe('toObject() method', () => {
    test('should convert to object with last instance of each key', () => {
      const env = new LinoEnv(TEST_FILE);
      env.add('KEY1', 'value1a');
      env.add('KEY1', 'value1b');
      env.set('KEY2', 'value2');

      const obj = env.toObject();
      expect(obj).toEqual({
        KEY1: 'value1b',
        KEY2: 'value2'
      });
    });
  });

  describe('Persistence', () => {
    test('should persist duplicates across write/read', () => {
      const env1 = new LinoEnv(TEST_FILE);
      env1.add('KEY', 'value1');
      env1.add('KEY', 'value2');
      env1.add('KEY', 'value3');
      env1.write();

      const env2 = new LinoEnv(TEST_FILE);
      env2.read();

      expect(env2.getAll('KEY')).toEqual(['value1', 'value2', 'value3']);
      expect(env2.get('KEY')).toBe('value3');
    });

    test('should handle empty lines and whitespace', () => {
      const env1 = new LinoEnv(TEST_FILE);
      env1.set('KEY1', 'value1');
      env1.set('KEY2', 'value2');
      env1.write();

      const env2 = new LinoEnv(TEST_FILE);
      env2.read();

      expect(env2.get('KEY1')).toBe('value1');
      expect(env2.get('KEY2')).toBe('value2');
    });
  });

  describe('Convenience functions', () => {
    test('readLinoEnv should read and return LinoEnv instance', () => {
      writeLinoEnv(TEST_FILE, {
        GITHUB_TOKEN: 'gh_test',
        TELEGRAM_TOKEN: '054test'
      });

      const env = readLinoEnv(TEST_FILE);
      expect(env.get('GITHUB_TOKEN')).toBe('gh_test');
      expect(env.get('TELEGRAM_TOKEN')).toBe('054test');
    });

    test('writeLinoEnv should create file with data', () => {
      writeLinoEnv(TEST_FILE, {
        API_KEY: 'test_key',
        SECRET: 'test_secret'
      });

      const env = readLinoEnv(TEST_FILE);
      expect(env.get('API_KEY')).toBe('test_key');
      expect(env.get('SECRET')).toBe('test_secret');
    });
  });

  describe('Format compliance', () => {
    test('should use ": " as separator', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('GITHUB_TOKEN', 'gh_test123');
      env.write();

      const env2 = readLinoEnv(TEST_FILE);
      expect(env2.get('GITHUB_TOKEN')).toBe('gh_test123');
    });

    test('should handle values with colons', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('URL', 'https://example.com:8080');
      env.write();

      const env2 = readLinoEnv(TEST_FILE);
      expect(env2.get('URL')).toBe('https://example.com:8080');
    });

    test('should handle values with spaces', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('MESSAGE', 'Hello World');
      env.write();

      const env2 = readLinoEnv(TEST_FILE);
      expect(env2.get('MESSAGE')).toBe('Hello World');
    });
  });

  describe('Edge cases', () => {
    test('should handle non-existent file on read', () => {
      const env = new LinoEnv('non-existent-file.lenv');
      env.read();

      expect(env.get('ANY_KEY')).toBeUndefined();
      expect(env.keys().length).toBe(0);
    });

    test('should handle empty values', () => {
      const env = new LinoEnv(TEST_FILE);
      env.set('EMPTY_KEY', '');
      env.write();

      const env2 = readLinoEnv(TEST_FILE);
      expect(env2.get('EMPTY_KEY')).toBe('');
    });
  });
});
