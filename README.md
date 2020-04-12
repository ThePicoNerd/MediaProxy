# Media Proxy

## Dependencies

- `libssl-dev`

## TODO:

- Proper error handling
- Kubernetes

## API

The API is very simple, with only one endpoint at the time of writing.

### `GET /`

| Field   | Type    | Description                                                     |
| ------- | ------- | --------------------------------------------------------------- |
| width?  | integer | width of the new image                                          |
| height? | integer | height of the new image                                         |
| source  | url     | URL of the original image                                       |
| format  | enum    | output format of the new image (one of `png`, `jpeg` and `gif`) |
