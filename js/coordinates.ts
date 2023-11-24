import {Coordinates as WasmCoordinates} from '@geodesy-wasm';

export type CoordTuple2D = [number, number];
export type CoordTuple3D = [number, number, number];
export type CoordTuple4D = [number, number, number, number];
export type CoordObj2D = {x: number; y: number};
export type CoordObj3D = {x: number; y: number; z: number};
export type CoordObj4D = {x: number; y: number; z: number; t: number};
export type Coord3D = CoordTuple3D | CoordObj3D;
export type Coord2D = CoordTuple2D | CoordObj2D;
export type Coord4D = CoordTuple4D | CoordObj4D;
export type CoordinateTupleArray =
  | CoordTuple2D[]
  | CoordTuple3D[]
  | CoordTuple4D[];
export type CoordinateObjectArray = CoordObj2D[] | CoordObj3D[] | CoordObj4D[];
export type Coordinate = Coord2D | Coord3D | Coord4D;
export type CoordinateArray = Coord2D[] | Coord3D[] | Coord4D[];

type Dimensions = 2 | 3 | 4;
type CoordMeta = {
  dimensions: Dimensions;
  inputType: 'array' | 'object';
  angularDetected?: boolean;
};

export type CoordinateOptions = {
  /** Force coordinates to be (latitude, longitude, height, time) */
  geo?: boolean;
  /** Force coordinates to be (longitude, latitude, height, time) */
  gis?: boolean;
  /** Force coordinates to be (easting, northing, height, time) */
  cart?: boolean;
};

function isCoordinateTupleArray(
  coord: CoordinateArray,
): coord is CoordinateTupleArray {
  return coord[0] instanceof Array;
}

function isCoordinateObjectArray(
  coord: CoordinateArray,
): coord is CoordinateObjectArray {
  return 'x' in coord[0] && 'y' in coord[0];
}

/**
 * Returns a pointer to a WasmCoordinates object and the dimensions and input type of the coordinates.
 * We attempt to infer if coordinates are using angular values or not.
 * If they are by default GIS convention (longitude, latitude, height, time) is assumed. Change it with the `opts` parameter.
 * @param coords - Input Coordinates to transform
 * @param opts - Indicate coordinate conventions if we can't infer them
 * @returns - [WasmCoordinates, CoordinateType]
 */
export function createWasmCoordinates(
  coords: CoordinateArray,
  opts: CoordinateOptions = {},
): [WasmCoordinates, CoordMeta] {
  validateCoordinates(coords);
  const dimensions = getDimensions(coords[0]);
  const usesAngularValues = hasAngularValues(coords[0]);
  const inputType = coords[0] instanceof Array ? 'array' : 'object';
  const coordinateType: CoordMeta = {
    dimensions,
    inputType,
    angularDetected: usesAngularValues,
  };

  const flatCoords = flattenCoords(coords);

  if (opts.cart) return [WasmCoordinates.fromCart(flatCoords), coordinateType];
  if (usesAngularValues) {
    if (opts.geo) return [WasmCoordinates.fromGeo(flatCoords), coordinateType];
    if (opts.gis) return [WasmCoordinates.fromGis(flatCoords), coordinateType];

    return [WasmCoordinates.fromGis(flatCoords), coordinateType];
  }

  return [WasmCoordinates.fromRaw(flatCoords), coordinateType];
}

export function unpackWasmCoordinates(
  coordPtr: WasmCoordinates,
  coordMeta: CoordMeta,
  opts: CoordinateOptions = {},
): CoordinateArray {
  if (opts.cart) return unflattenCoords(coordPtr.toArray(), coordMeta);
  if (opts.geo) return unflattenCoords(coordPtr.toGeoArray(), coordMeta);
  if (opts.gis) return unflattenCoords(coordPtr.toGisArray(), coordMeta);

  if (coordMeta.angularDetected)
    return unflattenCoords(coordPtr.toGisArray(), coordMeta);

  return unflattenCoords(coordPtr.toArray(), coordMeta);
}

export function diffCoordinates(
  a: CoordinateArray,
  b: CoordinateArray,
): CoordinateArray {
  throw new Error('Not implemented');
  return a;
}

function unflattenCoords(
  coords: Float64Array,
  coordMeta: CoordMeta,
): CoordinateArray {
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
    return deep.map(coord =>
      coord.slice(0, coordMeta.dimensions),
    ) as CoordinateArray;
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
  });
}

function flattenCoords(coords: CoordinateArray): Float64Array {
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
/**
 * Checks if a coordinate is using angular values.
 * If both the first and second values are between -180 and 180, it's assumed
 * that the values are in degrees or radians.
 * @param coord - A coordinate
 * @returns
 */
function hasAngularValues(coord: Coordinate): boolean {
  if (coord instanceof Array) {
    const first = coord[0];
    const second = coord[1];

    return -180 <= first && first <= 180 && -180 <= second && second <= 180;
  } else {
    return (
      -180 <= coord.x && coord.x <= 180 && -180 <= coord.y && coord.y <= 180
    );
  }
}

function getDimensions(coord: Coordinate): Dimensions {
  if (coord instanceof Array) {
    return coord.length;
  } else {
    return Object.keys(coord).length as Dimensions;
  }
}

function dimensionsAreConsistent(
  coords: CoordinateArray,
  dimensions: number,
): void {
  for (const coord of coords) {
    if (coord instanceof Array) {
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

export function validateCoordinates(coords: CoordinateArray): void {
  dimensionsAreConsistent(coords, getDimensions(coords[0]));
}
