import * as PacketFromClient from "./PacketFromClient.ts";
import * as PacketToClient from "./PacketToClient.ts";
import { sendPackets } from "./Sockets.ts";
import { Game } from "../Game.ts";

/**
 * Handles a packet received from the client. This method will process the
 * packet and perform the appropriate action based on its type.
 *
 * This method can be triggered manually to simulate receiving a packet.
 * @param packet The packet received from the client.
 */
export async function handlePacket(
  packet: PacketFromClient.Any
): Promise<void> {
  switch (packet.type) {
    case "shutdown":
      Game.shutdown(false);
      break;

    case "fileDrop":
      console.log("File dropped:", packet.path);
      let filename = packet.path.replace(/^.*[\\/]/, "");
      sendPackets(
        new PacketToClient.ImportAsset(
          packet.path,
          `editor://tiles/${filename}`
        )
      );
      break;
  }
}
