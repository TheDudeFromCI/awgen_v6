/**
 * A packet that contains a shutdown request.
 */
export interface Shutdown {
  type: "shutdown";
}

/**
 * A union type representing all packets that can be received from the client.
 */
export type PacketFromClient = Shutdown;
