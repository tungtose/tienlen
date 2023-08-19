REMOTE="ubuntu@tienlen.cedrus.cloud"

ssh $REMOTE 'bash -s' <<'ENDSSH'
  docker logs -f tienlen_server
ENDSSH
