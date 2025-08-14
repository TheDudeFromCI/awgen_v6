import * as PacketToClient from "./Packets/PacketToClient.ts";
import { handlePacket } from "./Packets/PacketHandler.ts";
import { fetchPacket, sendPackets } from "./Packets/Sockets.ts";
import { GameSettings } from "./Settings.ts";
import { TilesetList } from "./Tilesets.ts";

/**
 * The key used to store the game name in the settings.
 */
const GAME_NAME_KEY = "game_name";

/**
 * The key used to store the game version in the settings.
 */
const GAME_VERSION_KEY = "game_version";

/**
 * A singleton class representing the game instance. This class can be used to
 * control the game client.
 */
export class Game {
  private static instance: Game | null = null;

  private readonly settings: GameSettings;
  private readonly tilesets: TilesetList;
  private running: boolean = true;

  /**
   * Initializes the game engine. This method should be called once at the start
   * of the game. This method cannot be called more than once.
   * @param title The title of the game, as it will be displayed in the client.
   * @param version The version of the game, as it will be displayed in the
   * client.
   * @returns A promise that resolves when the game exits.
   */
  public static async start(title: string, version: string): Promise<void> {
    if (Game.instance) {
      console.warn("Cannot initialize the game more than once.");
      return;
    }
    Game.instance = new Game(title, version);

    while (Game.instance.running) {
      try {
        let packet = await fetchPacket();
        await handlePacket(packet);
      } catch (error) {
        console.error(error);
        Game.shutdown();
      }
    }
  }

  /**
   * Creates a new game instance with the specified title and version. This
   * constructor will also send an initialization packet to the server with the
   * game title and version.
   * @param title The title of the game, as it will be displayed in the client.
   * @param version The version of the game, as it will be displayed in the
   * client.
   */
  private constructor(title: string, version: string) {
    //Construct private API helpers

    //@ts-expect-error
    this.settings = new GameSettings();

    // @ts-expect-error
    this.tilesets = new TilesetList();

    // Init settings and send packet
    this.settings.setSetting(GAME_NAME_KEY, title);
    this.settings.setSetting(GAME_VERSION_KEY, version);
    sendPackets(new PacketToClient.Init(title, version));
  }

  /**
   * Gets the name of the game.
   * @returns The name of the game, as specified in the settings. Defaults to
   * "Awgen Game Engine" if not set.
   * @throws Will throw an error if the game has not been initialized.
   */
  public static get title(): string {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    return Game.getSetting(GAME_NAME_KEY, "Awgen Game Engine");
  }

  /**
   * Sets the title of the game. This will update the game settings and
   * send a packet to the server to update the title in the client.
   * @param title The new title of the game.
   */
  public static set title(title: string) {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    Game.setSetting(GAME_NAME_KEY, title);

    // TODO: Send a packet to the server to update the title in the client.
  }

  /**
   * Gets the version of the game.
   * @returns The version of the game, as specified in the settings. Defaults to
   * "0.0.1" if not set.
   * @throws Will throw an error if the game has not been initialized.
   */
  public static get version(): string {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    return Game.getSetting(GAME_VERSION_KEY, "0.0.1");
  }

  /**
   * Sets the version of the game. This will update the game settings and
   * send a packet to the server to update the version in the client.
   * @param version The new version of the game.
   */
  public static set version(version: string) {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    Game.instance.settings.setSetting(GAME_NAME_KEY, version);

    // TODO: Send a packet to the server to update the title in the client.
  }

  /**
   * Shuts down the game client. This will send a shutdown packet to the
   * server and stop the game loop.
   * @param alertClient Whether to inform the client that the game is shutting
   * down. Defaults to true.
   * @throws Will throw an error if the game has not been initialized.
   */
  public static shutdown(alertClient: boolean = true): void {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    if (!Game.instance.running) {
      console.warn("Game is already shutting down.");
      return;
    }

    console.log("Shutting down...");
    if (alertClient) sendPackets(new PacketToClient.Shutdown());
    Game.instance.running = false;
  }

  /**
   * Gets a saved value from the game's settings file.
   * @param key - The key of the setting to retrieve.
   * @returns The value of the setting, or null if it does not exist.
   */
  public static getSetting(key: string): string | null;

  /**
   * Gets a saved value from the game's settings file, with an optional
   * default value.
   * @param key - The key of the setting to retrieve.
   * @param def - An optional default value to return if the setting does not
   * exist. This value will be saved to the settings file if it does not exist.
   * @returns The value of the setting, or the default value if it does not
   * exist.
   */
  public static getSetting(key: string, def: string): string;

  /**
   * Gets a saved value from the game's settings file.
   * @param key - The key of the setting to retrieve.
   * @param def - An optional default value to return if the setting does not
   * exist. This value will be saved to the settings file if it does not exist.
   * @returns The value of the setting, or null if it does not exist and no
   * default value is provided.
   */
  public static getSetting(key: string, def?: string): string | null {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    return Game.instance.settings.getSetting(key, def);
  }

  /**
   * Sets a value in the game's settings file.
   * @param key - The key of the setting to set.
   * @param value - The value to set for the setting, or null to remove it.
   */
  public static setSetting(key: string, value: string | null): void {
    if (!Game.instance) {
      throw new Error("Game has not been started. Call Game.start() first.");
    }

    Game.instance.settings.setSetting(key, value);
  }
}
