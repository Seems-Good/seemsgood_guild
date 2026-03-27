#!/usr/bin/env bash

# ============================================================
# Upload progress.json & events.json to R2
# Uses curl + openssl only (no AWS CLI needed)
# ============================================================
# Required .env vars:
#   R2_ACCOUNT_ID, R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY, R2_BUCKET_NAME
#   Optional: R2_DESTINATION_DIR (prefix/folder inside bucket)
# ============================================================

set -euo pipefail

# --- Load .env if present ---
if [ -f ".env" ]; then
  export $(grep -v '^\s*#' .env | xargs)
fi

# --- Validate required env vars ---
REQUIRED_VARS=(R2_ACCOUNT_ID R2_ACCESS_KEY_ID R2_SECRET_ACCESS_KEY R2_BUCKET_NAME)
for var in "${REQUIRED_VARS[@]}"; do
  if [ -z "${!var:-}" ]; then
    echo "❌ Missing required env var: $var"
    exit 1
  fi
done

# --- Helpers: AWS SigV4 signing ---
hmac_sha256_hex() {
  local key="$1" data="$2"
  printf '%s' "$data" | openssl dgst -sha256 -mac HMAC -macopt "hexkey:$key" | awk '{print $2}'
}

hmac_sha256_hex_from_str() {
  local key="$1" data="$2"
  printf '%s' "$data" | openssl dgst -sha256 -mac HMAC -macopt "key:$key" | awk '{print $2}'
}

sha256_hex() {
  printf '%s' "$1" | openssl dgst -sha256 | awk '{print $2}'
}

sha256_file() {
  openssl dgst -sha256 "$1" | awk '{print $2}'
}

# --- Upload a single file ---
upload_file() {
  local FILE="$1"
  local DEST_KEY="$2"

  local ENDPOINT="https://${R2_ACCOUNT_ID}.r2.cloudflarestorage.com"
  local REGION="auto"
  local SERVICE="s3"
  local METHOD="PUT"
  local CONTENT_TYPE="application/json"

  # Timestamps
  local NOW
  NOW=$(date -u +"%Y%m%dT%H%M%SZ")
  local DATE="${NOW:0:8}"

  # Payload hash
  local PAYLOAD_HASH
  PAYLOAD_HASH=$(sha256_file "$FILE")

  # Canonical headers (must be sorted alphabetically by header name)
  local CANONICAL_HEADERS="content-type:${CONTENT_TYPE}
host:${R2_ACCOUNT_ID}.r2.cloudflarestorage.com
x-amz-content-sha256:${PAYLOAD_HASH}
x-amz-date:${NOW}
"
  local SIGNED_HEADERS="content-type;host;x-amz-content-sha256;x-amz-date"

  # Canonical request
  local CANONICAL_URI="/${R2_BUCKET_NAME}/${DEST_KEY}"
  local CANONICAL_QUERY=""
  local CANONICAL_REQUEST="${METHOD}
${CANONICAL_URI}
${CANONICAL_QUERY}
${CANONICAL_HEADERS}
${SIGNED_HEADERS}
${PAYLOAD_HASH}"

  # String to sign
  local CREDENTIAL_SCOPE="${DATE}/${REGION}/${SERVICE}/aws4_request"
  local HASHED_CANONICAL_REQUEST
  HASHED_CANONICAL_REQUEST=$(sha256_hex "$CANONICAL_REQUEST")
  local STRING_TO_SIGN="AWS4-HMAC-SHA256
${NOW}
${CREDENTIAL_SCOPE}
${HASHED_CANONICAL_REQUEST}"

  # Signing key
  local K_DATE K_REGION K_SERVICE K_SIGNING
  K_DATE=$(hmac_sha256_hex_from_str "AWS4${R2_SECRET_ACCESS_KEY}" "$DATE")
  K_REGION=$(hmac_sha256_hex "$K_DATE" "$REGION")
  K_SERVICE=$(hmac_sha256_hex "$K_REGION" "$SERVICE")
  K_SIGNING=$(hmac_sha256_hex "$K_SERVICE" "aws4_request")

  # Final signature
  local SIGNATURE
  SIGNATURE=$(hmac_sha256_hex "$K_SIGNING" "$STRING_TO_SIGN")

  # Authorization header
  local AUTH_HEADER="AWS4-HMAC-SHA256 Credential=${R2_ACCESS_KEY_ID}/${CREDENTIAL_SCOPE}, SignedHeaders=${SIGNED_HEADERS}, Signature=${SIGNATURE}"

  echo "⬆️  Uploading ${FILE} → ${R2_BUCKET_NAME}/${DEST_KEY}"

  HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -X "$METHOD" \
    -T "$FILE" \
    -H "Content-Type: ${CONTENT_TYPE}" \
    -H "x-amz-date: ${NOW}" \
    -H "x-amz-content-sha256: ${PAYLOAD_HASH}" \
    -H "Authorization: ${AUTH_HEADER}" \
    "${ENDPOINT}/${R2_BUCKET_NAME}/${DEST_KEY}")

  if [ "$HTTP_STATUS" -ge 200 ] && [ "$HTTP_STATUS" -lt 300 ]; then
    echo "✅ Done (HTTP ${HTTP_STATUS}): ${FILE}"
  else
    echo "❌ Failed (HTTP ${HTTP_STATUS}): ${FILE}"
    exit 1
  fi
}

# --- Main ---
DESTINATION_DIR="${R2_DESTINATION_DIR:-}"
FILES=("progress.json" "events.json")

for FILE in "${FILES[@]}"; do
  if [ ! -f "$FILE" ]; then
    echo "❌ File not found: $FILE"
    exit 1
  fi

  if [ -n "$DESTINATION_DIR" ]; then
    DEST_KEY="${DESTINATION_DIR}/${FILE}"
  else
    DEST_KEY="$FILE"
  fi

  upload_file "$FILE" "$DEST_KEY"
done

echo ""
echo "🎉 All files uploaded successfully."
