#!/bin/bash

# cd to the root of the project

cd $(dirname $0)/..

old_version=$(git describe --tags --abbrev=0)

echo "Old version: $old_version"

NEW_VERSION=$(semver next patch $old_version)
echo "New version: $NEW_VERSION"
sed -i '' "s/^version = ".*"/version = \"$NEW_VERSION\"/" ./Cargo.toml
cargo build
git add ./Cargo.*
git commit -m "Release $NEW_VERSION"
git tag $NEW_VERSION
git push --tags
