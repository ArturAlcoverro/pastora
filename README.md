# Pastora

Fork personal de [Herdr](https://github.com/herdrdev/herdr-app), la app de escritorio para terminales y agentes. Se instala junto a Herdr sin tocarlo: tiene su propia app, su propio comando y su propia carpeta de configuración.

## Versión de Herdr

Basado en **herdr-app 0.1.18**, en el commit [`a2e8e3f`](https://github.com/herdrdev/herdr-app/commit/a2e8e3ffa84439d4a245a1e6aa8947ef0dd20a1b): la 0.1.18 más «fix: let agent tabs follow the running program (#1447)».

El primer commit de este repositorio es ese código sin cambios. Cada cambio de Pastora va después, en su propio commit.

README de Herdr de esa versión: [aquí](README.herdr.md) o [en el repositorio oficial](https://github.com/herdrdev/herdr-app/blob/a2e8e3ffa84439d4a245a1e6aa8947ef0dd20a1b/README.md).

## Qué añade

- **Nombre, icono y carpeta propios.** Al compilar, `HERDR_APP_NAME` cambia «Herdr» en todo lo que se ve (ventana, menús, avisos, ajustes, bienvenida), `HERDR_APP_ICON` elige el icono y `HERDR_APP_DIR_NAME` la carpeta de configuración. Los mods leen el nombre con `appName()`. Sin estas variables se compila Herdr tal cual.
- **Estilos de Markdown en `TextView`.** Los mods pueden cambiar el color del código en línea y de los enlaces (`inline_code_color`, `inline_code_background`, `link_color`) y dar estilo a cada parte del Markdown con `md_style(part, s => …)`: títulos, bloques de código, tablas, citas, listas, separadores, negritas y cursivas. Documentado en [`defaults/AGENTS.md`](defaults/AGENTS.md).
- **Recarga en caliente con enlaces simbólicos.** Si la carpeta de configuración enlaza a otra (como `~/.config/pastora` a `~/.config/herdr-app`), los cambios en los ficheros enlazados se aplican al momento, en lugar de esperar al sondeo de cada 10 segundos.

## Instalar

En macOS, con Rust 1.95 y Zig 0.16:

```sh
git clone --recurse-submodules git@github.com:ArturAlcoverro/pastora.git
cd pastora
scripts/install-pastora
```

Instala `~/Applications/Pastora.app` y el comando `pastora` en `~/.local/bin`. La configuración vive en `~/.config/pastora`. Usa el mismo herdr-core que Herdr.

## Traer una versión nueva de Herdr

Este repositorio no comparte historial con el oficial. La rama `herdr` tiene un commit por cada versión de Herdr importada, y `main` lleva los cambios de Pastora encima. Para pasar a otra versión, con `origin` apuntando al repositorio oficial:

```sh
git fetch origin --tags
new=$(git commit-tree "v0.1.19^{tree}" -p herdr -m "chore: import herdr-app 0.1.19")
git branch -f herdr "$new"
git switch main && git rebase herdr
```

Después, resuelve los conflictos si los hay, actualiza la versión de este README, comprueba que compila y sube las dos ramas (`main` con `--force-with-lease`).

## Licencia

Herdr es de sus autores, con licencia [Apache 2.0](LICENSE). Los cambios de Pastora se publican bajo la misma licencia.
