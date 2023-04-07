#!/bin/bash

trunk build --release 
rm -rf docs
mv dist docs
sudo git add .
sudo git commit -m "build push"
sudo git push origin main
