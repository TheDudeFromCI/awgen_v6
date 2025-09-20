import { sendPackets } from "./API/Packets/Sockets.ts";
import * as PacketToClient from "./API/Packets/PacketToClient.ts";
import { Game } from "./API/Game.ts";

export async function main() {
  Game.once("ready", async () => {
    console.log("Game is ready!");
    await sleep(7500);

    sendPackets(
      new PacketToClient.CreateTileset(
        [
          "editor://tiles/grass.png",
          "editor://tiles/dirt.png",
          "editor://tiles/up.png",
          "editor://tiles/north.png",
          "editor://tiles/south.png",
          "editor://tiles/east.png",
          "editor://tiles/west.png",
        ],
        "game://tilesets/terrain.tiles"
      )
    );
  });

  await Game.start("Awgen Game Engine", "0.0.1");
}

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
