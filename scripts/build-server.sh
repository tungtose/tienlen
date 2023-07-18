#!/bin/sh

ECR_HOST=635506958747.dkr.ecr.ap-southeast-1.amazonaws.com

aws ecr get-login-password --region ap-southeast-1 | docker login --username AWS --password-stdin $ECR_HOST

cd $PWD/server
VERSION=$(cargo metadata --format-version 1  | jq -r '.packages[]  | select(.name | test("tienlen-server")) | .version')

cd ../

docker build -t 635506958747.dkr.ecr.ap-southeast-1.amazonaws.com/tienlen:$VERSION $PWD
docker push 635506958747.dkr.ecr.ap-southeast-1.amazonaws.com/tienlen:$VERSION
