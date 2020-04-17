# Media Proxy

![CI](https://github.com/ThePicoNerd/MediaProxy/workflows/CI/badge.svg)

## Requirements

- `libssl-dev`

## API

The API is very simple, with only one endpoint at the time of writing.

### `POST /`

**JSON parameters**

| Field   | Type    | Description                                                             |
| ------- | ------- | ----------------------------------------------------------------------- |
| width?  | integer | width of the new image                                                  |
| height? | integer | height of the new image                                                 |
| source  | url     | URL of the original image                                               |
| format  | enum    | output format of the new image (one of `png`, `jpeg`, `webp` and `gif`) |
