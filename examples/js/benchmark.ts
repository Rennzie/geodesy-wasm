import Benchmark from 'benchmark';
const suite = new Benchmark.Suite();

import {Geodesy} from '../../pkg/node/index';
import proj4 from 'proj4';

const EPSG_27700 =
  '+inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy';
const stdPipelineDefinition = `
    +proj=pipeline
      +step ${EPSG_27700}
      +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
    `;

const proj = proj4(EPSG_27700, 'EPSG:3857');
const geodesy = new Geodesy(stdPipelineDefinition);

const coords: number[][] = [];

const bounds = {
  min: {x: -103976.3, y: -16703.87},
  max: {x: 652897.98, y: 1199851.44},
};

for (let i = 0; i < 1_000_000; i++) {
  const x = Math.random() * (bounds.max.x - bounds.min.x) + bounds.min.x;
  const y = Math.random() * (bounds.max.y - bounds.min.y) + bounds.min.y;

  coords.push([x, y]);
}

suite
  .add('Proj4.js', () => {
    const processed = coords.map(c => proj.forward(c));
  })
  .add('GeodesyWasm', () => {
    const processedGeodesy = geodesy.forward(coords);
  })
  .on('cycle', (event: any) => {
    const benchmark = event.target;

    console.log(benchmark.toString());
  })
  .on('complete', (event: any) => {
    const suite = event.currentTarget;
    const fastestOption = suite.filter('fastest').map('name');

    console.log(`The fastest option is ${fastestOption}`);
  })
  .run();
