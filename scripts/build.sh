#! /usr/bin/env bash

# The contents of this file are borrowed with love from [Kyle Barrons geoarrow_wasm project](https://github.com/kylebarron/geoarrow-rs/blob/6ceb39a8054b8d845b28ceb0fae9f123444672d7/js/scripts/build.sh)

rm -rf tmp_build pkg
mkdir -p tmp_build

if [ "$ENV" == "DEV" ]; then
  echo "Building debug version"
   BUILD="--dev"
   FLAGS="--features debug"
else
  echo "Building release version"
   BUILD="--release"
   FLAGS=""
fi

######################################
# Build node version into tmp_build/node
echo "Building node target"
wasm-pack build \
  $BUILD \
  --out-dir tmp_build/node \
  --out-name geodesy-wasm \
  --target nodejs \
  $FLAGS

# Build bundler version into tmp_build/bundler
echo "Building bundler target"
wasm-pack build \
  $BUILD \
  --out-dir tmp_build/bundler \
  --out-name geodesy-wasm \
  --target bundler \
  $FLAGS



# Compile JS Wrapper

# Compile geodesy.ts for bundler
sed 's/@geodesy-wasm/\.\/geodesy-wasm.js/g' js/geodesy.ts > tmp_build/bundler/index.ts
yarn tsc tmp_build/bundler/index.ts --outDir tmp_build/bundler --declaration --declarationDir tmp_build/bundler --target es2020 --module ES2020
rm tmp_build/bundler/index.ts


# Compile geodesy.ts for Node
sed 's/@geodesy-wasm/\.\/geodesy-wasm.js/g' js/geodesy.ts > tmp_build/node/index.ts
yarn tsc tmp_build/node/index.ts --outDir tmp_build/node --declaration --declarationDir tmp_build/node --target es2020 --module CommonJS
rm tmp_build/node/index.ts

# Copy files into pkg/
mkdir -p pkg/{node,bundler}
cp tmp_build/bundler/geodesy-wasm* pkg/bundler/
cp tmp_build/bundler/index* pkg/bundler/

cp tmp_build/node/geodesy-wasm* pkg/node
cp tmp_build/node/index* pkg/node/

cp tmp_build/bundler/{package.json,LICENSE-MIT,LICENSE-APACHE,README.md} pkg/

# Update files array in package.json using JQ
# Set module field to bundler/geodesy-wasm.js
# Set types field to bundler/geodesy-wasm.d.ts
jq '.files = ["*"] | .module="bundler/index.js" | .types="bundler/index.d.ts"' pkg/package.json > pkg/package.json.tmp

# Overwrite existing package.json file
mv pkg/package.json.tmp pkg/package.json

rm -rf tmp_build