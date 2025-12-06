# Filmspec

Outil CLI pour générer des images de spectre de film à partir de fichiers vidéo.

#### Fonctionnement

Filmspec échantillonne uniformément les images d'un fichier vidéo dans le temps, traite chaque image selon le mode sélectionné et génère un spectre visuel unique.

**Mode Slice** - Extrait des lignes de pixels de chaque image :

![exemple slice](images/example_slice.png)

**Mode Hue** - Analyse la couleur dominante de chaque image :

![exemple hue](images/example_hue.png)

#### Modes de Traitement

| Mode | Description |
|------|-------------|
| slice | Extrait une ligne de pixels (ligne ou colonne) de chaque image et les empile |
| hue | Analyse la couleur dominante de chaque image et crée un spectre de bandes de couleur |

#### Modes d'Échantillonnage et de Disposition

| Mode d'Échantillonnage | Description |
|------------------------|-------------|
| row | Extraire la ligne horizontale centrale de chaque image (mode slice uniquement) |
| col | Extraire la ligne verticale centrale de chaque image (mode slice uniquement) |

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
| `--output` | `-o` | Chemin de l'image de sortie | `<nom>_spectrum.png` ou `<nom>_hue.png` |
| `--width` | `-w` | Largeur de l'image | 1920 |
| `--height` | `-H` | Hauteur de l'image | 300 |
| `--mode` | `-m` | Mode de traitement: slice/hue | slice |
| `--layout` | `-l` | Direction de disposition: h/v | h |
| `--sample` | `-s` | Direction d'échantillonnage: row/col (mode slice uniquement) | row |

#### Exemples

```bash
# Mode slice par défaut (1920×300, disposition horizontale, échantillonnage par ligne)
filmspec movie.mp4

# Mode spectre de teinte (analyse de couleur dominante)
filmspec movie.mp4 -m hue

# Chemin et taille personnalisés
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Disposition verticale (empiler de haut en bas)
filmspec movie.mp4 -l v

# Échantillonnage par colonne en mode slice (extraire les lignes de pixels verticales)
filmspec movie.mp4 -s col

# Mode hue + disposition verticale
filmspec movie.mp4 -m hue -l v -w 400 -H 1920
```

#### Licence

MIT
