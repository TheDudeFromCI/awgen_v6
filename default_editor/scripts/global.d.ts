declare global {
  /**
   * A namespace for packets that can be sent from the script engine to the
   * client.
   */
  namespace PacketToClient {
    /**
     * A packet that contains a set of packets. All of the packets contained
     * within this collection are garmented to be processed on the same frame.
     *
     * This packet can only be sent from the script engine to the client.
     */
    export interface Set {
      /**
       * The type of the packet, which is always "set" for this packet.
       */
      type: "set";

      /**
       * The packets to be sent to the client.
       *
       * This can include multiple packets, such as those containing text, images,
       * or other data.
       */
      packets: Any[];
    }

    /**
     * A packet that contains a shutdown request.
     */
    export interface Shutdown {
      type: "shutdown";
    }

    /**
     * A union type representing all packets that can be sent to the client.
     */
    export type Any = Set | Shutdown;
  }

  /**
   * A namespace for packets that can be sent from the client to the script
   * engine.
   */
  namespace PacketFromClient {
    /**
     * A packet that contains a shutdown request.
     */
    export interface Shutdown {
      type: "shutdown";
    }

    /**
     * A union type representing all packets that can be received from the client.
     */
    export type Any = Shutdown;
  }

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

  /**
   * Fetches the next packet from the client.
   * @returns A promise that resolves with the packet data.
   */
  function fetchPacket(): Promise<PacketFromClient.Any>;

  /**
   * Sends packets to the client.
   * @param packets - The packets to send to the client. Multiple packets can be
   * sent at once by passing them as separate arguments.
   */
  function sendPackets(...packets: PacketToClient.Any[]): void;
}

export {};
