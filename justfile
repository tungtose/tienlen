alias rs := restart_server
alias sl := stream_log
alias dw := deploy_web


bump_patch:
  cargo-release release version patch --manifest-path ./server/Cargo.toml --execute --no-confirm

dockerize:
  sh ./scripts/build-server.sh

update_server:
  sh ./scripts/update-server.sh


release_dev:
  just bump_path
  just dockerize 
  sh update_server

restart_server:
  sh ./scripts/restart-server.sh

stream_log:
  sh ./scripts/stream-server-log.sh

serve:
  just ./client/serve


deploy_web:
  just ./client/wasm_bindgen
  sh ./scripts/deploy-web.sh
