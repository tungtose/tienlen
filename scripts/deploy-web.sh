#!/bin/sh

# REMOTE="ubuntu@tienlen-api.cedrus.cloud"
# DEST="/home/ubuntu/workspaces/"
#
# ssh $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
#   rm -rf $DEST/*
# ENDSSH
#
# scp -r .env client/assets client/index.html client/wasm client/tools.js client/app.js "$REMOTE:$DEST"
#
# ssh $REMOTE DEST=$DEST 'bash -s' <<'ENDSSH'
#   sudo cp -r $DEST* /var/www/html
#   sudo systemctl start nginx
# ENDSSH

export AWS_DEFAULT_PROFILE=tung
BUCKET_NAME="tienlen-cedrus"
TEMP_DIR="web"

cd $PWD

echo "Copying files..."

mkdir -p $TEMP_DIR

cp client/index.html $TEMP_DIR
cp client/assets/ $TEMP_DIR -r
cp client/wasm $TEMP_DIR -r

echo "Sync files..."

aws s3 sync $TEMP_DIR "s3://$BUCKET_NAME"

rm -rf $TEMP_DIR

echo "--- Invalidate CDN cache ---"

aws cloudfront create-invalidation --distribution-id "ETHH491PYHL4N" --path "/*"

echo "--- Done ---"
