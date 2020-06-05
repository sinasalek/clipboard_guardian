Clears clipboard text content after x seconds (default is 15sec) to protect sensitive
contents like passwords from praying eyes and keeps an encrypted history of the last
few ones in case the user wanted to recover them.

Features
===
- Clears clipboard texts (ignores, very big texts, files, etc) after 8 seconds and keeps a history
- Ability to recover previously cleared clipboard texts
- UI for settings
- Fully configurable through the UI and tray icon support in Windows

Build Requirements
===
Note that this is developed on Windows 10
- Windows SDK
- CMake
- rustup install stable-x86_64-pc-windows-msvc

Build
===
cargo build --target x86_64-pc-windows-msvc

Todo
===
- Clipboard history and shortcut for recovering the last clipboard content
- Limiting the max text size. clipboard texts bigger than that won't be cleared and saved
- Encrypting clipboard history and pin code for accessing history
- Use OS built-in authentication system for encrypting and decrypting history
- Full Linux support
- GUI
- Cross-compile using https://github.com/japaric/trust
- When the clipboard content looks like a password or sensitive info. quickly clear it and put it in secure
  storage and ask user asks for confirmation whenever he tried to paste it
- Write automated tests