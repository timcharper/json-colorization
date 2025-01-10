# JSON Colorization

This little program colorizes and formats mixed text/JSON logs on-the-fly. Useful for visualizing logs for local running processes that mostly log JSON.

JSON is formatted in pretty-print format up to 2 levels deep, after which compact formatting is used to maintain readability.

## Features

- Automatic JSON detection (lines starting with `{` or `[`)
- Smart formatting: pretty-print for outer levels, compact for deeper nesting
- Color scheme:
  - Object keys: Bold Blue
  - Strings: Green
  - Numbers: Yellow
  - Booleans: Purple
  - Null: Red
  - Non-JSON lines: White

## Building and installing

```bash
cargo build --release
sudo mv target/release/json-colorization /usr/local/bin
```

## Usage

Pipe any output that contains JSON lines:

```bash
# Process logs
yarn serve 2>&1 | json-colorization

# File input
cat logfile.txt | json-colorization

# Direct JSON formatting
echo '{"hello": {"world": {"deeply": {"nested": "object"}}}}' | json-colorization
```

### Example Output

For input like:
```
{"level": "info", "msg": "Server starting", "config": {"port": 3000, "env": "dev", "params": {"a": 1, "b": 2, "arr": [1,2,3]}}}
Plain text log line
{"error": {"code": 500, "details": {"stack": "..."}}}
```

The output will be colorized with the first two levels pretty-printed and deeper levels in compact format.

```
{
  "config": {
    "env": "dev",
    "params": {"a": 1, "arr": [1, 2, 3], "b": 2},
    "port": 3000
  },
  "level": "info",
  "msg": "Server starting"
}
Plain text log line
{
  "error": {
    "code": 500,
    "details": {"stack": "..."}
  }
}

```