KEY="~/projects/keys/tung-nix.pem"
REMOTE="ubuntu@tienlen.cedrus.cloud"
DEST="/home/ubuntu/workspaces/"

ssh -i $KEY $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
  rm -rf $DEST/*
ENDSSH

scp -i $KEY -r .env client/assets client/index.html client/wasm "$REMOTE:$DEST"

ssh -i $KEY $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
  sudo cp -r $DEST* /var/www/html
  sudo systemctl start nginx
ENDSSH
