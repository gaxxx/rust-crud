-- example HTTP POST script which demonstrates setting the
-- HTTP method, body, and adding a header

wrk.method = "POST"
wrk.body   = '{ "name": "name", "identity": "id", "hometown": "hubei", "age": 100 }'
wrk.headers["Content-Type"] = "application/json"
