import {test, describe, expect} from 'bun:test';
import {Geodesy} from './geodesy';

const DEG_TO_RAD = Math.PI / 180;
const stdPipelineDefinition = `
    +proj=pipeline
      +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
      +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
    `;

const gsbPipelineDefinition = `
      | tmerc inv lat_0=49 lon_0=-2 k_0=0.9996012717 x_0=400000 y_0=-100000 ellps=airy
      | gridshift grids=OSTN15_NTv2_OSGBtoETRS.gsb
      | webmerc lat_0=0 lon_0=0 x_0=0 y_0=0 ellps=WGS84
  `;

const COPENHAGEN_GEO_2D = [55, 12];
const LONDON_BNG_3D = [544748.5367636156, 258372.49178149243, 9.61];

describe('Geodesy', () => {
  describe('Transform', () => {
    test('Basic geographic transform', () => {
      const ctx = new Geodesy('utm zone=32');
      const res = ctx.forward([COPENHAGEN_GEO_2D.map(v => v * DEG_TO_RAD)]);
      expect(res).toEqual([[6080642.1129675, 1886936.9691340544]]);
      ctx['ctx'].free();
    });

    test('Should be able to use a PROJ string', () => {
      const ctx = new Geodesy('+proj=utm +zone=32');
      const res = ctx.forward([COPENHAGEN_GEO_2D.map(v => v * DEG_TO_RAD)]);
      expect(res).toEqual([[6080642.1129675, 1886936.9691340544]]);
      ctx['ctx'].free();
    });

    test('Should be able to use a PROJ pipeline', () => {
      const ctx = new Geodesy(stdPipelineDefinition);
      const res = ctx.roundTrip([LONDON_BNG_3D])[0];

      for (const coord of res) {
        expect(coord).toBeCloseTo(0, 5);
      }

      ctx['ctx'].free();
    });

    test('Should be able to use an NTv2 gridshift file', async () => {
      // TODO: How can we use __dirname here?
      const file = Bun.file('./js/fixtures/OSTN15_NTv2_OSGBtoETRS.gsb');
      const buf = Buffer.from(await file.arrayBuffer());
      const gsb = new DataView(buf.buffer);

      const ctx = new Geodesy(gsbPipelineDefinition, {
        'OSTN15_NTv2_OSGBtoETRS.gsb': gsb,
      });

      const res = ctx.roundTrip([LONDON_BNG_3D])[0];

      for (const coord of res) {
        expect(coord).toBeCloseTo(0, 5);
      }

      ctx['ctx'].free();
    });
  });

  describe('Errors', () => {
    test('Should error if coordinate is not 2D or 3D', () => {
      const ctx = new Geodesy('utm zone=32');
      expect(() => ctx.forward([[1, 2, 3, 4]])).toThrow();
      ctx['ctx'].free();
    });

    test('Should error if coordinate dimensions are not consistent', () => {
      const ctx = new Geodesy('utm zone=32');
      expect(() =>
        ctx.inverse([
          [1, 2],
          [1, 2, 3],
        ]),
      ).toThrow();
      ctx['ctx'].free();
    });
  });
});
