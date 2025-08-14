import { Game } from "./API/Game.ts";

export async function main() {
  await Game.start("Awgen Game Engine", "0.0.1");
}
