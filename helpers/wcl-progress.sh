#!/usr/bin/env bash
# Warcraft Logs - Current Mythic progression boss
# Deps: curl, jq
# Usage: bash wcl_guild_progression.sh

set -euo pipefail

ENV_FILE="$(dirname "$0")/.env"
if [[ ! -f "$ENV_FILE" ]]; then
  echo "❌  .env file not found at $ENV_FILE" >&2
  exit 1
fi
set -o allexport
source <(grep -E '^\s*[A-Za-z_][A-Za-z0-9_]*=' "$ENV_FILE")
set +o allexport

for var in WCL_CLIENT_ID WCL_CLIENT_SECRET WCL_GUILD_ID WCL_ZONE_ID; do
  if [[ -z "${!var:-}" ]]; then
    echo "❌  Missing required .env variable: $var" >&2
    exit 1
  fi
done

TOKEN_URL="https://www.warcraftlogs.com/oauth/token"
API_URL="https://www.warcraftlogs.com/api/v2/client"

echo "🔑  Authenticating…" >&2
TOKEN=$(curl -sf -X POST "$TOKEN_URL" \
  -u "${WCL_CLIENT_ID}:${WCL_CLIENT_SECRET}" \
  -d "grant_type=client_credentials" \
  | jq -r '.access_token')

if [[ -z "$TOKEN" || "$TOKEN" == "null" ]]; then
  echo "❌  Failed to obtain access token." >&2
  exit 1
fi

echo "📡  Fetching Mythic progression for guild ${WCL_GUILD_ID}, zone ${WCL_ZONE_ID}…" >&2

PAYLOAD=$(jq -n \
  --argjson guildID "$WCL_GUILD_ID" \
  --argjson zoneID  "$WCL_ZONE_ID" \
  '{ query: "{ progressRaceData { progressRace(guildID: \($guildID), zoneID: \($zoneID), difficulty: 5) } }" }')

RESPONSE=$(curl -sf -X POST "$API_URL" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD")

if echo "$RESPONSE" | jq -e '.errors' > /dev/null 2>&1; then
  echo "❌  GraphQL errors:" >&2
  echo "$RESPONSE" | jq '.errors' >&2
  exit 1
fi

# Extract the current progression encounter (the boss they are on right now)
# currentEncounterId points to the boss currently being progressed
echo "$RESPONSE" | jq '
  .data.progressRaceData.progressRace[0] as $guild |
  $guild.currentEncounterId as $currentId |
  ($guild.encounters[] | select(.id == $currentId)) as $enc |
  {
    id:                         $enc.id,
    name:                       $enc.name,
    shortName:                  $enc.shortName,
    backgroundImageUrl:         $enc.backgroundImageUrl,
    backgroundImageFallbackUrl: $enc.backgroundImageFallbackUrl,
    iconImageUrl:               $enc.iconImageUrl,
    transparentImageUrl:        $enc.transparentImageUrl,
    isKilled:                   $enc.isKilled,
    killedAtTimestamp:          $enc.killedAtTimestamp,
    youtubeEmbedUrl:            $enc.youtubeEmbedUrl,
    showStats:                  $enc.showStats,
    bestPercent:                $enc.bestPercent,
    bestPercentForDisplay:      $enc.bestPercentForDisplay,
    pullCount:                  $enc.pullCount
  }
'
