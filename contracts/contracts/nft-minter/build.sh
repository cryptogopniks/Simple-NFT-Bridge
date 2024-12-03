# script for building contracts

cargo cw-optimizoor
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/workspace-optimizer:0.14.0

# rename wasm files
cd ../../artifacts
for file in *-*\.wasm; do
    prefix=${file%-*}
    mv "$file" "$prefix.wasm"
done
