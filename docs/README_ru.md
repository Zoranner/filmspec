# Filmspec

CLI-инструмент для генерации спектральных изображений фильмов из видеофайлов.

#### Принцип Работы

Filmspec равномерно сэмплирует кадры из видеофайла по времени, обрабатывает каждый кадр в соответствии с выбранным режимом и создаёт уникальный визуальный спектр.

**Режим Slice** - Извлекает линии пикселей из каждого кадра:

![пример slice](images/example_slice.png)

**Режим Hue** - Анализирует доминирующий цвет каждого кадра:

![пример hue](images/example_hue.png)

#### Галерея Примеров

Коллекция примеров радиального спектра из музыкальных видеоклипов:

<table>
<tr>
<td align="center"><img src="images/examples/Monica_spectrum_slice.png" width="200" alt="Monica"><br/>Monica</td>
<td align="center"><img src="images/examples/不要爱他_spectrum_slice.png" width="200" alt="不要爱他"><br/>不要爱他</td>
<td align="center"><img src="images/examples/为你钟情_spectrum_slice.png" width="200" alt="为你钟情"><br/>为你钟情</td>
</tr>
<tr>
<td align="center"><img src="images/examples/共同渡过_spectrum_slice.png" width="200" alt="共同渡过"><br/>共同渡过</td>
<td align="center"><img src="images/examples/大热_spectrum_slice.png" width="200" alt="大热"><br/>大热</td>
<td align="center"><img src="images/examples/我_spectrum_slice.png" width="200" alt="我"><br/>我</td>
</tr>
<tr>
<td align="center"><img src="images/examples/春夏秋冬_spectrum_slice.png" width="200" alt="春夏秋冬"><br/>春夏秋冬</td>
<td align="center"><img src="images/examples/热情的沙漠_spectrum_slice.png" width="200" alt="热情的沙漠"><br/>热情的沙漠</td>
<td align="center"><img src="images/examples/至少还有你_spectrum_slice.png" width="200" alt="至少还有你"><br/>至少还有你</td>
</tr>
</table>

#### Режимы Обработки

| Режим | Описание |
|-------|----------|
| slice | Извлекает линию пикселей (строку или столбец) из каждого кадра и складывает их |
| hue | Анализирует доминирующий цвет каждого кадра и создаёт спектр цветовых полос |

#### Режимы Сэмплирования и Компоновки

| Режим сэмплирования | Описание |
|---------------------|----------|
| row | Извлечь среднюю горизонтальную линию из каждого кадра (только режим slice) |
| col | Извлечь среднюю вертикальную линию из каждого кадра (только режим slice) |

| Режим компоновки | Описание |
|------------------|----------|
| h | Горизонтальная укладка (слева направо) |
| v | Вертикальная укладка (сверху вниз) |
| r | Радиальная компоновка (в форме диска, как CD) |

#### Требования

- Rust 1.70+
- FFmpeg (должен быть доступен в системном PATH)

#### Установка

```bash
cargo install --path .
```

#### Использование

```bash
filmspec <INPUT> [OPTIONS]
```

#### Параметры

| Параметр | Сокр. | Описание | По умолчанию |
|----------|-------|----------|--------------|
| `<INPUT>` | - | Путь к входному видеофайлу | обязательно |
| `--output` | `-o` | Путь к выходному изображению | `<имя>_spectrum_slice.png` или `<имя>_spectrum_hue.png` |
| `--width` | `-w` | Количество кадров (h/v: ширина изображения, r: кадры по окружности) | 1920 |
| `--height` | `-H` | Длина полосы (h/v: высота изображения, r: ширина кольца) | 300 (h/v), 500 (r) |
| `--mode` | `-m` | Режим обработки: slice/hue | slice |
| `--layout` | `-l` | Направление компоновки: h/v/r | h |
| `--sample` | `-s` | Направление сэмплирования: row/col (только режим slice) | row |
| `--inner-radius` | - | Внутренний радиус для радиальной компоновки (пиксели) | 250 |

#### Примеры

```bash
# Режим slice по умолчанию (1920×300, горизонтальная компоновка, сэмплирование по строкам)
filmspec movie.mp4

# Режим спектра оттенков (анализ доминирующего цвета)
filmspec movie.mp4 -m hue

# Пользовательский путь и размер
filmspec movie.mp4 -o output.png -w 1280 -H 400

# Вертикальная компоновка (укладка сверху вниз)
filmspec movie.mp4 -l v

# Сэмплирование по столбцам в режиме slice (извлечение вертикальных линий пикселей)
filmspec movie.mp4 -s col

# Режим hue + вертикальная компоновка
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# Радиальная компоновка (спектр в форме диска)
filmspec movie.mp4 -l r

# Радиальная компоновка + пользовательские параметры
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# Режим hue + радиальная компоновка
filmspec movie.mp4 -m hue -l r
```

#### Лицензия

MIT
