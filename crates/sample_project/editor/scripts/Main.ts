import { sendPackets } from "./API/Packets/Sockets.ts";
import * as PacketToClient from "./API/Packets/PacketToClient.ts";
import { Game } from "./API/Game.ts";
import { Cube } from "./API/BlockModel.ts";
import { WorldPos } from "./API/Units.ts";
import { sleep } from "./API/Utils.ts";

export async function main() {
  Game.once("ready", async () => {
    console.log("Game is ready!");
    await sleep(5000);

    sendPackets(
      // new PacketToClient.CreateTileset(
      //   [
      //     "editor://tiles/grass.png",
      //     "editor://tiles/dirt.png",
      //     "editor://tiles/up.png",
      //     "editor://tiles/north.png",
      //     "editor://tiles/south.png",
      //     "editor://tiles/east.png",
      //     "editor://tiles/west.png",
      //   ],
      //   "game://tilesets/terrain.tiles"
      // ),
      new PacketToClient.SetTilesets("game://tilesets/terrain.tiles")
    );

    let model = new Cube();
    model.posY.tile_index = 2;
    model.posZ.tile_index = 3;
    model.negZ.tile_index = 4;
    model.posX.tile_index = 5;
    model.negX.tile_index = 6;

    let packets = [] as PacketToClient.SetBlock[];

    for (let x = -10; x <= 10; x++) {
      for (let z = -10; z <= 10; z++) {
        let position = [x, 0, z] as WorldPos;
        packets.push(new PacketToClient.SetBlock(position, model));
      }
    }

    sendPackets(...packets);
  });

  await Game.start("Awgen Game Engine", "0.0.1");
}
