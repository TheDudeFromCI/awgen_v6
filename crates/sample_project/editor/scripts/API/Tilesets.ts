export class TilesetList {
  private readonly tilesets: Map<string, Tileset> = new Map();

  private constructor() {}
}

export class Tileset {
  private name: string;
  private tileSize: number;
  private assetPath: string;
  private length: number;

  private constructor(
    name: string,
    tileSize: number,
    assetPath: string,
    length: number
  ) {
    this.name = name;
    this.tileSize = tileSize;
    this.assetPath = assetPath;
    this.length = length;
  }
}
