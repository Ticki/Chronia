cargo build --release
cp ./target/release/chronia /usr/local/bin
cp -rf ./target/release/deps /usr/local/lib/rustlib/chronia
cp ./chronia.service /etc/systemd/system/
systemctl enable chronia.service
