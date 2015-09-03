rm /usr/local/bin/chronia
rm -rf /usr/local/lib/rustlib/chronia
systemctl disable chronia.service
rm /etc/systemd/system/chronia.service
