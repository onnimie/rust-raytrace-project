# ANSI Escape Sequences for Terminal Control

## Cursor Movement

| Sequence | Description |
|----------|-------------|
| `\x1B[<n>A` | Move cursor up n lines |
| `\x1B[<n>B` | Move cursor down n lines |
| `\x1B[<n>C` | Move cursor right n columns |
| `\x1B[<n>D` | Move cursor left n columns |
| `\x1B[<n>E` | Move cursor to beginning of line, n lines down |
| `\x1B[<n>F` | Move cursor to beginning of line, n lines up |
| `\x1B[<n>G` | Move cursor to column n |
| `\x1B[<row>;<col>H` | Move cursor to row, col (e.g., `\x1B[5;10H`) |
| `\x1B[H` | Move cursor to home (0,0) |

## Clearing

| Sequence | Description |
|----------|-------------|
| `\x1B[K` | Clear from cursor to end of line |
| `\x1B[J` | Clear from cursor to end of screen |
| `\x1B[2J` | Clear entire screen |
| `\x1B[0K` | Clear from start of line to cursor |
| `\x1B[0J` | Clear from top of screen to cursor |

## Text Formatting

| Sequence | Description |
|----------|-------------|
| `\x1B[0m` | Reset all attributes |
| `\x1B[1m` | Bold |
| `\x1B[2m` | Dim/Faint |
| `\x1B[3m` | Italic |
| `\x1B[4m` | Underline |
| `\x1B[5m` | Blink |
| `\x1B[7m` | Reverse video (invert colors) |
| `\x1B[9m` | Strikethrough |

## Foreground Colors (30-37)

| Sequence | Description |
|----------|-------------|
| `\x1B[30m` | Black |
| `\x1B[31m` | Red |
| `\x1B[32m` | Green |
| `\x1B[33m` | Yellow |
| `\x1B[34m` | Blue |
| `\x1B[35m` | Magenta |
| `\x1B[36m` | Cyan |
| `\x1B[37m` | White |
| `\x1B[39m` | Default |

## Bright Foreground Colors (90-97)

| Sequence | Description |
|----------|-------------|
| `\x1B[90m` | Bright Black (Gray) |
| `\x1B[91m` | Bright Red |
| `\x1B[92m` | Bright Green |
| `\x1B[93m` | Bright Yellow |
| `\x1B[94m` | Bright Blue |
| `\x1B[95m` | Bright Magenta |
| `\x1B[96m` | Bright Cyan |
| `\x1B[97m` | Bright White |

## Background Colors (40-47)

| Sequence | Description |
|----------|-------------|
| `\x1B[40m` | Black background |
| `\x1B[41m` | Red background |
| `\x1B[42m` | Green background |
| `\x1B[43m` | Yellow background |
| `\x1B[44m` | Blue background |
| `\x1B[45m` | Magenta background |
| `\x1B[46m` | Cyan background |
| `\x1B[47m` | White background |
| `\x1B[49m` | Default background |

## Bright Background Colors (100-107)

| Sequence | Description |
|----------|-------------|
| `\x1B[100m` | Bright Black background |
| `\x1B[101m` | Bright Red background |
| `\x1B[102m` | Bright Green background |
| `\x1B[103m` | Bright Yellow background |
| `\x1B[104m` | Bright Blue background |
| `\x1B[105m` | Bright Magenta background |
| `\x1B[106m` | Bright Cyan background |
| `\x1B[107m` | Bright White background |

## Cursor Visibility

| Sequence | Description |
|----------|-------------|
| `\x1B[?25h` | Show cursor |
| `\x1B[?25l` | Hide cursor |

## Screen Modes

| Sequence | Description |
|----------|-------------|
| `\x1B[?1049h` | Enable alternative screen buffer |
| `\x1B[?1049l` | Disable alternative screen buffer (return to normal) |

## Common Combinations

```rust
// Red text
stdout.write_all(b"\x1B[31mError!\x1B[0m\n")?;

// Bold green text
stdout.write_all(b"\x1B[1;32mSuccess!\x1B[0m\n")?;

// Yellow on blue background
stdout.write_all(b"\x1B[33;44mWarning\x1B[0m\n")?;

// Move up 3 lines and clear
stdout.write_all(b"\x1B[3A\x1B[J")?;
```

**Note:** Multiple attributes can be combined with semicolons (e.g., `\x1B[1;31m` = bold red).Screen Modes

\x1B[?1049h - Enable alternative screen buffer
\x1B[?1049l - Disable alternative screen buffer (return to normal)