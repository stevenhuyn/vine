#!/bin/bash
set -e

echo "Running post-create setup..."
echo "Current user: $(whoami)"
echo "Current UID: $(id -u)"

# Fix ownership of the workspace
echo "Fixing workspace permissions..."
sudo chown -R ubuntu:ubuntu /workspaces

# Clean any root-owned build artifacts
if [ -d "/workspaces/vine/target" ]; then
    echo "Fixing target directory permissions..."
    sudo chown -R ubuntu:ubuntu /workspaces/vine/target
fi

echo "Post-create setup complete!"