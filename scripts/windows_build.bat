setx BACKUP_ROOT "$pwd\src\fixtures"
cargo build --release
xcopy /s .\target\release\syncr32.exe .\builds\
