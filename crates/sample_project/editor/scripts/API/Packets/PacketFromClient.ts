/**
 * A packet that contains a shutdown request.
 */
export interface Shutdown {
  /**
   * The type of the packet, which is "shutdown" in this case.
   */
  type: "shutdown";
}

/**
 * A packet that contains a UX event, where the user has dropped a file into the
 * game window.
 */
export interface FileDrop {
  /**
   * The type of the packet, which is "fileDrop" in this case.
   */
  type: "fileDrop";

  /**
   * The path of the file that was dropped.
   */
  path: string;
}

/**
 * A union type representing all packets that can be received from the client.
 */
export type Any = Shutdown | FileDrop;
