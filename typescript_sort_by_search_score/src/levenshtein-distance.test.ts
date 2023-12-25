import levenshtein from './levenshtein-distance';

describe('levenshtein', () => {
  it('should return 0 for identical strings', () => {
    expect(levenshtein('test', 'test')).toBe(0);
  });

  it('should return 0 for identical strings with different cases', () => {
    expect(levenshtein('TEST', 'test')).toBe(0);
  });

  it('should return the correct distance for different strings', () => {
    expect(levenshtein('kITTen', 'sitting')).toBe(3);
  });

  it('should handle empty strings', () => {
    expect(levenshtein('', '')).toBe(0);
    expect(levenshtein('test', '')).toBe(4);
    expect(levenshtein('', 'test')).toBe(4);
  });

  it('should handle non-ASCII characters', () => {
    expect(levenshtein('café', 'caffè')).toBe(2);
  });

  it('should handle strings with different lengths', () => {
    expect(levenshtein('Pumpkin', 'short')).toBe(7);
  });
});
