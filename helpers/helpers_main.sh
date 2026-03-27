#!/usr/bin/env bash
# Warcraft Logs - Current Mythic progression boss
# Deps: curl, jq

pretty_load() {
  local DURATION=${1:-3}
  local LABEL=${2:-"Loading"}
  local FRAMES=('\' '|' '-' '/')
  local END_TIME=$(( $(date +%s) + DURATION ))
 
  while [ $(date +%s) -lt $END_TIME ]; do
    for FRAME in "${FRAMES[@]}"; do
      printf "\r  [%s]  %s  " "$FRAME" "$LABEL"
      sleep 0.1
    done
  done
  printf "\r  [✔]  %s  \n" "$LABEL"
}

set -euo pipefail

ENV_FILE="$(dirname "$0")/.env"
if [[ ! -f "$ENV_FILE" ]]; then
  echo "❌  .env file not found at $ENV_FILE" >&2
  exit 1
fi
set -o allexport
source <(grep -E '^\s*[A-Za-z_][A-Za-z0-9_]*=' "$ENV_FILE")
set +o allexport

for var in WCL_CLIENT_ID WCL_CLIENT_SECRET WCL_GUILD_ID WCL_ZONE_ID WOWAUDIT_TOKEN; do
  if [[ -z "${!var:-}" ]]; then
    echo "❌  Missing required .env variable: $var" >&2
    exit 1
  fi
done

./json_helpers.sh -t "$WOWAUDIT_TOKEN" --events 
./wcl-progress.sh > progress.json
pretty_load 5 "Uploading Files"
./handle_r2_upload.sh
pretty_load 0 "Finished Uploading files! Refresh https://seemsgood.org"
./json_helpers.sh -t "$WOWAUDIT_TOKEN" --roster > roster.json && cat roster.json
