import { Mat2, Mat2Math } from "./Units.ts";

/**
 * TileRotation class to store the UV rotation matrix for tiles faces.
 */
export class TileFace {
  public tile_index: number = 0;
  public rotation: Mat2 = Mat2Math.IDENTITY.slice() as Mat2;

  /**
   * Rotates the Mat2 90 degrees clockwise.
   */
  public rotateClockwise(): void {
    this.rotation = Mat2Math.mul(this.rotation, Mat2Math.ROTATE_CW);
  }

  /**
   * Rotates the Mat2 90 degrees counter-clockwise.
   */
  public rotateCounterClockwise(): void {
    this.rotation = Mat2Math.mul(this.rotation, Mat2Math.ROTATE_CCW);
  }

  /**
   * Mirrors the Mat2 along the X axis.
   */
  public mirrorX(): void {
    this.rotation = Mat2Math.mul(this.rotation, Mat2Math.MIRROR_X);
  }

  /**
   * Mirrors the Mat2 along the Y axis.
   */
  public mirrorY(): void {
    this.rotation = Mat2Math.mul(this.rotation, Mat2Math.MIRROR_Y);
  }
}

/**
 * BlockModel type which can be either Empty or Cube.
 */
export type BlockModel = Empty | Cube;

/**
 * Empty class representing an empty block model.
 */
export class Empty {
  /**
   * The type of the block model, which is always "empty" for this class.
   */
  public readonly type: "empty" = "empty";
}

/**
 * Cube class representing a cube block model with tile faces for each side.
 */
export class Cube {
  /**
   * The type of the block model, which is always "cube" for this class.
   */
  public readonly type: "cube" = "cube";

  /**
   * The tile face for the top side of the cube.
   */
  public posY: TileFace = new TileFace();

  /**
   * The tile face for the north side of the cube.
   */
  public posZ: TileFace = new TileFace();

  /**
   * The tile face for the south side of the cube.
   */
  public negZ: TileFace = new TileFace();

  /**
   * The tile face for the east side of the cube.
   */
  public posX: TileFace = new TileFace();

  /**
   * The tile face for the west side of the cube.
   */
  public negX: TileFace = new TileFace();

  /**
   * Creates a new Cube block model and initializes the rotations of its tile
   * faces.
   */
  public constructor() {
    this.posX.rotateCounterClockwise();
    this.negX.rotateClockwise();
    this.negZ.rotateClockwise();
    this.negZ.rotateClockwise();
  }
}
