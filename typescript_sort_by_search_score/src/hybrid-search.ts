import levenshtein from './levenshtein-distance';
import jaccardIndex from './jaccard-index';

function hybridSearch(
  searchTerm: string,
  source: string[],
  top: number
): string[] {
  const jaccardResult = source.map((item) => {
    return {
      item,
      score: jaccardIndex(searchTerm, item),
    };
  });

  const sortedJaccardResult = jaccardResult.sort((a, b) => {
    return b.score - a.score;
  });

  const topJaccardResult = sortedJaccardResult.slice(0, top).map((item) => {
    return {
      item: item.item,
      score: levenshtein(searchTerm, item.item),
    };
  });

  return topJaccardResult.map((item) => item.item);
}

export default hybridSearch;
