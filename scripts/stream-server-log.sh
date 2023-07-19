REMOTE="ubuntu@tienlen.cedrus.cloud"

ssh -i ~/projects/keys/tung-nix.pem $REMOTE 'bash -s' <<'ENDSSH'
  docker logs -f tienlen_server
ENDSSH
