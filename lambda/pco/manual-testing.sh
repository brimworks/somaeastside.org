#!/bin/bash
#
# Assumes these two environment variables are set:
#   PCO_CLIENT_ID
#   PCO_SECRET
set -euo pipefail
#
# pco-proxy:
#    Encrypts the current refresh token
#    
SCOPES='calendar+check_ins+giving+groups+people+services'
REDIRECT_URI='http://localhost:3000'
#open "https://api.planningcenteronline.com/oauth/authorize?client_id=${PCO_CLIENT_ID}&redirect_uri=$REDIRECT_URI&response_type=code&scope=$SCOPES&state=EXTRA"

# After accepting, the browser should redirect back to something like this:
#    http://localhost:3000/?code=b86bd791e98bac33d35dffd1d30573122b1c0ae785798b98f2b0689ba6f44a08
CODE=b86bd791e98bac33d35dffd1d30573122b1c0ae785798b98f2b0689ba6f44a08

curl -v -X POST https://api.planningcenteronline.com/oauth/token \
              -F grant_type=authorization_code \
              -F code=$CODE \
              -F client_id=$PCO_CLIENT_ID \
              -F client_secret=$PCO_SECRET \
              -F redirect_uri=$REDIRECT_URI

# Example response
# ~ 2 hrs?
# {"access_token":"a7d0d295a3ca01c2936ab0592fbda9b5be84d0a85154fbdf34bead2e7cde22bd","token_type":"Bearer","expires_in":7199,"refresh_token":"aa42dda39fc08374cf57ca7b0036f4a1c1a7d3feb9665c2cece6aef167b0e116","scope":"calendar check_ins giving groups people services","created_at":1705336667}


# PCO_PAT_APP_ID=
# PCO_PAT_SECRET=