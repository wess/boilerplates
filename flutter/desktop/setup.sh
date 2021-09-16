#!/usr/bin/env bash

APP=${PWD##*/}
TAG="{{APP}}"

echo "Creating flutter desktop app"
IFS=$'\n'; set -f
for f in $(find . -name '*.dart' -or -name '*.yaml'); do 
  sed -i "" "s/${TAG}/${APP}/g" $f
done

unset IFS; set +f

flutter create --platforms=windows,macos,linux .

echo "Don't forget to run 'skelup' and 'direnv allow'"
echo "All done."