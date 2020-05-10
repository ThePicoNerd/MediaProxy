# Media Proxy Router

A standalone binary that normalizes the incoming requests before passing them onto MediaProxy. It is designed to be used in combination with Nginx, so that the cache works correctly (since Nginx stores cache based on the request bodies).

![](https://github.com/ThePicoNerd/mediaproxy-router/workflows/Master%20Release/badge.svg)

## API

### Media Proxy request object

#### Query

| field   | type    | description                         |
| ------- | ------- | ----------------------------------- |
| source  | string  | the URL of the original image       |
| width?  | integer | the new width of the image          |
| height? | integer | the new height of the image         |
| format  | string  | an [output format](#output-formats) |

#### Output formats

| format |
| ------ |
| `jpeg` |
| `png`  |
| `webp` |
| `gif`  |


### `GET /`

#### Query string parameters

| field       | type   | description                                                                                                                           |
| ----------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| fingerprint | string | URL-safe base64 (with `-` and `_`, see [RFC 3548](https://tools.ietf.org/html/rfc3548#section-4)) encoded [query](#query) JSON object |

Returns the processed media.

### `POST /`

#### JSON parameters

The JSON body should be a [query](#query) object. The processed media will be returned.