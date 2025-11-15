#!/bin/bash
set -e

echo "Building Circuit Documentation..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Step 1: Build cargo docs
echo -e "${BLUE}Step 1: Generating Rust API documentation...${NC}"
cd ..
cargo doc --no-deps --all-features --document-private-items
echo -e "${GREEN}✓ Cargo docs generated${NC}"

# Step 2: Build mdBook
echo -e "${BLUE}Step 2: Building mdBook documentation...${NC}"
cd docs
mdbook build
echo -e "${GREEN}✓ mdBook built${NC}"

# Step 3: Copy cargo docs into mdBook output
echo -e "${BLUE}Step 3: Integrating Rust API docs...${NC}"
mkdir -p book/rustdoc
cp -r ../target/doc/* book/rustdoc/
echo -e "${GREEN}✓ API docs integrated${NC}"

# Step 4: Create index redirect for rustdoc
cat > book/rustdoc/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Circuit API Documentation</title>
    <meta http-equiv="refresh" content="0; url=circuit_core/index.html">
</head>
<body>
    <p>Redirecting to <a href="circuit_core/index.html">Circuit Core API documentation</a>...</p>
</body>
</html>
EOF
echo -e "${GREEN}✓ Created rustdoc index${NC}"

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Documentation built successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "To view the documentation:"
echo "  cd docs && mdbook serve --open"
echo ""
echo "Or open directly:"
echo "  open docs/book/index.html"
