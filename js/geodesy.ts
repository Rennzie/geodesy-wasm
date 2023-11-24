import * as GeodesyWasm from '@geodesy-wasm';
import {
  CoordinateArray,
  CoordinateOptions,
  createWasmCoordinates,
  diffCoordinates,
  unpackWasmCoordinates,
} from './coordinates';

export class Geodesy {
  private ctx: GeodesyWasm.Geo;

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
   * @param gridMap - A Map of gridshift files used by the definition. The key is the grid name and the value is a `DataView` of the grid file.
   */
  constructor(definition: string, gridMap?: Record<string, DataView>) {
    this.ctx = new GeodesyWasm.Geo(tidyProjString(definition));

    if (gridMap) {
      for (const [key, value] of Object.entries(gridMap)) {
        this.ctx.registerGrid(key, value);
      }
    }
    // TODO: How do we cleanup wasm memory if the class is GC'd?
    // Could try Explicit Resource Management: https://iliazeus.github.io/articles/js-explicit-resource-management-en/
  }

  /**
   * Transform an array of coordinates in the forward direction of the specified definition.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h].
   *      Note, if inputs are angular they MUST be in radians.
   *      Dimensionality of the coordinates must be consistent.
   * @returns
   */
  public forward(
    coordinates: CoordinateArray,
    opts: CoordinateOptions = {},
  ): CoordinateArray {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates, opts);

    this.ctx.forward(coordPtr);

    return unpackWasmCoordinates(coordPtr, coordMeta, opts);
  }

  /**
   * Transform an array of coordinates in the inverse direction of the specified definition.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h].
   *      Note, if inputs are angular they MUST be in radians.
   *      Dimensionality of the coordinates must be consistent.
   * @returns
   */
  public inverse(
    coordinates: CoordinateArray,
    opts: CoordinateOptions = {},
  ): CoordinateArray {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates, opts);

    this.ctx.inverse(coordPtr);

    return unpackWasmCoordinates(coordPtr, coordMeta, opts);
  }

  /**
   * Returns the difference between input and the result of a roundtrip transformation.
   * A helper method primarily used for testing.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h].
   *      Note, if inputs are angular they MUST be in radians.
   *      Dimensionality of the coordinates must be consistent.
   * @param diff - If true, return the difference between input and output. If false, return the output.
   * @returns
   */
  public roundTrip(
    coordinates: CoordinateArray,
    diff = true,
    opts: CoordinateOptions = {},
  ): CoordinateArray {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates, opts);

    this.ctx.roundTrip(coordPtr);

    const outputCoords = unpackWasmCoordinates(coordPtr, coordMeta, opts);
    // diff input and output
    const diffedCoords = diffCoordinates(coordinates, outputCoords);

    return diff ? diffedCoords : outputCoords;
  }
}

// ---- Utils ----

/**
 * A temporary helper for minor differences between PROJ and Geodesy.
 * @param projString
 * @returns
 */
function tidyProjString(projString: string): string {
  if (!projString.includes('+proj=')) return projString;

  // Replace 'hgridshift' with 'gridshift'
  projString = projString.replace(/hgridshift/g, 'gridshift');

  return projString;
}

export * as GeodesyWasm from '@geodesy-wasm';
