#!/bin/sh -x
# Borrowed from @victorskl's libica syncapi.sh ;)

BASE_URL="https://aps2.platform.illumina.com"
CONVERTER_URL="https://converter.swagger.io/api/convert?url="
SWAGGER_JSON="swagger/v1/swagger.json"

services="wes gds tes ens console"

for service in $services; do
	ep=$CONVERTER_URL$BASE_URL/$service/$SWAGGER_JSON
	wget -q --show-progress --wait=20 --limit-rate=50K -O "$api".json -O $service.json $ep
done
