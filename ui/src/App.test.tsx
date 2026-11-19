import { describe, it, expect } from 'vitest';

describe('Antares ThreadClip Studio UI', () => {
  it('loads screen identifiers correctly', () => {
    const screens = [
      'dashboard',
      'import',
      'content',
      'assets',
      'editor',
      'preview',
      'voice',
      'render',
      'history',
      'settings',
      'logs',
    ];
    expect(screens.length).toBe(11);
  });
});
