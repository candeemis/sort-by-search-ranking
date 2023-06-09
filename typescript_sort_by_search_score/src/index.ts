const calculateScore = (searchTerm: string, source: string): number => {
  if (source.startsWith(searchTerm)) {
    return 1 + searchTerm.length / source.length;
  } else if (source.includes(searchTerm)) {
    return searchTerm.length / source.length;
  } else {
    return -1;
  }
};

export const sortByRankingScore = (
  searchTerm: string,
  searchResult: string[]
): string[] => {
  return searchResult
    .map((searchItem) => ({
      score: calculateScore(searchTerm, searchItem),
      searchItem,
    }))
    .sort((a, b) => b.score - a.score)
    .map((sorted) => sorted.searchItem);
};
