/**
 * A packet that initializes the script engine with a name. This packet should
 * be sent to the client when the script engine is first started to initialize
 * the game engine. Sending any other packet first will result in an error.
 *
 * Subsequent packets of this type will throw a warning in the console and
 * will not be processed.
 */
export class Init {
  /**
   * The type of the packet, which is always "init" for this packet.
   */
  readonly type: "init" = "init";

  /**
   * The name of the game.
   */
  name: string;

  /**
   * The version of the game engine. This should be an array of three numbers
   * representing the major, minor, and patch versions.
   *
   * For example, [1, 0, 0] represents version 1.0.0.
   */
  version: string;

  /**
   * Creates a new initialization packet.
   *
   * @param name The name of the game.
   * @param version The version of the game engine.
   */
  constructor(name: string, version: string) {
    this.name = name;
    this.version = version;
  }
}

/**
 * A packet that contains a shutdown request.
 */
export class Shutdown {
  /**
   * The type of the packet, which is always "shutdown" for this packet.
   */
  readonly type: "shutdown" = "shutdown";
}

/**
 * A packet that contains a request to import a file into the game assets.
 */
export class ImportAsset {
  /**
   * The type of the packet, which is always "importAsset" for this packet.
   */
  readonly type: "importAsset" = "importAsset";

  /**
   * The path of the file that should be imported into the game assets.
   */
  file: string;

  /**
   * The path where the asset should be stored in the game assets.
   *
   * This must be a valid asset path, following the format:
   * <br/>`game://my/path/asset.ext`
   * or
   * <br/>`editor://my/path/asset.ext`
   */
  assetPath: string;

  /**
   * Creates a new import asset packet.
   * @param file The path of the file that should be imported into the game assets.
   * @param assetPath The path where the asset should be stored in the game assets.
   * This must be a valid asset path, following the format:
   * <br/>`game://my/path/asset.ext`
   * or
   * <br/>`editor://my/path/asset.ext`
   */
  constructor(file: string, assetPath: string) {
    this.file = file;
    this.assetPath = assetPath;
  }
}

/**
 * A union type representing all packets that can be sent to the client.
 */
export type Any = Init | Shutdown | ImportAsset;
