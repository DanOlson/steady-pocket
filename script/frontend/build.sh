#!/usr/bin/env bash
set -e

WORKING_DIR=$(pwd)
BUILD_DIR="$WORKING_DIR/script/build/"
FRONTEND_BUILD_PATH="$WORKING_DIR/client/build"

cd $FRONTEND_BUILD_PATH
npm run build
cd $WORKING_DIR
tar -czf "$BUILD_DIR/frontend-app.tar.gz" client/build
