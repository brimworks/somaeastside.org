IAM_ROLE=arn:aws:iam::303389128770:role/service-role/pco-proxy-role-7vw71y3m

.PHONY: build
build:
	cargo lambda build --release --arm64


.PHONY: deploy
deploy: build
	cargo lambda deploy pco-proxy

.PHONY: run
run:
	cargo run -- --local

.PHONY: put-pco-secrets
put-pco-secrets:
	aws ssm put-parameter --name "/prod/pco/pat/app_id" --type "SecureString" --value "$(PCO_PAT_APP_ID)"
	aws ssm put-parameter --name "/prod/pco/pat/secret" --type "SecureString" --value "$(PCO_PAT_SECRET)"