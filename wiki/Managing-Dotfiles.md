# Managing Dotfiles

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

## Preview apply

```bash
chezmoi diff
chezmoi apply --dry-run --verbose
chezmoi apply -n -v
```
