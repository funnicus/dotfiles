1. Obsidian integration
2. Don't change existing ssh config if not logged to bitwarden
3. Add fedora support
4. Add ability to skip package install
  1. (Default) full install
  2. Minimal install (only dotfile dependencies)
  3. No install
5. Uninstall packages and reset/remove dotfiles?
6. Automatic periodic checks to pipeline
7. gdu-go to mac only
8. Refactor install script (break into different files?)
  1. Would it be more maintanable just to have a different script file for each os type? 
9. Make sure all the necessary package managers are installed (brew, cargo etc.)
  1. Handle cases where you have multiple package managers installed
10. Check if repo contains unnecessary things (random python scripts etc)
11. Add ngrok to install https://ngrok.com/docs/guides/share-localhost/overview
12. Easily add more secrets at will (without chezmoi init/writing only new values)
13. Initial install re triggers on macos every apply
