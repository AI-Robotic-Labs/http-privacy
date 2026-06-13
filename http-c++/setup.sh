#!/bin/bash
# Setup script for C++ HTTP server (Ubuntu/Debian)

echo "Installing dependencies..."
sudo apt update
sudo apt install -y g++ cmake libboost-all-dev libssl-dev libspdlog-dev

echo "Creating build directory..."
mkdir -p build
cd build

echo "Running CMake..."
cmake ..

echo "Compiling..."
make

echo "Build complete! Run './http_server' to start the server."