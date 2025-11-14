export default {
  testEnvironment: 'node',
  transform: {},
  moduleFileExtensions: ['js', 'mjs'],
  testMatch: ['<rootDir>/tests/**/*.test.mjs'],
  collectCoverageFrom: ['<rootDir>/src/**/*.mjs'],
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov'],
};
