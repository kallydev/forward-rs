#!/bin/bash

project="forward"
release=$1

declare -A platforms=(
  ["linux_x86-64"]="x86_64-unknown-linux-gnu"
)

for platform in "${!platforms[@]}"; do
  target="${platforms[$platform]}"
  rustup target add "$target"
  if [[ $release == "--release" ]]; then
    mkdir -p release
    cargo build --target="$target" --release
    strip -s ./target/"$target"/release/"$project"
    upx -9 ./target/"$target"/release/"$project"
    7z a ./release/"$project"_"$platform".7z ./target/"$target"/release/"$project" README.md LICENSE
    sha1sum ./release/"$project"_"$platform".7z > ./release/"$project"_"$platform".7z.sha1
  else
    mkdir -p build
    cargo build --target="$target"
    cp target/"$target"/debug/"$project" build/"$project"_"$target"
  fi
done
