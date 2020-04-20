# MediaProxy Router

A standalone binary that normalizes the incoming requests before passing them onto MediaProxy. It is designed to be used in combination with Nginx, so that the cache works correctly (since Nginx stores cache based on the request bodies).
