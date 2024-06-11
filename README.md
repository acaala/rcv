# Rust Converter

This tool allows you to convert image files or all images in a directory to WebP format with specified quality.

## Usage

```sh
rcv [OPTIONS]
```

## Options

| Short | Long          | Description                                                                          | Default   |
|-------|---------------|--------------------------------------------------------------------------------------|-----------|
| `-i`  | `--input_file`| Path to a single image file to be converted. Only `input_file` or `directory` may be used, not both. | `None`    |
| `-d`  | `--directory` | Path to a directory containing images to be converted. Only `input_file` or `directory` may be used, not both. | `None`    |
| `-o`  | `--output_path` | Path to the output directory where converted images will be saved.                | Required  |
| `-q`  | `--quality`   | Quality of the output WebP images.                                                  | `75.0`    |

### Examples

Convert a single image file:
```sh
rcv -i input.png -o ./output -q 80
```

Convert all images in a directory:
```sh
rcv -d ./images -o ./output -q 80
```
