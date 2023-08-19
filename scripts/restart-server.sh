REMOTE="ubuntu@tienlen.cedrus.cloud"

ssh $REMOTE 'bash -s' <<'ENDSSH'
  docker restart tienlen_server
  docker ps
ENDSSH
