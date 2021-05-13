#!/bin/bash
NUMBER="3.1.0"
SZYSZKA_PATH="/home/rafal"

cd "$SZYSZKA_PATH"
SZYSZKA_PATH="$SZYSZKA_PATH/szyszka"
rm -rf $SZYSZKA_PATH
git clone https://github.com/qarmin/szyszka.git "$SZYSZKA_PATH"
cd $SZYSZKA_PATH
git checkout "$NUMBER"

cd "$SZYSZKA_PATH"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed"
  exit 1
fi
git reset --hard

cd "$SZYSZKA_PATH"
cargo publish
git reset --hard

