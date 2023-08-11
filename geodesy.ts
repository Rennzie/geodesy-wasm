import * as GeodesyWasm from './pkg';

export class Geodesy {
  private ctx: GeodesyWasm.Ctx;

  /**
   * The `Geodesy` class wraps the geodesy-wasm library and provides a simpler JS friendly interface that abstracts some of the wasmness out of it.
   * @example
   * ```typescript
   *  import { Geodesy } from 'geodesy-wasm';
   *  const geoCtx = new Geodesy('+proj=utm +zone=32 +ellps=GRS80 +units=m +no_defs');
   *  const res = geoCtx.forward([[10, 60], [11, 61]]);
   * ```
   * @param definition - A [Geodesy](https://github.com/busstoptaktik/geodesy/blob/main/README.md) transform definition.
   * - See [Rust Geodesy](https://github.com/busstoptaktik/geodesy/blob/main/ruminations/002-rumination.md) for what operations are currently supported.
   * - There is also limited support for using a PROJ pipeline definition. Not all operations are supported and it's left to the users to figure out what can and can't be done.
   * - If you are used to using the Proj4.js CRS to CRS workflow you could try building a pipeline definition using `projinfo -o PROJ -k operation -s <source CRS> -t <target CRS>`
   * @param gridKey
   * @param grid
   */
  constructor(definition: string, gridKey = 'gsb', grid?: DataView) {
    this.ctx = new GeodesyWasm.Ctx(definition, gridKey, grid);

    // An attempt to avoid leaking wasm memory if the Geodesy object is Garbage Collected
    const finalizer = new FinalizationRegistry(obj => {
      this.ctx.free();
    });
    finalizer.register(this, 'Geodesy');
  }

  /**
   * Transform an array of coordinates in the forward direction of the specified definition.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h]. Note, if inputs are angular they MUST be in radians.
   * @returns
   */
  public forward(coordinates: number[][]): number[][] {
    const [coordBufPtr, dimensions] = prepareCoordinates(coordinates);
    this.ctx.forward(coordBufPtr);

    // toArray() cleans ups the WASM memory so we don't need to call free() on the coordBufPtr
    const resArray = coordBufPtr.toArray();

    return unflattenCoords(resArray, dimensions);
  }

  /**
   * Transform an array of coordinates in the inverse direction of the specified definition.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h]. Note, if inputs are angular they MUST be in radians.
   * @returns
   */
  public inverse(coordinates: number[][]): number[][] {
    const [coordBufPtr, dimensions] = prepareCoordinates(coordinates);
    this.ctx.inverse(coordBufPtr);

    // toArray() cleans ups the WASM memory so we don't need to call free() on the coordBufPtr
    const resArray = coordBufPtr.toArray();

    return unflattenCoords(resArray, dimensions);
  }
}

// ---- Coordinate helpers ----

function prepareCoordinates(
  coordinates: number[][],
): [GeodesyWasm.CoordBuffer, GeodesyWasm.CoordDimension] {
  const dimensions = getCoordinateDimensions(coordinates);
  const flatCoords = flattenCoords(coordinates);

  // Move the coordinates into WASM memory
  const coordBufPtr = new GeodesyWasm.CoordBuffer(
    new Float64Array(flatCoords),
    dimensions,
  );

  return [coordBufPtr, dimensions];
}

function getCoordinateDimensions(
  coords: number[][],
): GeodesyWasm.CoordDimension {
  const dimensions = coords[0].length;
  if (dimensions === 2) {
    return GeodesyWasm.CoordDimension.Two;
  } else if (dimensions === 3) {
    return GeodesyWasm.CoordDimension.Three;
  } else {
    throw new Error(
      `Invalid coordinate dimensions: ${dimensions}. Coordinates must be 2D or 3D`,
    );
  }
}

function flattenCoords(coords: number[][]): number[] {
  return coords.reduce((acc, val) => acc.concat(val), []);
}

function unflattenCoords(
  coords: Float64Array,
  dimensions: GeodesyWasm.CoordDimension,
): number[][] {
  const res: number[][] = [];
  for (let i = 0; i < coords.length; i += dimensions) {
    res.push(Array.from(coords.subarray(i, i + dimensions)));
  }
  return res;
}
