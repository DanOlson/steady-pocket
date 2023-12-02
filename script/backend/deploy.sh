#!/usr/bin/env bash
set -e

WORKING_DIR=$(pwd)
API_ARTIFACT_PATH="$WORKING_DIR/target/arm-unknown-linux-gnueabihf/release/steady-pocket"

rsync -av $API_ARTIFACT_PATH pizero:/opt/smart-money/
ssh pizero << EOF
  sudo setcap 'cap_net_bind_service=+ep' /opt/smart-money/steady-pocket
  sudo systemctl restart steady-pocket
EOF
