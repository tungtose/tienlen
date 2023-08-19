REMOTE="ubuntu@tienlen.cedrus.cloud"
DEST="/home/ubuntu/workspaces/"

ssh $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
  rm -rf $DEST/*
ENDSSH

scp -r .env client/assets client/index.html client/wasm "$REMOTE:$DEST"

ssh $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
  sudo cp -r $DEST* /var/www/html
  sudo systemctl start nginx
ENDSSH
