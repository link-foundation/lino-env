#!/usr/bin/env node

import linoenv from '../src/lino-env.mjs';

console.log('=== Dotenvx-style API Example ===\n');

// Example 1: Using config() to load .lenv file
console.log('1. Using config() to load .lenv file...');
// First, let's create a .lenv file
linoenv.set('HELLO', 'World');
linoenv.set('API_KEY', 'secret123');

// Now load it with config()
linoenv.config();
console.log('   process.env.HELLO:', process.env.HELLO);
console.log('   process.env.API_KEY:', process.env.API_KEY);
console.log('');

// Example 2: Using get() method
console.log('2. Using get() method...');
const hello = linoenv.get('HELLO');
const apiKey = linoenv.get('API_KEY');
console.log('   HELLO:', hello);
console.log('   API_KEY:', apiKey);
console.log('');

// Example 3: Using set() method
console.log('3. Using set() method...');
linoenv.set('NEW_KEY', 'new_value');
console.log('   Set NEW_KEY to:', linoenv.get('NEW_KEY'));
console.log('   Also in process.env:', process.env.NEW_KEY);
console.log('');

// Example 4: Using custom path
console.log('4. Using custom path...');
linoenv.set('PRODUCTION_KEY', 'prod_value', { path: 'examples/.lenv.production' });
const prodValue = linoenv.get('PRODUCTION_KEY', { path: 'examples/.lenv.production' });
console.log('   PRODUCTION_KEY from .lenv.production:', prodValue);
console.log('');

// Example 5: Named exports
console.log('5. Using named exports...');
import { config, get, set } from '../src/lino-env.mjs';

set('NAMED_EXPORT_TEST', 'works!');
console.log('   NAMED_EXPORT_TEST:', get('NAMED_EXPORT_TEST'));
console.log('');

console.log('=== Examples completed successfully ===');
