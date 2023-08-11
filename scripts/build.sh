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
if [ -z "${TARGET+x}" ] || [ "$TARGET" == "node" ]; then
  echo "Building node target"
  wasm-pack build \
    $BUILD \
    --out-dir tmp_build/node \
    --out-name index \
    --target nodejs \
    $FLAGS
else
  echo "Skipping node target"
fi


# Build web version into tmp_build/esm
if [ -z "${TARGET+x}" ] || [ "$TARGET" == "esm" ]; then
  echo "Building esm target"
  wasm-pack build \
    $BUILD \
    --out-dir tmp_build/esm \
    --out-name index \
    --target web \
    $FLAGS
else
  echo "Skipping esm target"
fi

# Build bundler version into tmp_build/bundler
if [ -z "${TARGET+x}" ] || [ "$TARGET" == "bundler" ]; then
  echo "Building bundler target"
  wasm-pack build \
    $BUILD \
    --out-dir tmp_build/bundler \
    --out-name index \
    --target bundler \
    $FLAGS
else
  echo "Skipping bundler target"
fi
# Copy files into pkg/
mkdir -p pkg/{node,esm,bundler}

cp tmp_build/bundler/index* pkg/bundler/
cp tmp_build/esm/index* pkg/esm
cp tmp_build/node/index* pkg/node

cp tmp_build/bundler/{package.json,LICENSE,README.md} pkg/

# Create minimal package.json in esm/ folder with type: module
echo '{"type": "module"}' > pkg/esm/package.json

# Update files array in package.json using JQ
# Set module field to bundler/arrow1.js
# Set types field to bundler/arrow1.d.ts
jq '.files = ["*"] | .module="bundler/index.js" | .types="bundler/index.d.ts"' pkg/package.json > pkg/package.json.tmp

# Overwrite existing package.json file
mv pkg/package.json.tmp pkg/package.json

rm -rf tmp_build