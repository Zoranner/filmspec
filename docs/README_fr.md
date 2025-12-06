# Filmspec

Outil CLI pour générer des images de spectre de film à partir de fichiers vidéo.

#### Fonctionnement

Filmspec échantillonne uniformément les images d'un fichier vidéo dans le temps, extrait une ligne de pixels (ligne ou colonne) de chaque image, puis les empile pour créer un spectre visuel unique.

![example](images/example.png)

#### Modes d'Échantillonnage et de Disposition

| Mode de Tranche | Description |
|-----------------|-------------|
| row | Extraire la ligne horizontale centrale de chaque image |
| col | Extraire la ligne verticale centrale de chaque image |

| Mode de Disposition | Description |
|---------------------|-------------|
| h | Empiler horizontalement (gauche à droite) |
| v | Empiler verticalement (haut en bas) |

#### Prérequis

- Rust 1.70+
- FFmpeg (doit être disponible dans le PATH système)

#### Installation

```bash
cargo install --path .
```

#### Utilisation

```bash
filmspec <INPUT> [OPTIONS]
```

#### Options

| Option | Court | Description | Défaut |
|--------|-------|-------------|--------|
| `<INPUT>` | - | Chemin du fichier vidéo | requis |
| `--output` | `-o` | Chemin de l'image de sortie | `<nom>_spectrum.png` |
| `--width` | `-w` | Largeur de l'image | 1920 |
| `--height` | `-H` | Hauteur de l'image | 300 |
| `--layout` | `-l` | Direction de disposition h/v | h |
| `--slice` | `-s` | Direction de tranche row/col | row |

#### Exemples

```bash
# Défaut (1920×300, disposition horizontale, tranche par ligne)
filmspec movie.mp4

# Chemin et taille personnalisés
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Disposition verticale (empiler de haut en bas)
filmspec movie.mp4 -l v

# Tranche par colonne (extraire les lignes de pixels verticales)
filmspec movie.mp4 -s col
```

#### Licence

MIT

