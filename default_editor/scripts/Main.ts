import { fetchPacket, sendPackets } from "./Sockets/Sockets.ts";
import * as PacketFromClient from "./Sockets/PacketFromClient.ts";
import * as PacketToClient from "./Sockets/PacketToClient.ts";
import { Game } from "./Game.ts";

export async function main() {
  let game = new Game();

  let initPacket = (await fetchPacket()) as PacketFromClient.Init;
  console.log(`Received initialization packet: ${JSON.stringify(initPacket)}`);

  let gameName = game.getSetting("game_name", "Awgen Game Engine");
  let gameVersion = game.getSetting("game_version", "0.0.1");
  sendPackets(new PacketToClient.Init(gameName, gameVersion));

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
