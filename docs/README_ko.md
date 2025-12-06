# Filmspec

비디오 파일에서 필름 스펙트럼 이미지를 생성하는 CLI 도구.

#### 작동 원리

Filmspec은 비디오에서 균등하게 프레임을 샘플링하고, 선택한 모드에 따라 각 프레임을 처리하여 독특한 시각적 스펙트럼을 생성합니다.

**Slice 모드** - 각 프레임에서 픽셀 라인 추출:

![slice 예시](images/example_slice.png)

**Hue 모드** - 각 프레임의 주요 색조 분석:

![hue 예시](images/example_hue.png)

#### 처리 모드

| 모드 | 설명 |
|------|------|
| slice | 각 프레임에서 하나의 픽셀 라인(행 또는 열)을 추출하여 쌓기 |
| hue | 각 프레임의 주요 색조를 분석하여 컬러 밴드 스펙트럼 생성 |

#### 샘플링 및 레이아웃 모드

| 샘플 모드 | 설명 |
|----------|------|
| row | 각 프레임의 중앙 수평 라인 추출 (slice 모드만) |
| col | 각 프레임의 중앙 수직 라인 추출 (slice 모드만) |

| 레이아웃 모드 | 설명 |
|--------------|------|
| h | 수평 스택 (왼쪽에서 오른쪽) |
| v | 수직 스택 (위에서 아래) |
| r | 방사형 레이아웃 (CD처럼 원반 모양) |

#### 사전 요구사항

- Rust 1.70+
- FFmpeg (시스템 PATH에서 사용 가능해야 함)

#### 설치

```bash
cargo install --path .
```

#### 사용법

```bash
filmspec <INPUT> [OPTIONS]
```

#### 옵션

| 옵션 | 단축 | 설명 | 기본값 |
|------|------|------|--------|
| `<INPUT>` | - | 입력 비디오 파일 경로 | 필수 |
| `--output` | `-o` | 출력 이미지 경로 | `<파일명>_spectrum.png` 또는 `<파일명>_hue.png` |
| `--width` | `-w` | 프레임 수 (h/v: 이미지 너비, r: 원주 프레임 수) | 1920 |
| `--height` | `-H` | 밴드 길이 (h/v: 이미지 높이, r: 링 너비) | 300 (h/v), 800 (r) |
| `--mode` | `-m` | 처리 모드: slice/hue | slice |
| `--layout` | `-l` | 레이아웃 방향: h/v/r | h |
| `--sample` | `-s` | 샘플 방향: row/col (slice 모드만) | row |
| `--inner-radius` | - | 방사형 레이아웃 내부 반경 (픽셀) | 250 |

#### 예시

```bash
# 기본 slice 모드 (1920×300, 수평 레이아웃, 행 샘플)
filmspec movie.mp4

# 색조 스펙트럼 모드 (주요 색조 분석)
filmspec movie.mp4 -m hue

# 출력 경로 및 크기 지정
filmspec movie.mp4 -o output.png -w 1280 -H 400

# 수직 레이아웃 (위에서 아래로 스택)
filmspec movie.mp4 -l v

# slice 모드에서 열 샘플 (수직 픽셀 라인 추출)
filmspec movie.mp4 -s col

# 색조 모드 + 수직 레이아웃
filmspec movie.mp4 -m hue -l v -w 400 -H 1920

# 방사형 레이아웃 (원반 모양 스펙트럼)
filmspec movie.mp4 -l r

# 방사형 레이아웃 + 사용자 정의 매개변수
filmspec movie.mp4 -l r -w 2400 -H 600 --inner-radius 200

# 색조 모드 + 방사형 레이아웃
filmspec movie.mp4 -m hue -l r
```

#### 라이선스

MIT
