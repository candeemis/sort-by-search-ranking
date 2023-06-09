import { sortByRankingScore } from '.';

describe('index', () => {
  describe('sortByRankingScore', () => {
    it('returns empty array when empty array is provided as search result', () => {
      const searchTerm = 'berlin';
      const searchResult: string[] = [];

      const result = sortByRankingScore(searchTerm, searchResult);

      expect(result).toBeArrayOfSize(0);
    });

    it('returns with same sorting order when the search result does not contain the search term', () => {
      const searchTerm = 'berlin';
      const searchResult = [
        'london',
        'tokyo',
        'hamburg',
        'frankfurt',
        'washington',
        'new york',
        'moscow',
        'sidney',
      ];

      const result = sortByRankingScore(searchTerm, searchResult);

      expect(result).toEqual(searchResult);
    });

    it('returns sorted search result when the search result contains the search term', () => {
      const searchTerm = 'berlin';
      const searchResult = [
        'berlino',
        'mo-berlin',
        'potsdam',
        'berlin wannsee',
        'berlin',
        'weder',
        'alt berlin',
      ];

      const result = sortByRankingScore(searchTerm, searchResult);

      const expectedResult = [
        'berlin',
        'berlino',
        'berlin wannsee',
        'mo-berlin',
        'alt berlin',
        'potsdam',
        'weder',
      ];

      expect(result).toEqual(expectedResult);
    });
  });
});
