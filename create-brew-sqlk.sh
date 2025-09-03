#!/bin/bash

# # Get project metadata from Cargo.toml
PROJECT_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
DESC=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].description')
HOMEPAGE=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].repository')

# Build the release binary
cargo build --release -p "${PROJECT_NAME}"

# Navigate to the release folder
cd target/release

# Create the compressed tarball
tar -czf "${PROJECT_NAME}-mac.tar.gz" "${PROJECT_NAME}"

# Calculate the SHA256 hash of the tarball
HASH=$(shasum -a 256 "${PROJECT_NAME}-mac.tar.gz" | awk '{print $1}')

# Create the GitHub Release and attach the tarball
gh release create "v${VERSION}" \
    --title "Release v${VERSION}" \
    --notes "${DESC}" \
    "./${PROJECT_NAME}-mac.tar.gz"

# Go back to the project root
cd ../..

# ---------------------------------------------
# Now, update the Homebrew tap repository
# ---------------------------------------------

# Clone your Homebrew tap repo (if it doesn't already exist)
git clone https://github.com/sethrollinsbah/homebrew-sqlk

# Navigate into the tap repo
cd homebrew-sqlk

# Remove the old formula file (if it exists)
rm -f "${PROJECT_NAME}.rb"

PROJECT_CLASS_NAME=$(echo "$PROJECT_NAME" | awk '{print toupper(substr($0,1,1))substr($0,2)}')
# Create the new Homebrew formula file with dynamic values
cat << EOF > ${PROJECT_NAME}.rb
class ${PROJECT_CLASS_NAME} < Formula
  desc "${DESC}"
  homepage "${HOMEPAGE}"
  url "https://github.com/sethrollinsbah/sqlk/releases/download/v${VERSION}/${PROJECT_NAME}-mac.tar.gz"
  sha256 "${HASH}"
  version "${VERSION}"

  def install
    bin.install "${PROJECT_NAME}"
  end
end
EOF

# Stage, commit, and push the new formula file
git add .
git commit -m "chore(formula): automated update to v${VERSION}"
git push
cd ..
rm -rf ./homebrew-sqlk
