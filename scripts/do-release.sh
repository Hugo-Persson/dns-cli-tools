#!/bin/bash
set -euo pipefail

# cd to the root of the project
cd "$(dirname "$0")/.." || exit 1

old_version=$(git describe --tags --abbrev=0)
echo "Old version: $old_version"

# Bump patch version without depending on the `semver` npm CLI.
if [[ ! "$old_version" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
  echo "error: last tag '$old_version' is not a MAJOR.MINOR.PATCH version" >&2
  exit 1
fi
NEW_VERSION="${BASH_REMATCH[1]}.${BASH_REMATCH[2]}.$((BASH_REMATCH[3] + 1))"
echo "New version: $NEW_VERSION"

sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" ./Cargo.toml
cargo build
git cliff --tag "$NEW_VERSION" -o CHANGELOG.md
git add CHANGELOG.md
git add ./Cargo.*
git commit -m "Release $NEW_VERSION"
git tag "$NEW_VERSION"

git push
git push --tags
