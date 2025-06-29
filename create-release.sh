#!/bin/bash
# GitHub Release Creation Script
# Usage: ./create-release.sh YOUR_GITHUB_TOKEN

set -e

GITHUB_TOKEN="$1"
REPO="hxhippy/pwgen"
TAG="v1.2.0"
RELEASE_NAME="PwGen-rust v1.2.0 - Optimized Release"

if [ -z "$GITHUB_TOKEN" ]; then
    echo "Usage: $0 YOUR_GITHUB_TOKEN"
    exit 1
fi

# Create release
echo "Creating GitHub release..."
RELEASE_DATA=$(cat <<EOF
{
  "tag_name": "$TAG",
  "target_commitish": "main",
  "name": "$RELEASE_NAME",
  "body": "# PwGen-rust v1.2.0 Release\n\n## 🚀 What's New\n- 30-40% smaller binaries through dependency optimization\n- Enhanced security with modern cryptography (SHA-256 only)\n- Flexible build options for different deployment scenarios\n- Conditional compilation for platform-specific features\n- Improved performance and reduced memory footprint\n\n## 📦 Downloads\n\n### Linux\n- **Snap Package**: pwgen-rust_1.2.0_amd64.snap (118MB)\n- **CLI Binary**: pwgen-cli-linux-x64 (5.2MB)\n- **GUI Binary**: pwgen-gui-linux-x64 (9.0MB)\n\n### Windows\n- **CLI Binary**: pwgen-cli-windows-x64.exe (4.4MB)\n- **GUI Binary**: pwgen-gui-windows-x64.exe (5.2MB)\n\n## 📋 Installation\n\n### Linux (Snap)\n\`\`\`bash\nwget https://github.com/hxhippy/pwgen/releases/download/v1.2.0/pwgen-rust_1.2.0_amd64.snap\nsudo snap install --dangerous pwgen-rust_1.2.0_amd64.snap\n\`\`\`\n\n🤖 Automated deployment",
  "draft": false,
  "prerelease": false
}
EOF
)

RELEASE_RESPONSE=$(curl -s \
  -X POST \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  "https://api.github.com/repos/$REPO/releases" \
  -d "$RELEASE_DATA")

UPLOAD_URL=$(echo "$RELEASE_RESPONSE" | grep -o '"upload_url": "[^"]*' | cut -d'"' -f4 | sed 's/{?name,label}//')

echo "Release created. Upload URL: $UPLOAD_URL"

# Upload each asset
for file in release-assets/*; do
    filename=$(basename "$file")
    echo "Uploading $filename..."
    
    curl -s \
      -X POST \
      -H "Authorization: token $GITHUB_TOKEN" \
      -H "Content-Type: application/octet-stream" \
      --data-binary @"$file" \
      "$UPLOAD_URL?name=$filename"
    
    echo "✅ Uploaded $filename"
done

echo "🎉 Release v1.2.0 created successfully!"
echo "🔗 Visit: https://github.com/$REPO/releases/tag/$TAG"