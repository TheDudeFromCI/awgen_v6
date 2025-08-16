import { sendPackets } from "./API/Packets/Sockets.ts";
import * as PacketToClient from "./API/Packets/PacketToClient.ts";
import { Game } from "./API/Game.ts";

export async function main() {
  Game.once("ready", async () => {
    console.log("Game is ready!");
    sendPackets(
      new PacketToClient.CreateTileset(
        ["editor://tiles/grass.png", "editor://tiles/dirt.png"],
        "game://tilesets/terrain.tiles"
      )
    );
  });

  await Game.start("Awgen Game Engine", "0.0.1");
}
