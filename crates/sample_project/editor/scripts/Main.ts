import { fetchPacket, sendPackets } from "./Sockets/Sockets.ts";
import * as PacketToClient from "./Sockets/PacketToClient.ts";
import { Game } from "./Game.ts";

export async function main() {
  let game = new Game();

  let gameName = game.getSetting("game_name", "Awgen Game Engine");
  let gameVersion = game.getSetting("game_version", "0.0.1");
  sendPackets(new PacketToClient.Init(gameName, gameVersion));

  let run = true;
  while (run) {
    try {
      let packet = await fetchPacket();
      switch (packet["type"]) {
        case "shutdown":
          console.log("Shutting down...");
          run = false;
          break;
      }
    } catch (error) {
      console.error(error);
      run = false;
    }
  }
}
