setx BACKUP_DIR "$pwd\src\fixtures"
cargo build --release
xcopy /s .\target\release\syncr32.exe .\builds\
