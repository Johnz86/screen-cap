# Screen-Cap

A command-line tool for capturing screenshots of specific windows.

## Description

Screen-Cap allows you to capture screenshots of a target window at specified intervals and save them in WEBP format. It offers command-line options to customize the target window, output directory, and capture interval.  If the target window closes, Screen-Cap will exit gracefully.

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Johnz86/screen-cap
cd screen-cap
```

**Build the project:**

```bash
cargo build --release
```

## Usage

```bash
./target/release/screen-cap.exe [OPTIONS]
```

## Options


| Short | Long        | Description                               | Default Value |
| ----- | ----------- | ----------------------------------------- | ------------- |
| `-w`  | `--window-title` | Target window title (case-sensitive, contains) | "Chrome"      |
| `-o`  | `--output-dir`  | Output directory for screenshots          | "screenshots" |
| `-i`  | `--interval`   | Capture interval in seconds               | 1             |
| `-h`  | `--help`      | Prints help information                   |               |
| `-V`  | `--version`   | Prints version information                |               |


## Examples

Capture screenshots of a window containing 'Notepad' every 2 seconds and save them to the 'my_screenshots' directory:

```bash
./target/release/screen-cap.exe -w Notepad -o my_screenshots -i 2
```

Capture screenshots of a window containing "Firefox" with the default settings (every second, "screenshots" directory):

```bash
./target/release/screen-cap.exe -w "Firefox"
```

Capture screenshots with all default values:

```bash
./target/release/screen-cap.exe
```

## Error Handling

Screen-Cap handles the following errors gracefully:

  * **Directory creation failure:** If the specified output directory cannot be created, an error message is printed, and the program exits.
  * **Window not found:** If the target window is not found, a message is printed, and the program exits.
  * **Screenshot capture/save failure:** If there's an issue capturing or saving a screenshot, an error message is printed, but the program continues to run (unless the window has closed).
  * **Target window closure:** If the target window is closed during execution, Screen-Cap will detect this and exit cleanly.

## License

[MIT](https://www.google.com/url?sa=E&source=gmail&q=LICENSE)
