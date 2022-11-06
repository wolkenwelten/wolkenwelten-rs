#!/usr/bin/env bash
VER=`git describe --tags --exact-match 2> /dev/null || git symbolic-ref -q --short HEAD || git rev-parse --short HEAD`
ARCH="x86_64-linux"

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

rm -rf ./tmp/
mkdir -p tmp/
cp target/release/wolkenwelten tmp/wolkenwelten && \
rm -rf ./dist/ && \
mkdir dist/ && \
cd "tmp/" && tar -cJf "../dist/wolkenwelten-$VER-$ARCH.tar.xz" ./wolkenwelten
