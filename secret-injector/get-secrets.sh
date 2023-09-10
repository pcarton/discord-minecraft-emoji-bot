#!/bin/bash

DISCORD_API_TOKEN=`gcloud secrets versions access $DISCORD_API_TOKEN_VERSION --secret=cartonbot-discord-token`
DISCORD_GUILD_ID=`gcloud secrets versions access $DISCORD_GUILD_ID_VERSION --secret=cartonbot-discord-guild-id`

cat > /secrets/secrets.yaml <<EOF
secrets:
  discord_api_token: "$DISCORD_API_TOKEN"
  discord_guild_id: "$DISCORD_GUILD_ID"
EOF