#!/bin/bash

USER=$(whoami)
HOME_DIR="/home/$USER"
SERVICE_DIR="/etc/systemd/system"

# Substitute $USER into service file and copy to systemd
sed "s/\$USER/$USER/g" update_raid.service | sudo tee "$SERVICE_DIR/update_raid.service" > /dev/null

# Copy timers
sudo cp update_raid-daily.timer "$SERVICE_DIR/update_raid-daily.timer"
sudo cp update_raid-raidnight.timer "$SERVICE_DIR/update_raid-raidnight.timer"

# Reload and enable
sudo systemctl daemon-reload
sudo systemctl enable --now update_raid-daily.timer
sudo systemctl enable --now update_raid-raidnight.timer

echo "Done! Active timers:"
systemctl list-timers | grep update_raid
