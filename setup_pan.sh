#!/bin/bash

# Create and configure pan0 bridge
sudo ip link add name pan0 type bridge 2>/dev/null || echo "Bridge pan0 already exists"
sudo ip link set pan0 up
sudo ip addr add 192.168.7.1/24 dev pan0 2>/dev/null || echo "IP already assigned to pan0"
