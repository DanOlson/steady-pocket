#!/usr/bin/env bash
set -e

WORKING_DIR=$(pwd)
BUILD_DIR="$WORKING_DIR/script/build/"
FRONTEND_BUILD_PATH="$WORKING_DIR/client/build"
FRONTEND_TMP_DIR=/opt/smart-money-frontend/tmp
FRONTEND_CURRENT_DIR=/opt/smart-money-frontend/current
FRONTEND_PREV_DIR=/opt/smart-money-frontend/prev

ssh pizero << EOF
  mkdir -p $FRONTEND_TMP_DIR
  mkdir -p $FRONTEND_CURRENT_DIR
  mkdir -p $FRONTEND_PREV_DIR

  rm -rf "$FRONTEND_TMP_DIR/client"
  rm -rf "$FRONTEND_PREV_DIR/client"
EOF

rsync -av "$BUILD_DIR/frontend-app.tar.gz" "pizero:$FRONTEND_TMP_DIR/frontend-app.tar.gz"

ssh pizero << EOF
  cd $FRONTEND_TMP_DIR && tar -xzvf frontend-app.tar.gz
  mv "$FRONTEND_CURRENT_DIR/client" "$FRONTEND_PREV_DIR/"
  mv "$FRONTEND_TMP_DIR/client" "$FRONTEND_CURRENT_DIR/"
  ln -snf "$FRONTEND_CURRENT_DIR/client" /opt/smart-money/client
EOF
