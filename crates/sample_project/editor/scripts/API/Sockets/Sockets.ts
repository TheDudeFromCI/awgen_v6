import { Any as PacketToClient } from "./PacketToClient";
import { Any as PacketFromClient } from "./PacketFromClient";

/**
 * Fetches the next packet from the client.
 * @returns A promise that resolves with the packet data.
 */
export const fetchPacket =
  // @ts-ignore
  () => rustyscript.async_functions["fetchPacket"]() as PacketFromClient;

/**
 * Sends packets to the client.
 * @param packets - The packets to send to the client. Multiple packets can be
 * sent at once by passing them as separate arguments.
 */
export const sendPackets = (...packets: PacketToClient[]) =>
  // @ts-ignore
  rustyscript.functions["sendPackets"](...packets);
