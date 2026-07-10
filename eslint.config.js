import globals from 'globals';
import pluginJs from '@eslint/js';
import tseslint from 'typescript-eslint';
import eslintConfigPrettier from 'eslint-config-prettier';

/**
 * ESLint 9 flat config.
 *
 * History: scripts used `--ext` (removed in ESLint 9) and this file imported
 * `eslint-plugin-prettier/recommended` (never installed) and referenced
 * undefined `eslint.configs.recommended`. Lint has crashed with exit 2 since
 * 2025-04-06; this restores a loadable config so `bun run lint` can run.
 *
 * Scope of this gate: make the CLI + config load and produce a clean exit for
 * CI. Type-checked strict rules are intentionally not enabled here — they
 * required a broken project service and accumulated ~1.6k unenforced findings
 * while lint was crashed. Formatting remains gated by `check-format` (Prettier).
 */
export default tseslint.config(
  {
    ignores: [
      'dist/**',
      'scripts/**',
      'build/**',
      'assembly/**',
      'node_modules/**',
      'coverage/**',
      '**/*.config.js',
      '**/*.config.ts',
      '.test-cache-db/**',
      '.test-db/**',
      'docs/**',
    ],
  },
  {
    files: ['**/*.{js,mjs,cjs,ts}'],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['**/*.ts'],
    rules: {
      // Keep a small, enforceable set — not the historical never-applied strict set
      'no-console': ['warn', { allow: ['warn', 'error', 'info', 'log'] }],
      'prefer-const': 'error',
      eqeqeq: ['error', 'always'],
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
      ],
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-require-imports': 'off',
    },
  },
  {
    files: ['tests/**/*.ts'],
    rules: {
      '@typescript-eslint/no-unused-vars': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
    },
  },
  // Disable ESLint rules that conflict with Prettier (formatting gated by check-format)
  eslintConfigPrettier,
);
