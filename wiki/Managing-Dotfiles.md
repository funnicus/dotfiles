# Managing Dotfiles

## Capture all local changes

```bash
chezmoi re-add

chezmoi cd
git status
git add -A
git commit -m "Update dotfiles"
git push
```

## Edit source files directly

```bash
chezmoi cd
git diff
git add .
git commit -m "chore: update dotfiles"
git push
```

## Add or refresh a managed file

```bash
chezmoi add ~/.config/fish/config.fish
chezmoi cd
git diff
git commit -am "chore: update fish config"
git push
```

## Edit through chezmoi

```bash
chezmoi edit ~/.config/fish/config.fish
chezmoi diff
chezmoi apply
```

## Edit prompts

```bash
chezmoi edit-config
chezmoi diff path/to/file
chezmoi apply path/to/file
```

## Preview apply

```bash
chezmoi diff
chezmoi apply --dry-run --verbose
chezmoi apply -n -v
```
