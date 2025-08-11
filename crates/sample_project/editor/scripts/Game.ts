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
export class Game {
  /**
   * Gets a saved value from the game's settings file.
   * @param key - The key of the setting to retrieve.
   * @returns The value of the setting, or null if it does not exist.
   */
  getSetting(key: string): string | null;

  /**
   * Gets a saved value from the game's settings file, with an optional
   * default value.
   * @param key - The key of the setting to retrieve.
   * @param def - An optional default value to return if the setting does not
   * exist. This value will be saved to the settings file if it does not exist.
   * @returns The value of the setting, or the default value if it does not
   * exist.
   */
  getSetting(key: string, def: string): string;

  /**
   * Gets a saved value from the game's settings file.
   * @param key - The key of the setting to retrieve.
   * @param def - An optional default value to return if the setting does not
   * exist. This value will be saved to the settings file if it does not exist.
   * @returns The value of the setting, or null if it does not exist and no
   * default value is provided.
   */
  getSetting(key: string, def?: string): string | null {
    let value = getSetting(key);
    if (value === null && def !== undefined) {
      this.setSetting(key, def);
      return def;
    }
    return value;
  }

  /**
   * Sets a value in the game's settings file.
   * @param key - The key of the setting to set.
   * @param value - The value to set for the setting, or null to remove it.
   */
  setSetting(key: string, value: string | null): void {
    setSetting(key, value);
  }
}
