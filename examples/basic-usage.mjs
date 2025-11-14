#!/usr/bin/env node

import { LinoEnv, readLinoEnv, writeLinoEnv } from '../lino-env.mjs';

console.log('=== LinoEnv Basic Usage Example ===\n');

// Example 1: Create and write a .lenv file
console.log('1. Creating a new .lenv file...');
const env1 = new LinoEnv('examples/test.lenv');
env1.set('GITHUB_TOKEN', 'gh_example123');
env1.set('TELEGRAM_TOKEN', '054example456');
env1.set('API_KEY', 'my_secret_key');
env1.write();
console.log('   ✓ Created examples/test.lenv\n');

// Example 2: Read the .lenv file
console.log('2. Reading the .lenv file...');
const env2 = readLinoEnv('examples/test.lenv');
console.log('   GITHUB_TOKEN:', env2.get('GITHUB_TOKEN'));
console.log('   TELEGRAM_TOKEN:', env2.get('TELEGRAM_TOKEN'));
console.log('   API_KEY:', env2.get('API_KEY'));
console.log('');

// Example 3: Working with duplicates
console.log('3. Working with duplicate keys...');
const env3 = new LinoEnv('examples/duplicates.lenv');
env3.add('SERVER', 'server1.example.com');
env3.add('SERVER', 'server2.example.com');
env3.add('SERVER', 'server3.example.com');
env3.write();

const env4 = readLinoEnv('examples/duplicates.lenv');
console.log('   Last SERVER value:', env4.get('SERVER'));
console.log('   All SERVER values:', env4.getAll('SERVER'));
console.log('');

// Example 4: Updating values
console.log('4. Updating values...');
const env5 = readLinoEnv('examples/test.lenv');
console.log('   Old API_KEY:', env5.get('API_KEY'));
env5.set('API_KEY', 'updated_secret_key');
env5.write();
const env6 = readLinoEnv('examples/test.lenv');
console.log('   New API_KEY:', env6.get('API_KEY'));
console.log('');

// Example 5: Using convenience functions
console.log('5. Using convenience functions...');
writeLinoEnv('examples/quick.lenv', {
  DATABASE_URL: 'postgresql://localhost:5432/mydb',
  REDIS_URL: 'redis://localhost:6379',
  PORT: '3000',
});
const quickEnv = readLinoEnv('examples/quick.lenv');
console.log('   Environment loaded:', quickEnv.toObject());
console.log('');

// Example 6: Checking existence and keys
console.log('6. Checking existence and listing keys...');
console.log('   Has DATABASE_URL?', quickEnv.has('DATABASE_URL'));
console.log('   Has NON_EXISTENT?', quickEnv.has('NON_EXISTENT'));
console.log('   All keys:', quickEnv.keys());
console.log('');

console.log('=== Examples completed successfully ===');
