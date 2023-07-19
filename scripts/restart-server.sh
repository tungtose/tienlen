REMOTE="ubuntu@tienlen.cedrus.cloud"

ssh -i ~/projects/keys/tung-nix.pem $REMOTE 'bash -s' <<'ENDSSH'
  docker restart tienlen_server
  docker ps
ENDSSH
