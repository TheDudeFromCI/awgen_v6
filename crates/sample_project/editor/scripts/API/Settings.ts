const getSetting =
  // @ts-ignore
  rustyscript.functions["getSetting"] as (key: string) => string | null;

const setSetting =
  // @ts-ignore
  rustyscript.functions["setSetting"] as (
    key: string,
    value: string | null
  ) => void;

/**
 * A singleton class representing the game instance. This class can be used to
 * control the game client.
 */
export class GameSettings {
  private readonly cache: Record<string, string> = {};

  private constructor() {}

  /**
   * Gets a saved value from the game's settings file.
   * @param key - The key of the setting to retrieve.
   * @param def - An optional default value to return if the setting does not
   * exist. This value will be saved to the settings file if it does not exist.
   * @returns The value of the setting, or null if it does not exist and no
   * default value is provided.
   */
  public getSetting(key: string, def?: string): string | null {
    // Check the cache first to avoid unnecessary calls to the database.
    if (key in this.cache) {
      return this.cache[key];
    }

    let value = getSetting(key);

    if (value === null && def !== undefined) {
      this.setSetting(key, def);
      this.cache[key] = def;
      return def;
    }

    if (value !== null) {
      this.cache[key] = value;
    }

    return value;
  }

  /**
   * Sets a value in the game's settings file.
   * @param key - The key of the setting to set.
   * @param value - The value to set for the setting, or null to remove it.
   */
  public setSetting(key: string, value: string | null): void {
    setSetting(key, value);

    if (value === null) {
      delete this.cache[key];
    } else {
      this.cache[key] = value;
    }
  }
}
