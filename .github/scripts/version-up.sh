#!/bin/bash

set -eux

CRATE_NAME="$1"
VERSION="$2"
GIT_EMAIL="$3"

declare -a GIT_TAGS
declare -a CRATES

function git_setup() {
  git config user.name "robot"
  git config user.email "$GIT_EMAIL"
}

#function publish_crate() {
#  local CRATE="$1"
#  (
#    cd "$CRATE"
#    cargo publish
#  )
#}

function version_set() {
  local CRATE="$1"
  local VERSION="$2"

  # Update version in Cargo.toml
  sed -i.bak -E "s/^version\s*=.*/version = \"$VERSION\"/" "$CRATE/Cargo.toml"
  rm -f "$CRATE/Cargo.toml.bak"

  # Update version in README if needed
  sed -i.bak -E "s/^ydb\s*=.*/ydb = \"$VERSION\"/" README.md || true
  rm -f README.md.bak
}

function version_dep_set() {
  local DEP_NAME="$1"
  local VERSION="$2"

  for FILE in $(find . -mindepth 2 -maxdepth 2 -name Cargo.toml); do
    sed -i.bak -E "s|^$DEP_NAME\s*=.*|$DEP_NAME = { version = \"$VERSION\", path = \"../$DEP_NAME\" }|" "$FILE"
    rm -f "$FILE.bak"
  done
}

function handle_crate_version() {
  local CRATE="$1"
  local VERSION="$2"

  version_set "$CRATE" "$VERSION"
  GIT_TAGS+=("$CRATE-$VERSION")
  CRATES+=("$CRATE")

  case "$CRATE" in
    ydb) version_dep_set "ydb" "$VERSION" ;;
    ydb-grpc) version_dep_set "ydb-grpc" "$VERSION" ;;
    ydb-grpc-helpers) version_dep_set "ydb-grpc-helpers" "$VERSION" ;;
    *) echo "Unexpected crate name: $CRATE"; exit 1 ;;
  esac
}

# ---------------------
# Script execution
# ---------------------

git_setup

handle_crate_version "$CRATE_NAME" "$VERSION"

# Rebuild lock file
cargo build --workspace --all-targets

# Commit and tag
git add .
git commit -m "Set version to $VERSION for $CRATE_NAME"

# Push changes
git push origin HEAD

# Publish all crates
  #for CRATE in "${CRATES[@]}"; do
   # publish_crate "$CRATE"
  #done

# Output version to file for GitHub Release step
if [[ "$CRATE_NAME" == "ydb" ]]; then
  echo "$VERSION" > .crate-version
fi
