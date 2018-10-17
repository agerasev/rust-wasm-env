#!/usr/bin/python3

from http.server import HTTPServer, SimpleHTTPRequestHandler

addr = ("", 8000)
with HTTPServer(addr, SimpleHTTPRequestHandler) as httpd:
    print("[info] http server started on %s:%s" % addr)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    print("[ok] server stopped")
