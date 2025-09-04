export async function waitFor<T>(
  fn: () => Promise<T>,
  predicate: (v: T) => boolean,
  opts?: { timeoutMs?: number; intervalMs?: number },
): Promise<T> {
  const timeoutMs = opts?.timeoutMs ?? 30_000;
  const intervalMs = opts?.intervalMs ?? 1_000;
  const start = Date.now();
  // eslint-disable-next-line no-constant-condition
  while (true) {
    const v = await fn();
    if (predicate(v)) return v;
    if (Date.now() - start > timeoutMs) throw new Error("waitFor: timeout");
    await new Promise((r) => setTimeout(r, intervalMs));
  }
}
