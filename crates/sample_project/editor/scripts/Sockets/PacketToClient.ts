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
 * A packet that contains a set of packets. All of the packets contained
 * within this collection are garmented to be processed on the same frame.
 *
 * This packet can only be sent from the script engine to the client.
 */
export class Set {
  /**
   * The type of the packet, which is always "set" for this packet.
   */
  readonly type: "set" = "set";

  /**
   * The packets to be sent to the client.
   *
   * This can include multiple packets, such as those containing text, images,
   * or other data.
   */
  packets: PacketToClient[];

  /**
   * Creates a new set packet.
   *
   * @param packets The packets to be sent to the client.
   */
  constructor(...packets: PacketToClient[]) {
    this.packets = packets;
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
 * A union type representing all packets that can be sent to the client.
 */
export type PacketToClient = Init | Set | Shutdown;
