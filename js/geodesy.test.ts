import {test, describe, expect} from 'bun:test';
import {Geodesy, Coord2D, Coord3D} from './geodesy';
``;
const gsbPipelineDefinition = `
      | tmerc inv lat_0=49 lon_0=-2 k_0=0.9996012717 x_0=400000 y_0=-100000 ellps=airy
      | gridshift grids=OSTN15_NTv2_OSGBtoETRS.gsb
      | webmerc lat_0=0 lon_0=0 x_0=0 y_0=0 ellps=WGS84
  `;

const CPH_GIS: Coord2D = [55, 12];
const CPH_GEO: Coord2D = [12, 55];
const CPH_UTM_32: Coord2D = [6080642.1129675, 1886936.9691340544];
const CPH_GIS_OB: Coord3D = {x: 55, y: 12, z: 100};
const CPH_UTM_32_OB: Coord3D = {
  x: 6080642.1129675,
  y: 1886936.9691340544,
  z: 100,
};
const LDN_BNG: Coord3D = [544748, 258372, 10];
const LDN_WebMerc: Coord3D = [13003.411436655724, 6837201.986651563, 10];

describe('Geodesy', () => {
  describe('Transform', () => {
    describe('forward()', () => {
      test('With a Geodesy Definition', () => {
        const ctx = new Geodesy('gis:in | utm zone=32');
        const res = ctx.forward([CPH_GIS]);
        expect(res).toEqual([CPH_UTM_32]);
        ctx['ctx'].free();
      });

      test('With a PROJ pipeline', () => {
        const ctx = new Geodesy(
          '+proj=pipeline +step +proj=unitconvert xy_in=deg +step +proj=utm +zone=32',
        );
        const res = ctx.forward([CPH_GIS_OB]);
        expect(res).toEqual([CPH_UTM_32_OB]);
        ctx['ctx'].free();
      });

      test('With a geodetic coordinates', () => {
        const ctx = new Geodesy('geo:in | utm zone=32');
        const res = ctx.forward([CPH_GEO]);
        expect(res).toEqual([CPH_UTM_32]);
        ctx['ctx'].free();
      });

      test('With an NTv2 gridshift file', async () => {
        // TODO: How can we use __dirname here?
        const file = Bun.file('./js/fixtures/OSTN15_NTv2_OSGBtoETRS.gsb');
        const buf = Buffer.from(await file.arrayBuffer());
        const gsb = new DataView(buf.buffer);

        const ctx = new Geodesy(gsbPipelineDefinition, {
          'OSTN15_NTv2_OSGBtoETRS.gsb': gsb,
        });

        const res = ctx.forward([LDN_BNG]);

        expect(res).toEqual([LDN_WebMerc]);

        ctx['ctx'].free();
      });
    });

    describe('inverse()', () => {
      test('With a Geodesy Definition', () => {
        const ctx = new Geodesy('gis:in | utm zone=32');
        const res = ctx.inverse([CPH_UTM_32])[0];

        res.forEach((c, i) => expect(c).toBeCloseTo(CPH_GIS[i]));

        ctx['ctx'].free();
      });

      test('With a PROJ pipeline', () => {
        const ctx = new Geodesy(
          '+proj=pipeline +step +proj=unitconvert xy_in=deg +step +proj=utm +zone=32',
        );
        const res = ctx.inverse([CPH_UTM_32_OB])[0];
        expect(res.x).toBeCloseTo(CPH_GIS_OB.x);
        expect(res.y).toBeCloseTo(CPH_GIS_OB.y);
        expect(res.z).toBeCloseTo(CPH_GIS_OB.z);
        ctx['ctx'].free();
      });

      test('With a geodetic coordinates', () => {
        const ctx = new Geodesy('geo:in | utm zone=32');
        const res = ctx.forward([CPH_GEO]);
        expect(res).toEqual([CPH_UTM_32]);
        ctx['ctx'].free();
      });

      test('With an NTv2 gridshift file', async () => {
        // TODO: How can we use __dirname here?
        const file = Bun.file('./js/fixtures/OSTN15_NTv2_OSGBtoETRS.gsb');
        const buf = Buffer.from(await file.arrayBuffer());
        const gsb = new DataView(buf.buffer);

        const ctx = new Geodesy(gsbPipelineDefinition, {
          'OSTN15_NTv2_OSGBtoETRS.gsb': gsb,
        });

        const res = ctx.inverse([LDN_WebMerc])[0];
        res.forEach((c, i) => expect(c).toBeCloseTo(LDN_BNG[i]));

        ctx['ctx'].free();
      });
    });

    describe('roundTrip()', () => {
      test('With a Geodesy Definition', () => {
        const ctx = new Geodesy('gis:in | utm zone=32');
        const res = ctx.roundTrip([CPH_GIS])[0];
        res.forEach(c => expect(c).toBeCloseTo(0));
        ctx['ctx'].free();
      });

      test('With a PROJ pipeline', () => {
        const ctx = new Geodesy(
          '+proj=pipeline +step +proj=unitconvert xy_in=deg +step +proj=utm +zone=32',
        );
        const res = ctx.roundTrip([CPH_GIS_OB])[0];
        Object.values(res).forEach((c, i) => expect(c).toBeCloseTo(0));
        ctx['ctx'].free();
      });

      test('With a geodetic coordinates', () => {
        const ctx = new Geodesy('geo:in | utm zone=32');
        const res = ctx.roundTrip([CPH_GEO])[0];
        res.forEach(c => expect(c).toBeCloseTo(0));
        ctx['ctx'].free();
      });

      test('With an NTv2 gridshift file', async () => {
        // TODO: How can we use __dirname here?
        const file = Bun.file('./js/fixtures/OSTN15_NTv2_OSGBtoETRS.gsb');
        const buf = Buffer.from(await file.arrayBuffer());
        const gsb = new DataView(buf.buffer);

        const ctx = new Geodesy(gsbPipelineDefinition, {
          'OSTN15_NTv2_OSGBtoETRS.gsb': gsb,
        });

        const res = ctx.roundTrip([LDN_BNG])[0];
        res.forEach((c, i) => expect(c).toBeCloseTo(0));
        ctx['ctx'].free();
      });

      test('should return input if diff is false', () => {
        const ctx = new Geodesy('gis:in | utm zone=32');
        const res = ctx.roundTrip([CPH_GIS], false)[0];
        res.forEach((c, i) => expect(c).toBeCloseTo(CPH_GIS[i]));
        ctx['ctx'].free();
      });
    });
  });

  describe('Errors', () => {
    test('Should error if coordinate dimensions are not consistent', () => {
      const ctx = new Geodesy('utm zone=32');
      expect(() =>
        // @ts-ignore
        ctx.inverse([[1, 2], {x: 1, y: 2}, [1, 2, 3, 4]]),
      ).toThrow();
      ctx['ctx'].free();
    });
  });
});
