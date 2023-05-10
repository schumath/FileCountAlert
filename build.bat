@echo off
cargo build --release
xcopy config.yml target\release\ /Y /i