cargo build --release
cp -f ./target/release/chronia /usr/local/bin
cp -rf ./target/release/deps /usr/local/lib/rustlib/chronia
cp -r ./chronia.service /etc/systemd/system/
systemctl enable chronia.service
