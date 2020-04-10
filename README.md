# Media Proxy

## Dependencies

- `libssl-dev`

## TODO:

- Size limits (so that people don't upload petabyte-sized images and scale them by a million)
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
