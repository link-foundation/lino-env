export default {
  testEnvironment: 'node',
  transform: {},
  moduleFileExtensions: ['js', 'mjs'],
  testMatch: ['**/*.test.mjs'],
  collectCoverageFrom: ['lino-env.mjs'],
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov'],
};
