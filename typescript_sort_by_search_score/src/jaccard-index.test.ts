import jaccardIndex from './jaccard-index';

describe('jaccardIndex', () => {
  it('should return 1 for identical strings', () => {
    expect(jaccardIndex('hello', 'hello')).toBe(1);
  });

  it('should return 1 for identical strings with different cases', () => {
    expect(jaccardIndex('HELLO', 'hello')).toBe(1);
  });

  it('should return 0 for completely different strings', () => {
    expect(jaccardIndex('pumpkin', 'world')).toBe(0);
  });

  it('should be order-independent', () => {
    expect(jaccardIndex('hello', 'hell')).toBe(jaccardIndex('hell', 'hello'));
  });

  it('should return 0 when any of the string is empty', () => {
    expect(jaccardIndex('hello', '')).toBe(0);
    expect(jaccardIndex('', 'hello')).toBe(0);
  });

  const testCases: [string, string, number][] = [
    ['hello', 'hell', 0.75],
    ['hello', 'hallo', 0.6],
    ['hello', 'helxo', 0.8],
  ];

  testCases.forEach(([a, b, expected]) => {
    it(`should return the correct value for ${a} and ${b}`, () => {
      expect(jaccardIndex(a, b)).toBe(expected);
    });
  });
});
