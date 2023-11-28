import {Geo, registerGridSync} from '@geodesy-wasm';
import {Coordinates as WasmCoordinates} from '@geodesy-wasm';

export class Geodesy {
  private ctx: Geo;

  /**
   * The `Geodesy` class wraps the geodesy-wasm library and provides a simpler JS friendly interface that abstracts some of the wasmness out of it.
   * @param definition - A [Geodesy](https://github.com/busstoptaktik/geodesy/blob/main/README.md) transform definition.
   * - See [Rust Geodesy](https://github.com/busstoptaktik/geodesy/blob/main/ruminations/002-rumination.md) for supported operators.
   * - There is also some support for PROJ pipeline definitions. Only the operators implemented by Rust Geodesy are supported.
   * - If you are used to using the Proj4.js CRS to CRS workflow you'll need to install and generate a pipeline definition
   *  using `projinfo -o PROJ -k operation -s <source CRS> -t <target CRS>`
   * @param gridMap - A Map of gridshift files used by the definition. The key is the grid name and the value is a `DataView` of the grid file.
   *
   * Example:
   * ---
   * ```typescript
   *  import { Geodesy } from 'geodesy-wasm';
   *  const geoCtx = new Geodesy('+proj=pipeline +step +proj=unitconvert +xy_in=deg +step +proj=utm +zone=32 +ellps=GRS80 +units=m +no_defs');
   *  const res = geoCtx.forward([[10, 60], [11, 61]]);
   * ```
   */
  constructor(definition: string, gridMap?: Record<string, DataView>) {
    this.ctx = new Geo(tidyProjString(definition));

    if (gridMap) {
      for (const [key, value] of Object.entries(gridMap)) {
        registerGridSync(key, value);
      }
    }
    // TODO: How do we cleanup wasm memory if the class is GC'd?
    // Could try Explicit Resource Management: https://iliazeus.github.io/articles/js-explicit-resource-management-en/
  }

  /**
   * Transform an array of coordinates in the forward direction of the specified definition.
   *
   * Examples:
   * ---
   *
   * 1. Using a Rust Geodesy definition:
   * ```typescript
   *  const geoCtx = new Geodesy('unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = [55, 12];
   *  const res = geoCtx.forward([copenhagen]);
   *  // res = [[6080642.1129675, 1886936.9691340544]]
   * ```
   *
   * 2. Using a PROJ pipeline definition:
   * ```typescript
   *  const geoCtx = new Geodesy('+proj=pipeline +step +proj=unitconvert +xy_in=deg +step +proj=utm +zone=32');
   *  const copenhagen = [55, 12, 100];
   *  const res = geoCtx.forward([copenhagen]);
   *  // res = [[6080642.1129675, 1886936.9691340544, 100]]
   * ```
   *
   * 3. Using Geodetic coordinates:
   *
   * ```typescript
   *  const geoCtx = new Geodesy('geo:in | unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = {x: 12, y: 55,  z: 100};
   *  const res = geoCtx.forward([copenhagen]);
   *  // res = [{x: 6080642.1129675, y: 1886936.9691340544, z: 100}]
   * ```
   * ---
   * @param coordinates - An array of 2D, 3D or 4D object or tuple coordinates.
   *  - Only Geographic Information System convention is supported eg (longitude, latitude, height, time) or (east, north, up, time).
   *  - When using angular values ensure that the definition includes a `unitconvert` step; `unitconvert xy_in=deg` or using a PROJ string (`+step +proj=unitconvert +xy_in=deg`).
   *  - Geodetic convention (latitude, longitude, height, time) is only supported by using a Rust Geodesy definition and specifying the `geo:in` macro as the first step.
   * @returns - An array of transformed coordinates in the GIS convention.
   *    - Note: If you're using the `geo:in` macro and are expecting geodetic coordinates as output be sure to add the `geo:out` macro as the last step in your definition.
   */
  public forward<T extends Coordinate>(coordinates: T[]): T[] {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates);

    this.ctx.forward(coordPtr);

    return unpackWasmCoordinates(coordPtr, coordMeta);
  }

  /**
   * Transform an array of coordinates in the inverse direction of the specified definition.
   *
   * Examples:
   * ---
   *
   * 1. Using a Rust Geodesy definition:
   * ```typescript
   *  const geoCtx = new Geodesy('unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = [[6080642.1129675, 1886936.9691340544]]
   *  const res = geoCtx.inverse([copenhagen]);
   *  // res = [55, 12];
   * ```
   *
   * 2. Using a PROJ pipeline definition:
   * ```typescript
   *  const geoCtx = new Geodesy('+proj=pipeline +step +proj=unitconvert +xy_in=deg +step +proj=utm +zone=32');
   *  const copenhagen = [[6080642.1129675, 1886936.9691340544, 100]]
   *  const res = geoCtx.inverse([copenhagen]);
   *  // res = [55, 12, 100];
   * ```
   *
   * 3. Using Geodetic coordinates:
   *
   * ```typescript
   *  const geoCtx = new Geodesy('geo:in | unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = [{x: 6080642.1129675, y: 1886936.9691340544, z: 100}]
   *  const res = geoCtx.inverse([copenhagen]);
   *  // res = {x: 12, y: 55,  z: 100};
   * ```
   * ---
   * @param coordinates - An array of 2D, 3D or 4D object or tuple coordinates.
   *  - Only Geographic Information System convention is supported eg (longitude, latitude, height, time) or (east, north, up, time).
   *  - When using angular values ensure that the definition includes a `unitconvert` step; `unitconvert xy_in=deg` or using a PROJ string (`+step +proj=unitconvert +xy_in=deg`).
   *  - Geodetic convention (latitude, longitude, height, time) is only supported by using a Rust Geodesy definition and specifying the `geo:in` macro as the first step.
   * @returns - An array of transformed coordinates in the GIS convention.
   *    - Note: If you're using the `geo:in` macro and are expecting geodetic coordinates as output be sure to add the `geo:out` macro as the last step in your definition.
   */
  public inverse<T extends Coordinate>(coordinates: T[]): T[] {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates);

    this.ctx.inverse(coordPtr);

    return unpackWasmCoordinates(coordPtr, coordMeta);
  }

  /**
   * Returns the difference between input and the result of a round trip transformation.
   * A helper method primarily used for testing.
   * Examples:
   * ---
   *
   * 1. Using a Rust Geodesy definition:
   * ```typescript
   *  const geoCtx = new Geodesy('unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = [55, 12];
   *  const res = geoCtx.roundTrip([copenhagen]);
   *  // res = [[0, 0]]
   * ```
   *
   * 2. Using a PROJ pipeline definition:
   * ```typescript
   *  const geoCtx = new Geodesy('+proj=pipeline +step +proj=unitconvert +xy_in=deg +step +proj=utm +zone=32');
   *  const copenhagen = [55, 12, 100];
   *  const res = geoCtx.roundTrip([copenhagen], false);
   *  // res = [[55, 12, 100]]
   * ```
   *
   * 3. Using Geodetic coordinates:
   *
   * ```typescript
   *  const geoCtx = new Geodesy('geo:in | unitconvert xy_in=deg | utm zone=32');
   *  const copenhagen = {x: 12, y: 55,  z: 100};
   *  const res = geoCtx.roundTrip([copenhagen]);
   *  // res = [{x: 0, y: 0, z: 0}]
   * ```
   * ---
   * @param coordinates - An array of 2D, 3D or 4D object or tuple coordinates.
   *  - Only Geographic Information System convention is supported eg (longitude, latitude, height, time) or (east, north, up, time).
   *  - When using angular values ensure that the definition includes a `unitconvert` step; `unitconvert xy_in=deg` or using a PROJ string (`+step +proj=unitconvert +xy_in=deg`).
   *  - Geodetic convention (latitude, longitude, height, time) is only supported by using a Rust Geodesy definition and specifying the `geo:in` macro as the first step.
   * @param diff - If true, return the difference between input and output. If false, return the output.
   * @returns - An array of transformed coordinates in the GIS convention.
   *    - Note: If you're using the `geo:in` macro and are expecting geodetic coordinates as output be sure to add the `geo:out` macro as the last step in your definition.
   */
  public roundTrip<T extends Coordinate>(coordinates: T[], diff = true): T[] {
    const [coordPtr, coordMeta] = createWasmCoordinates(coordinates);

    this.ctx.roundTrip(coordPtr);

    const outputCoords = unpackWasmCoordinates<T>(coordPtr, coordMeta);
    return diff ? diffCoordinates<T>(coordinates, outputCoords) : outputCoords;
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

// ----- Coordinates -----
export type CoordTuple2D = [number, number];
export type CoordTuple3D = [number, number, number];
export type CoordTuple4D = [number, number, number, number];
export type CoordObj2D = {x: number; y: number};
export type CoordObj3D = {x: number; y: number; z: number};
export type CoordObj4D = {x: number; y: number; z: number; t: number};
export type Coord3D = CoordTuple3D | CoordObj3D;
export type Coord2D = CoordTuple2D | CoordObj2D;
export type Coord4D = CoordTuple4D | CoordObj4D;
export type CoordTuple = CoordTuple2D | CoordTuple3D | CoordTuple4D;
export type CoordObj = CoordObj2D | CoordObj3D | CoordObj4D;
export type Coordinate = CoordTuple | CoordObj;

function isCoordTuple(coord: Coordinate): coord is CoordTuple {
  return Array.isArray(coord);
}

function isCoordinateTupleArray(coord: Coordinate[]): coord is CoordTuple[] {
  return coord[0] instanceof Array;
}

function isCoordinateObjectArray(coord: Coordinate[]): coord is CoordObj[] {
  return 'x' in coord[0] && 'y' in coord[0];
}

type Dimensions = 2 | 3 | 4;
type CoordMeta = {
  dimensions: Dimensions;
  inputType: 'array' | 'object';
};

/**
 * Returns a pointer to a WasmCoordinates object and the dimensions and input type of the coordinates.
 * We attempt to infer if coordinates are using angular values or not.
 * If they are by default GIS convention (longitude, latitude, height, time) is assumed. Change it with the `opts` parameter.
 * @param coords - Input Coordinates to transform
 * @param opts - Indicate coordinate conventions if we can't infer them
 * @returns - [WasmCoordinates, CoordinateType]
 */
export function createWasmCoordinates<T extends Coordinate>(
  coords: T[],
): [WasmCoordinates, CoordMeta] {
  validateCoordinates(coords);
  const dimensions = getDimensions(coords[0]);
  const inputType = coords[0] instanceof Array ? 'array' : 'object';
  const coordinateType: CoordMeta = {
    dimensions,
    inputType,
  };

  const flatCoords = flattenCoords(coords);
  return [new WasmCoordinates(flatCoords), coordinateType];
}

export function unpackWasmCoordinates<T extends Coordinate>(
  coordPtr: WasmCoordinates,
  coordMeta: CoordMeta,
): T[] {
  const coords = coordPtr.toArray();

  // This is the fastest way to unflatten an array.
  // It's faster than using Array.from(coords.subarray(i, i + dim)) because it avoids the overhead of
  // creating a new a Float64Array on each item and calling subarray on it.
  const deep: number[][] = [];
  for (let i = 0; i < coords.length; i += 4) {
    const row: number[] = [];
    for (let j = 0; j < 4; j++) {
      row.push(coords[i + j]);
    }
    deep.push(row);
  }

  if (coordMeta.inputType === 'array') {
    return deep.map(coord => coord.slice(0, coordMeta.dimensions)) as T[];
  }

  return deep.map(coord => {
    switch (coordMeta.dimensions) {
      case 2:
        return {x: coord[0], y: coord[1]};
      case 3:
        return {x: coord[0], y: coord[1], z: coord[2]};
      case 4:
        return {x: coord[0], y: coord[1], z: coord[2], t: coord[3]};
    }
  }) as T[];
}

export function diffCoordinates<T extends Coordinate>(a: T[], b: T[]): T[] {
  if (isCoordinateTupleArray(a)) {
    return a.map((coord, i) => {
      return (coord as CoordTuple).map(
        (val, j) => val - (b[i] as CoordTuple)[j],
      );
    }) as T[];
  }

  if (isCoordinateObjectArray(a)) {
    if (isCoordinateObjectArray(b)) {
      return (a as CoordObj[]).map((coord: CoordObj, i) => {
        return Object.entries(coord).map(
          // @ts-ignore - Can't get the compiler to understand b is a CoordObj[]
          ([key, val]) => val - (b[i] as CoordObj)[key],
        );
      }) as T[];
    }
  }
  return a;
}

/**
 * Flattens coordinates and pads them out to 4 dimensions.
 * @param coords
 * @returns
 */
function flattenCoords(coords: Coordinate[]): Float64Array {
  let padded: CoordTuple4D[] = [];

  if (isCoordinateTupleArray(coords)) {
    padded = coords.map(coord => [
      coord[0],
      coord[1],
      coord[2] ?? 0,
      coord[3] ?? 0,
    ]);
  }

  if (isCoordinateObjectArray(coords)) {
    padded = coords.map(coord => [
      coord.x,
      coord.y,
      // @ts-ignore
      coord?.z ?? 0,
      // @ts-ignore
      coord?.t ?? 0,
    ]);
  }

  const res = new Float64Array(coords.length * 4);
  // Fastest way to flatten an array while creating a Float64Array
  // It's faster than using Float64Array.set() because it avoids the overhead of
  // creating a new Float64Array each item.
  let index = 0;
  for (let i = 0; i < coords.length; i++) {
    for (let j = 0; j < 4; j++) {
      res[index++] = padded[i][j];
    }
  }
  return res;
}

// ---- Coordinate helpers ----
function getDimensions(coord: Coordinate): Dimensions {
  if (isCoordTuple(coord)) {
    return coord.length;
  } else {
    return Object.keys(coord).length as Dimensions;
  }
}

function dimensionsAreConsistent(
  coords: Coordinate[],
  dimensions: number,
): void {
  for (const coord of coords) {
    if (isCoordTuple(coord)) {
      if (coord.length !== dimensions) {
        throw new Error(
          `Coordinate dimensions are not consistent. Expected ${dimensions}, got ${coord.length}`,
        );
      }
    } else {
      if (Object.keys(coord).length !== dimensions) {
        throw new Error(
          `Coordinate dimensions are not consistent. Expected ${dimensions}, got ${
            Object.keys(coord).length
          }`,
        );
      }
    }
  }
}

export function validateCoordinates(coords: Coordinate[]): void {
  dimensionsAreConsistent(coords, getDimensions(coords[0]));
}
