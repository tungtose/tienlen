REMOTE="ubuntu@tienlen-api.cedrus.cloud"
ECR_HOST=635506958747.dkr.ecr.ap-southeast-1.amazonaws.com
DEST="/home/ubuntu/workspaces/"

cd $PWD/server
VERSION=$(cargo metadata --format-version 1  | jq -r '.packages[]  | select(.name | test("tienlen-server")) | .version')

echo $VERSION

echo "================ REMOTE ========================"


cd ../

scp .env "$REMOTE:$DEST"

ssh $REMOTE ECR_HOST=$ECR_HOST VERSION=$VERSION DEST=$DEST 'bash -s' <<'ENDSSH'
  echo "Update version: $VERSION"
  aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin $ECR_HOST
  docker pull 635506958747.dkr.ecr.ap-southeast-1.amazonaws.com/tienlen:$VERSION
  docker rm -f $(docker ps -aq)
  export ENVIRONMENT="PROD"
  docker run --name tienlen_server -e ENVIRONMENT=PROD --env-file=$DEST/.env -p 14191:14191 -p 14192:14192 --network host -d 635506958747.dkr.ecr.ap-southeast-1.amazonaws.com/tienlen:$VERSION
  docker image prune
ENDSSH
