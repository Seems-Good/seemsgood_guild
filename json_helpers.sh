#!/usr/bin/bash

ASSET_PATH="$PWD/events.json"
GIT_REPO="https://github.com/Jeremy-Gstein/seemsgood_guild"
REPO_NAME="seemsgood_guild"

# git remote set-url origin git@github.com:Jeremy-Gstein/seemsgood_guild
# Get the latest events.json from wowaudit and save to templates/assets/events.json
get_events() {
  curl -X 'GET' \
    'https://wowaudit.com/v1/historical_data' \
    -H 'accept: application/json' \
    -H 'Authorization: '$WOWAUDIT_TOKEN > "$ASSET_PATH"
}

# get_repo

# diff the current events.json and check if its actually populated.
#
# format_roster() {
#   curl -s -X GET "https://wowaudit.com/v1/characters" \
#     -H 'accept: application/json' \
#     -H 'Authorization: '$WOWAUDIT_TOKEN \
#     | jq -r '.[] 
#            | select(.rank == "Officer" or .rank == "Raider") 
#            | "Player { name: \"\(.name)\", class: PlayerClass::\(.class), realm: \"\(.realm)\" },"'
# }
format_roster() {
  curl -s -X GET "https://wowaudit.com/v1/characters" \
    -H "accept: application/json" \
    -H "Authorization: $WOWAUDIT_TOKEN" \
    | jq -r '.[] | select(.rank == "Officer" or .rank == "Raider") | "Player { name: \"\(.name)\", class: PlayerClass::\(.class), realm: \"\(.realm)\" },"'
}


ci() {
  local BRANCH VERSION FILE_VERSION

  [[ -f Cargo.toml ]] || { echo "Cargo.toml not found"; exit 1; }

  BRANCH=$(git branch --show-current)

  if [[ ! "$BRANCH" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Branch name must be vX.Y.Z (got: $BRANCH)"
    exit 1
  fi

  VERSION="${BRANCH#v}"

  FILE_VERSION=$(grep -m1 '^version *= *"' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')

  if [[ "$FILE_VERSION" != "$VERSION" ]]; then
    echo "Updating Cargo.toml version: $FILE_VERSION → $VERSION"

    sed -i.bak "s/^version *= *.*/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
  else
    echo "Cargo.toml already at version $VERSION"
  fi
}



help() {
  cat << 'EOF'
json_helpers.sh - a few tools to gather information from APIs we use.

USAGE:
          -e|--events) get events from wowaudit (mythic keystones)
          -r|--roster) get roster from wowaudit 
          -t|--token)  set wowaudit API token
          -i|--ci)     update Cargo.toml version to match branch
          -h|--help)   print this help menu
EXAMPLE:
          Get roster:
            ./json_helpers.sh -t $token -r 
          Get Events And Roster:
            ./json_helpers.sh -t $token -r -e

          CI:
            git branch v$VERSION
            git checkout v$VERSION
            git push origin v$VERSION 
              * This will start GH cloudflare automated build. *
              * Open PR for branch on GH and merge v$VERSION -> main *
          Bump Cargo.toml:
            ./json_helpers.sh --ci
              * assumes branch name is NOT main and follows the above format. *
EOF
}

WOWAUDIT_TOKEN=""
# Parse arguments properly
while [[ $# -gt 0 ]]; do
  case "$1" in
    -t|--token)
      shift
      if [[ -z "$1" ]]; then
        echo "Error: --token requires a value"
        exit 1
      fi
      WOWAUDIT_TOKEN="$1"
      ;;
    -e|--events) get_events ;;
    -r|--roster) format_roster ;;
    -i|--ci) ci ;;
    -u|--help) help ;;
    *) echo "Unknown argument: $1" ;;
  esac
  shift
done

# If no args, show help
if [[ -z "$WOWAUDIT_TOKEN" && $# -eq 0 ]]; then
  help
fi
