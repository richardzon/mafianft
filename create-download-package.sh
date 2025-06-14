#!/bin/bash

# Create a downloadable package of the Mafia NFT project
echo "📦 Creating download package for Mafia NFT project..."

# Create package directory
mkdir -p mafianft-package

# Copy all project files
cp -r programs/ mafianft-package/
cp -r app/ mafianft-package/
cp -r backend/ mafianft-package/
cp -r scripts/ mafianft-package/
cp -r docs/ mafianft-package/
cp -r tests/ mafianft-package/
cp -r .github/ mafianft-package/

# Copy configuration files
cp Anchor.toml mafianft-package/
cp Cargo.toml mafianft-package/
cp package.json mafianft-package/
cp .gitignore mafianft-package/
cp README.md mafianft-package/
cp *.md mafianft-package/

# Create archive
tar -czf mafianft-complete-project.tar.gz mafianft-package/

echo "✅ Package created: mafianft-complete-project.tar.gz"
echo "📁 Size: $(du -h mafianft-complete-project.tar.gz | cut -f1)"

# Cleanup
rm -rf mafianft-package/

echo "🚀 Ready to download and deploy!"
