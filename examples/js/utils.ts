import {CoordTuple} from '../../pkg/node/index';
const reset = '\x1b[0m';

export const log = {
  white: console.log,
  green: (text: string, ...args: any) =>
    console.log('\x1b[32m' + text + reset, ...args),
  red: (text: string, ...args: any) =>
    console.log('\x1b[31m' + text + reset, ...args),
  blue: (text: string, ...args: any) =>
    console.log('\x1b[34m' + text + reset, ...args),
  yellow: (text: string, ...args: any) =>
    console.log('\x1b[33m' + text + reset, ...args),
};

export function colourString(num: number): [string, number] {
  if (Math.abs(num) < 0.001) {
    return ['\x1b[32m%s\x1b[0m', num];
  } else {
    return ['\x1b[31m%s\x1b[0m', num];
  }
}

export function logCoordinate(coord: CoordTuple): void {
  if (coord.length === 2) {
    console.log(coord[0].toFixed(4), coord[1].toFixed(4));
  } else {
    console.log(coord[0].toFixed(4), coord[1].toFixed(4), coord[2].toFixed(4));
  }
}

export function logCoordinates(coords: CoordTuple[]): void {
  for (let i = 0; i < coords.length; i += 1) {
    logCoordinate(coords[i]);
  }
}

export function logCoordDiff(coordsA: CoordTuple[], coordsB: CoordTuple[]) {
  for (let i = 0; i < coordsA.length; i += 1) {
    let x = colourString(coordsA[i][0] - coordsB[i][0]);
    let y = colourString(coordsA[i][1] - coordsB[i][1]);

    // @ts-ignore
    let z = colourString(coordsA[i][2] - coordsB[i][2]);

    console.log(`${x[0]} ${x[0]} ${z[0]}`, x[1], y[1], z[1]);
  }
}
