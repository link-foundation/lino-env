// Run via npm run test:consumer. Install the packed library alongside each
// supported consumer parser release without overrides or npm dedupe.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import {
  mkdtempSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { createRequire } from 'node:module';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath, pathToFileURL, URL } from 'node:url';

const npmCli = process.env.npm_execpath;
assert.ok(npmCli, 'Run with npm run test:consumer');
const packageRoot = fileURLToPath(new URL('../', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'lino-env-consumer-'));

function npm(args, cwd) {
  return execFileSync(process.execPath, [npmCli, ...args], {
    cwd,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'inherit'],
  });
}

try {
  const [packed] = JSON.parse(
    npm(
      ['pack', '--pack-destination', directory, '--json', '--ignore-scripts'],
      packageRoot
    )
  );

  for (const version of ['0.25.2', '0.26.0', '0.27.1']) {
    const consumer = join(directory, version);
    mkdirSync(consumer);
    writeFileSync(
      join(consumer, 'package.json'),
      JSON.stringify({
        name: 'lino-env-consumer-check',
        version: '1.0.0',
        private: true,
        type: 'module',
        dependencies: {
          'lino-env': `file:../${packed.filename}`,
          'links-notation': version,
        },
      })
    );
    npm(['install', '--ignore-scripts', '--no-audit', '--no-fund'], consumer);
    console.log(npm(['ls', 'links-notation', '--all'], consumer));

    const lock = JSON.parse(
      readFileSync(join(consumer, 'package-lock.json'), 'utf8')
    );
    const parserPackages = Object.entries(lock.packages).filter(([path]) =>
      path.endsWith('node_modules/links-notation')
    );
    assert.equal(parserPackages.length, 1, 'Expected one parser installation');
    assert.equal(parserPackages[0][1].version, version);

    const require = createRequire(join(consumer, 'package.json'));
    const linoEntry = require.resolve('lino-env');
    const parserEntry = require.resolve('links-notation');
    assert.equal(
      realpathSync(createRequire(linoEntry).resolve('links-notation')),
      realpathSync(parserEntry)
    );

    const { LinoEnv } = await import(pathToFileURL(linoEntry).href);
    const { Parser, Link } = await import(pathToFileURL(parserEntry).href);
    const file = join(consumer, 'preferences.lenv');
    const expected = {
      LAZADA_MARKET: 'vn',
      CITY: 'Nha Trang',
      ENABLED: 'true',
      FLAVOUR: 'unflavoured',
      SPACE: '  preserve  ',
    };
    for (const ending of ['\n', '\r\n']) {
      const lines = [
        'LAZADA_MARKET: th',
        ...Object.entries(expected).map(([key, value]) => `${key}: ${value}`),
      ];
      writeFileSync(file, `${lines.join(ending)}${ending}`);
      const env = new LinoEnv(file).read();
      assert.deepEqual(env.toObject(), expected);
      env.write();
      assert.deepEqual(new LinoEnv(file).read().toObject(), expected);

      const parser = new Parser();
      const [link] = parser.parse('CITY: "Nha Trang"');
      assert.ok(link.equals(new Link('CITY', [new Link('Nha Trang')])));
      assert.ok(parser.parse(link.toString())[0].equals(link));
    }
    console.log(
      `Consumer with links-notation ${version}: deduped and verified`
    );
  }
} finally {
  rmSync(directory, { recursive: true, force: true });
}
