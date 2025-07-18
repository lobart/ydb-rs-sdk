#!/bin/bash

set -eux

CRATE_NAME="$1"
VERSION_OR_PART="$2"
GIT_EMAIL="$3"

declare -a GIT_TAGS
declare -a CRATES

function git_setup() {
  git config user.name "robot"
  git config user.email "$GIT_EMAIL"
}

function publish_crate() {
  local CRATE_NAME="$1"
  (
    cd "$CRATE_NAME"
    cargo publish
  )
}

function version_get() {
  local CRATE_NAME="$1"
  grep "^version\\s*=" "$CRATE_NAME/Cargo.toml" | cut -d '"' -f 2
}

function version_increment() {
  local VERSION="$1"
  local UP_PART="$2"

  local MAJOR MINOR PATCH
  IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

  case "$UP_PART" in
    major)
      MAJOR=$((MAJOR+1))
      MINOR=0
      PATCH=0
      ;;
    minor)
      MINOR=$((MINOR+1))
      PATCH=0
      ;;
    patch)
      PATCH=$((PATCH+1))
      ;;
    *)
      echo "Invalid version part: $UP_PART"
      exit 1
  esac

  echo "$MAJOR.$MINOR.$PATCH"
}

function version_set() {
  local CRATE_NAME="$1"
  local VERSION="$2"

  sed -i.bak -E "s/^version\s*=.*/version = \"$VERSION\"/" "$CRATE_NAME/Cargo.toml"
  rm "$CRATE_NAME/Cargo.toml.bak"

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

function bump_version() {
  local CRATE_NAME="$1"
  local VERSION_PART="$2"

  local CURRENT VERSION
  CURRENT=$(version_get "$CRATE_NAME")
  VERSION=$(version_increment "$CURRENT" "$VERSION_PART")

  version_set "$CRATE_NAME" "$VERSION"
  GIT_TAGS+=("$CRATE_NAME-$VERSION")
  CRATES+=("$CRATE_NAME")

  case "$CRATE_NAME" in
    ydb) version_dep_set "ydb" "$VERSION" ;;
    ydb-grpc) version_dep_set "ydb-grpc" "$VERSION" ;;
    ydb-grpc-helpers) version_dep_set "ydb-grpc-helpers" "$VERSION" ;;
    *) echo "Unexpected crate name '$CRATE_NAME'"; exit 1 ;;
  esac

  echo "$VERSION"
}

# ----------------------
# MAIN ENTRYPOINT
# ----------------------

git_setup

if [[ "$VERSION_OR_PART" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  # -----------------------
  # Mode: Tag push (version is passed directly)
  # -----------------------
  TAG_VERSION="$VERSION_OR_PART"
  ACTUAL_VERSION=$(version_get "$CRATE_NAME")

  if [[ "$ACTUAL_VERSION" != "$TAG_VERSION" ]]; then
    echo "Error: Version mismatch — tag says $TAG_VERSION but $CRATE_NAME/Cargo.toml has $ACTUAL_VERSION"
    exit 1
  fi

  echo "Verified version $ACTUAL_VERSION for $CRATE_NAME"

  cargo build --workspace --all-targets

  publish_crate "$CRATE_NAME"

  if [[ "$CRATE_NAME" == "ydb" ]]; then
    echo "$ACTUAL_VERSION" > .crate-version
  fi

else
  # -----------------------
  # Mode: Version bump (patch/minor)
  # -----------------------
  VERSION=$(bump_version "$CRATE_NAME" "$VERSION_OR_PART")

  cargo build --workspace --all-targets

  git add .
  git commit -m "bump version for $CRATE_NAME, $VERSION_OR_PART"
  for TAG in "${GIT_TAGS[@]}"; do
    git tag "$TAG"
  done

  git push origin --tags
  git push

  for CRATE in "${CRATES[@]}"; do
    publish_crate "$CRATE"
  done

  if [[ "$CRATE_NAME" == "ydb" ]]; then
    echo "$VERSION" > .crate-version
  fi
fi
