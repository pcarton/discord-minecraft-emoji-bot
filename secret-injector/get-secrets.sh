#!/bin/bash 
mkdir /secrets
cat > /secrets/secrets.yaml <<EOF
secrets:
  discord_api_token: "`gcloud secrets versions access $DISCORD_API_TOKEN_VERSION --secret=cartonbot-discord-token`"
  discord_guild_id: "`gcloud secrets versions access $DISCORD_GUILD_ID_VERSION --secret=cartonbot-discord-guild-id`"
EOF