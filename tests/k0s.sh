#!/bin/bash

# install K0s

curl -sSLf https://get.k0s.sh | sudo sh
sudo k0s install controller --single
sudo k0s start
sudo k0s status
sudo k0s kubectl get nodes


# run test


# stop and cleanup

sudo k0s stop
sudo k0s reset