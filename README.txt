Clears clipboard text content after x second (default is 15sec) to protect sensitive
contents like password from praying eyes

Requirements
===
Note that this is developed on Windows 10
- Windows SDK
- rustup install stable-x86_64-pc-windows-msvc

Build
===
cargo build --target x86_64-pc-windows-msvc

Todo
===
- Clipboard history and shortcut for recovering the last clipboard content
- Encrypting clipboard history and pin code for accessing history
- Use OS built-in authentication system for encrypting and decrypting history
- Full Linux support
- GUI
- Cross-compile using https://github.com/japaric/trust
- Write automated tests