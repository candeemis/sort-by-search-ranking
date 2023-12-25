function levenshtein(searchTerm: string, sourceString: string): number {
  const stLength = searchTerm.length;
  const srcLength = sourceString.length;

  if (stLength === 0) {
    return srcLength;
  }

  if (srcLength === 0) {
    return stLength;
  }

  const st = searchTerm.toLowerCase();
  const ss = sourceString.toLowerCase();

  if (st === ss) {
    return 0;
  }

  const matrix = Array.from({ length: stLength + 1 }, (_, i) => i);

  for (let j = 1; j <= srcLength; j++) {
    let previous = matrix[0];
    matrix[0] = j;

    for (let i = 1; i <= stLength; i++) {
      const current = matrix[i];
      if (st[i - 1].codePointAt(0) === ss[j - 1].codePointAt(0)) {
        matrix[i] = previous;
      } else {
        matrix[i] = Math.min(previous, matrix[i], matrix[i - 1]) + 1;
      }
      previous = current;
    }
  }

  return matrix[stLength];
}
export default levenshtein;
