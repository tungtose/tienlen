# Load env
set dotenv-load

alias rsr := restart_server_remote
alias sl := stream_log
alias dw := deploy_web
alias rs := run_server
alias rc := run_client

bump_patch:
  cargo-release release version patch --manifest-path ./server/Cargo.toml --execute --no-confirm

dockerize:
  sh ./scripts/build-server.sh

update_server:
  sh ./scripts/update-server.sh

run_server:
  cd server && cargo run --release --config env.ENVIRONMENT='"DEV"'

release_dev:
  just bump_patch
  just dockerize 
  sh ./scripts/update-server.sh

restart_server_remote:
  sh ./scripts/restart-server.sh

stream_log:
  sh ./scripts/stream-server-log.sh

serve:
  just ./client/serve


deploy_web:
  export ENVIRONMENT=PROD
  just ./client/gen_wasm
  sh ./scripts/deploy-web.sh

run_client:
  just ./client/run

styles:
  just ./client/styles
