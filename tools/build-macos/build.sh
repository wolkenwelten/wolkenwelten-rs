#!/usr/bin/env bash
VER=`git describe --tags --exact-match 2> /dev/null || git symbolic-ref -q --short HEAD || git rev-parse --short HEAD`
ARCH="x86_64-macos"

function cg {
  function git_root {
    local top; top="$(git rev-parse --show-cdup)"
    top="${top:-./}"
    local super_root; super_root="$(git rev-parse --show-superproject-working-tree)"
    if [[ "$super_root" ]]; then
      printf '%s' "$top../"
      ( cd "$top../" && git_root || return )
    fi
    printf '%s' "$top"
  }
  local tree_root
  tree_root="$(git_root)"
  [[ "x${tree_root}" != "x./" ]] && cd "${tree_root}" && return || return 0
}
cg

cargo build --release --locked
strip -sg target/release/wolkenwelten

rm -rf ./tmp/
mkdir -p "tmp/WolkenWelten.app/Contents/MacOS"
mkdir -p "tmp/WolkenWelten.app/Contents/Resources"
cp "target/release/wolkenwelten" "tmp/WolkenWelten.app/Contents/MacOS/wolkenwelten" && \
cp "tools/build-macos/wolkenwelten.icns" "tmp/WolkenWelten.app/Contents/Resources/wolkenwelten.icns" && \
cp "tools/build-macos/Info.plist" "tmp/WolkenWelten.app/Contents/Info.plist" && \
rm -rf ./dist/ && \
mkdir dist/ && \
cd "tmp/" && tar -cJf "../dist/wolkenwelten-$VER-$ARCH.tar.xz" ./WolkenWelten.app