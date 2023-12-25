function jaccardIndex(a: string, b: string): number {
  if (a.length === 0 || b.length === 0) {
    return 0;
  }

  const aCodePoints = new Set(
    Array.from(a.toLowerCase(), (c) => c.codePointAt(0))
  );
  const bCodePoints = new Set();
  const intersection = new Set();

  for (let i = 0; i < b.length; i++) {
    const bc = b[i].toLowerCase().codePointAt(0);
    bCodePoints.add(bc);
    if (aCodePoints.has(bc)) {
      intersection.add(bc);
    }
  }

  return (
    intersection.size /
    (aCodePoints.size + bCodePoints.size - intersection.size)
  );
}

export default jaccardIndex;
