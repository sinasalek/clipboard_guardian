Clears clipboard on specified intervals for protection against spywares

Requirements
===
Note that this is developed on Windows 10
- Windows SDK
- rustup install stable-x86_64-pc-windows-msvc

Build
===
cargo build --target x86_64-pc-windows-msvc

Roadmap
===
- Ignore anything other than text
- Clipboard history and shortcut for recovering the last clipboard content
- Encrypting clipboard history and pin code for accessing history
- Use OS built-in authentication system for encrypting and decrypting history
- Full Linux support
- Cross-compile using https://github.com/japaric/trust
- GUI