import * as GeodesyWasm from '@geodesy-wasm';

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
    if (gridMap) {
      for (const [key, value] of Object.entries(gridMap)) {
        GeodesyWasm.registerGrid(key, value);
      }
    }

    this.ctx = new GeodesyWasm.Geo(tidyProjString(definition));

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
  public forward(coordinates: number[][]): number[][] {
    const [coordBufPtr, dimensions] = prepareCoordinates(coordinates);
    this.ctx.forward(coordBufPtr);

    // toArray() cleans ups the WASM memory so we don't need to call free() on the coordBufPtr
    const resArray = coordBufPtr.toArray();

    return unflattenCoords(resArray, dimensions);
  }

  /**
   * Transform an array of coordinates in the inverse direction of the specified definition.
   * @param coordinates - Coordinates can be 2D or 3D and are ordered [east, north, up] or [lon, lat, h].
   *      Note, if inputs are angular they MUST be in radians.
   *      Dimensionality of the coordinates must be consistent.
   * @returns
   */
  public inverse(coordinates: number[][]): number[][] {
    const [coordBufPtr, dimensions] = prepareCoordinates(coordinates);
    this.ctx.inverse(coordBufPtr);

    // toArray() cleans ups the WASM memory so we don't need to call free() on the coordBufPtr
    const resArray = coordBufPtr.toArray();

    return unflattenCoords(resArray, dimensions);
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
  public roundTrip(coordinates: number[][], diff = true): number[][] {
    const [coordBufPtr, dimensions] = prepareCoordinates(coordinates);
    this.ctx.roundTrip(coordBufPtr);

    const outputCoords = unflattenCoords(coordBufPtr.toArray(), dimensions);
    // diff input and output
    const diffedCoords = coordinates.map((coord, i) =>
      coord.map((val, j) => val - outputCoords[i][j]),
    );

    return diff ? diffedCoords : outputCoords;
  }
}

// ---- Coordinate helpers ----

function prepareCoordinates(
  coordinates: number[][],
): [GeodesyWasm.CoordBuffer, GeodesyWasm.CoordDimension] {
  const dimensions = getCoordinateDimensions(coordinates);
  const flatCoords = flattenCoords(coordinates);

  // Move the coordinates into WASM memory
  const coordBufPtr = new GeodesyWasm.CoordBuffer(flatCoords, dimensions);

  return [coordBufPtr, dimensions];
}

function getCoordinateDimensions(
  coords: number[][],
): GeodesyWasm.CoordDimension {
  const firstCoordLength = coords[0].length;
  let dimensions: GeodesyWasm.CoordDimension;
  if (firstCoordLength === 2) {
    dimensions = GeodesyWasm.CoordDimension.Two;
  } else if (firstCoordLength === 3) {
    dimensions = GeodesyWasm.CoordDimension.Three;
  } else {
    throw new Error(
      `Invalid coordinate dimensions: ${firstCoordLength}. Coordinates must be 2D or 3D`,
    );
  }

  dimensionsAreConsistent(coords, dimensions);

  return dimensions;
}

function dimensionsAreConsistent(
  coords: number[][],
  dimensions: GeodesyWasm.CoordDimension,
): void {
  const dim = dimensions === GeodesyWasm.CoordDimension.Two ? 2 : 3;
  for (const coord of coords) {
    if (coord.length !== dim) {
      throw new Error(
        `Coordinate dimensions are not consistent. Expected ${dim}, got ${coord.length}`,
      );
    }
  }
}

function flattenCoords(coords: number[][]): Float64Array {
  const dimension = coords[0].length;
  const res = new Float64Array(coords.length * dimension);

  // Fastest way to flatten an array while creating a Float64Array
  // It's faster than using Float64Array.set() because it avoids the overhead of
  // creating a new Float64Array each item.
  let index = 0;
  for (let i = 0; i < coords.length; i++) {
    for (let j = 0; j < dimension; j++) {
      res[index++] = coords[i][j];
    }
  }
  return res;
}

function unflattenCoords(
  coords: Float64Array,
  dimensions: GeodesyWasm.CoordDimension,
): number[][] {
  const dim = dimensions === GeodesyWasm.CoordDimension.Two ? 2 : 3;

  // This is the fastest way to unflatten an array.
  // It's faster than using Array.from(coords.subarray(i, i + dim)) because it avoids the overhead of
  // creating a new a Float64Array on each item and calling subarray on it.
  const res: number[][] = [];
  for (let i = 0; i < coords.length; i += dim) {
    const row: number[] = [];
    for (let j = 0; j < dim; j++) {
      row.push(coords[i + j]);
    }
    res.push(row);
  }
  return res;
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
