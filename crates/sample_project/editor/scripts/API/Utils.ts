/**
 * Sleep for a given number of milliseconds.
 * @param ms milliseconds to sleep.
 * @returns Promise that resolves after the given number of milliseconds.
 */
export async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
