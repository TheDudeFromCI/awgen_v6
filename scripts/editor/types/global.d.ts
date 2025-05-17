declare global {
  /**
   * Logs a message to the console.
   * @param message - The message to log to the console.
   */
  function log(message: string): void;

  /**
   * Sleeps for a specified number of milliseconds.
   * @param ms - The number of milliseconds to sleep.
   */
  function sleep(ms: number): Promise<void>;
}

export {};
