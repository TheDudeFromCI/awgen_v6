import { fetchPacket, sendPackets } from "./Sockets/Sockets.ts";
import * as PacketFromClient from "./Sockets/PacketFromClient.ts";
import * as PacketToClient from "./Sockets/PacketToClient.ts";

export async function main() {
  let initPacket = (await fetchPacket()) as PacketFromClient.Init;
  console.log(`Received initialization packet: ${JSON.stringify(initPacket)}`);

  sendPackets(new PacketToClient.Init("Awgen Game Engine", [0, 0, 1]));

  let run = true;
  while (run) {
    try {
      let packet = await fetchPacket();
      switch (packet["type"]) {
        case "init":
          console.log("Initialization packet received.");
          break;
        case "shutdown":
          console.log("Shutting down...");
          run = false;
          break;
      }
    } catch (error) {
      console.error(`Error: ${error}`);
      run = false;
    }
  }
}
