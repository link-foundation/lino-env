import { test, assert } from 'test-anywhere';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { LinoEnv, config, readLinoEnv } from '../src/lino-env.mjs';

function withFile(run) {
  const directory = mkdtempSync(join(tmpdir(), 'lino-env-endings-'));
  try {
    run(join(directory, 'preferences.lenv'));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

const preferences = [
  '# Buying preferences',
  '',
  'LAZADA_MARKET: th',
  'LAZADA_MARKET: vn',
  'CITY: Nha Trang',
  'ENABLED: true',
  'DISABLED: false',
  'SORT: newest',
  'FLAVOUR: chocolate',
  'ALTERNATIVE: unflavoured',
  'SPACE:   preserve  ',
  'TABS: \tpreserve\t',
  'EMPTY: ',
  'DOUBLE: "quoted value"',
  "SINGLE: 'quoted value'",
  'URL: https://example.com:8080/a: b',
];

const expectedPreferences = {
  LAZADA_MARKET: 'vn',
  CITY: 'Nha Trang',
  ENABLED: 'true',
  DISABLED: 'false',
  SORT: 'newest',
  FLAVOUR: 'chocolate',
  ALTERNATIVE: 'unflavoured',
  SPACE: '  preserve  ',
  TABS: '\tpreserve\t',
  EMPTY: '',
  DOUBLE: '"quoted value"',
  SINGLE: "'quoted value'",
  URL: 'https://example.com:8080/a: b',
};

for (const [name, ending] of [
  ['LF', '\n'],
  ['CRLF', '\r\n'],
]) {
  for (const finalNewline of [false, true]) {
    test(`${name} preferences roundtrip (final newline: ${finalNewline})`, () => {
      withFile((path) => {
        writeFileSync(
          path,
          preferences.join(ending) + (finalNewline ? ending : '')
        );
        const env = new LinoEnv(path).read();
        assert.deepEqual(env.toObject(), expectedPreferences);

        // Values must survive the library's LF writer as well as file reads.
        env.write();
        assert.deepEqual(readLinoEnv(path).toObject(), expectedPreferences);
      });
    });
  }

  for (const quote of ['"', "'"]) {
    test(`${name} multiline ${quote} values preserve content and skip nested keys`, () => {
      withFile((path) => {
        const lines = [
          `BLOCK: ${quote}`,
          '  CITY: Nha Trang  ',
          '',
          '  ENABLED: true',
          `  ${quote}`,
          'AFTER: unflavoured',
        ];
        writeFileSync(path, lines.join(ending) + ending);
        assert.deepEqual(readLinoEnv(path).toObject(), {
          BLOCK: '\n  CITY: Nha Trang  \n\n  ENABLED: true\n  ',
          AFTER: 'unflavoured',
        });
      });
    });
  }
}

test('mixed LF/CRLF endings preserve spaces and literal carriage returns', () => {
  withFile((path) => {
    writeFileSync(
      path,
      'A: first\r\nA: last  \nB: left\rright\r\nC: keep\r\r\nD: final\r'
    );
    assert.deepEqual(readLinoEnv(path).toObject(), {
      A: 'last  ',
      B: 'left\rright',
      C: 'keep\r',
      D: 'final\r',
    });
  });
});

test('config injects CRLF consumer choices without carriage returns', () => {
  withFile((path) => {
    const choices = {
      LINO_ENV_CRLF_MARKET: 'vn',
      LINO_ENV_CRLF_CITY: 'Nha Trang',
      LINO_ENV_CRLF_ENABLED: 'true',
      LINO_ENV_CRLF_DISABLED: 'false',
      LINO_ENV_CRLF_SORT: 'newest',
      LINO_ENV_CRLF_FLAVOUR: 'chocolate',
      LINO_ENV_CRLF_ALTERNATIVE: 'unflavoured',
    };
    const original = new Map(
      Object.keys(choices).map((key) => [key, process.env[key]])
    );
    try {
      for (const key of Object.keys(choices)) {
        delete process.env[key];
      }
      writeFileSync(
        path,
        Object.entries(choices)
          .map(([key, value]) => `${key}: ${value}\r\n`)
          .join('')
      );
      assert.deepEqual(config({ path }).parsed, choices);
      for (const [key, value] of Object.entries(choices)) {
        assert.equal(process.env[key], value);
      }
      assert.equal(process.env.LINO_ENV_CRLF_ENABLED === 'true', true);
      assert.equal(process.env.LINO_ENV_CRLF_DISABLED === 'true', false);
    } finally {
      for (const [key, value] of original) {
        if (value === undefined) {
          delete process.env[key];
        } else {
          process.env[key] = value;
        }
      }
    }
  });
});
