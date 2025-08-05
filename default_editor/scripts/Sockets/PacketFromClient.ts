/**
 * A packet that initializes the script engine with a project folder. This
 * packet is sent from the client on startup to initialize the script
 * engine.
 */
export interface Init {
  /**
   * The type of the packet, which is always "init" for this packet.
   */
  type: "init";

  /**
   * The absolute path to the project folder containing the game files.
   */
  projectFolder: string;
}

/**
 * A packet that contains a shutdown request.
 */
export interface Shutdown {
  type: "shutdown";
}

/**
 * A union type representing all packets that can be received from the client.
 */
export type PacketFromClient = Init | Shutdown;
