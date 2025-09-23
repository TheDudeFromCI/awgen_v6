/**
 * The 3D position of a chunk represented as a tuple of three numbers [x, y, z].
 */
export type WorldPos = [x: number, y: number, z: number];

/**
 * A 2x2 matrix represented as a tuple of four numbers [m00, m01, m10, m11].
 */
export type Mat2 = [number, number, number, number];

export namespace Mat2Math {
  export const IDENTITY: Mat2 = [1, 0, 0, 1];
  export const ROTATE_CW: Mat2 = [0, 1, -1, 0];
  export const ROTATE_CCW: Mat2 = [0, -1, 1, 0];
  export const MIRROR_X: Mat2 = [1, 0, 0, -1];
  export const MIRROR_Y: Mat2 = [-1, 0, 0, 1];

  export function mul(a: Mat2, b: Mat2): Mat2 {
    return [
      a[0] * b[0] + a[1] * b[2],
      a[0] * b[1] + a[1] * b[3],
      a[2] * b[0] + a[3] * b[2],
      a[2] * b[1] + a[3] * b[3],
    ];
  }
}
