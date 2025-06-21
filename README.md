
<h1 align="center">
  🍺 BeerPM
</h1>

<p align="center">
  <a href="https://github.com/Numbers-Technologies/Beer/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/Numbers-Technologies/Beer/ci.yml?branch=main&label=CI" alt="CI Status"/>
  </a>
  <a href="https://github.com/Numbers-Technologies/Beer/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/Numbers-Technologies/Beer" alt="License"/>
  </a>
  <a href="https://github.com/Numbers-Technologies/Beer/stargazers">
    <img src="https://img.shields.io/github/stars/Numbers-Technologies/Beer?style=social" alt="GitHub stars"/>
  </a>
  <a href="https://github.com/Numbers-Technologies/Beer/issues">
    <img src="https://img.shields.io/github/issues/Numbers-Technologies/Beer" alt="GitHub issues"/>
  </a>
</p>

---

🍻 **BeerPM** — простой и стильный пакетный менеджер для вашего Linux/macOS!
<p align="center">
  <img src="./screenshot.png" alt="BeerPM Terminal Demo" width="600"/>
</p>





---

## 🚀 Быстрый старт

```sh
sudo /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Numbers-Technologies/Beer/refs/heads/main/install.sh)"
```

---

## 📦 Основные команды

| <img src="https://raw.githubusercontent.com/Numbers-Technologies/Beer/main/assets/svg/terminal.svg" width="18"/> Команда | Описание |
|-------------------------------------------|------------------------------------------|
| `beer install <package>`                  | Установить пакет                         |
| `beer uninstall <package>`                | Удалить установленный пакет              |
| `beer update <package>`                   | Обновить пакет                           |
| `beer list`                               | Список установленных пакетов             |
| `beer find <package>`                     | Найти формулу пакета                     |
| `beer info <package>`                     | Информация о пакете                      |
| `beer info`                               | Информация о системе BeerPM              |
| `beer --create-package <dir>`             | Создать шаблон beer_package.toml          |
| `beer help`                               | Показать справку                         |



---

## 🍺 Примеры использования

```sh
beer install cmake
beer install llvm --verbose
beer uninstall cmake
beer update cmake
beer list
beer find python3
beer info cmake
beer info
```

---

## 🛠️ Формула пакета (пример)

```toml
name = "cmake"
git_repository = "https://github.com/Kitware/CMake.git"
dependencies = ["ninja"]

[formula]
install_cmds = [
  "./bootstrap",
  "make",
  "sudo make install"
]
```

---

## 🤝 Контрибьютинг

Pull requests приветствуются! Открывайте issues, предлагайте идеи и улучшения.

---

## ⚡️ Лицензия

BeerPM распространяется под лицензией BSD-3.

---

> 🍻 Enjoy your fresh packages with BeerPM!
