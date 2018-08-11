# actix-web-ssl-test

A simple setup to see how the different TLS and HTTP options compare.

plaintext vs openssl vs native_tls and HTTP/1.1 vs HTTP2.

Result: on linux, it appears (except for connection setup) that TLS is about just as
fast as plaintext. Openssl and native_ssl are the same, which makes sense, since
native_tls is just a wrapper around openssl on Linux.

However, it appears that HTTP/2 is quite a bit slower than HTTP/1.1, which is
unexpected since a design goal of HTTP/2 is to be faster than HTTP1.1.

Also, `ab` (apachebench) performance is disappointing. I think it renegotiates something
TLS related for every request. As `curl` does not do this, it must be something `ab`-specific.
Might be worth figuring out what, to see if any real-world clients show the same behaviour.

## Performance testing using `ab`

### `ab` plaintext HTTP/1.1
```
$ ab -n 10000 -k http://localhost.xs4all.net:8080/
Time taken for tests:   1.036 seconds
Complete requests:      10000
Requests per second:    9655.15 [#/sec] (mean)
Time per request:       0.104 [ms] (mean)
```

### `ab` TLS HTTP/1.1
```
Time taken for tests:   1.917 seconds
Complete requests:      100
Requests per second:    52.16 [#/sec] (mean)
Time per request:       19.173 [ms] (mean)
```


## Performance test using curl.

### plaintext HTTP/1.1
```
$ curl -s -k --http1.1 -w '%{time_total}\n' http://localhost.xs4all.net:8080/{1,2,3,4}
0.004625
0.000162
0.000118
0.000118
```

## plaintext HTTP/2
```
$ curl -s  -k --http2-prior-knowledge -w '%{time_total}\n' http://localhost.xs4all.net:8080/{1,2,3,4}
0.004933
0.000185
0.000179
0.000176
```

### openssl HTTP/1.1
```
$ curl -s -k --http1.1 -w '%{time_total}\n' https://localhost.xs4all.net:8081/{1,2,3,4}
0.036808
0.000140
0.000115
0.000117
```

### openssl HTTP/2
```
$ curl -s -k --http2-prior-knowledge -w '%{time_total}\n' https://localhost.xs4all.net:8081/{1,2,3,4}
0.034956
0.000217
0.000182
0.000173
```

### native_tls HTTP/1.1
```
$ curl -s -k --http1.1 -w '%{time_total}\n' https://localhost.xs4all.net:8082/{1,2,3,4}
0.036615
0.000148
0.000131
0.000124
```

### native_tls HTTP/2
```
$ curl -s -k --http2-prior-knowledge -w '%{time_total}\n' https://localhost.xs4all.net:8082/{1,2,3,4}
0.038324
0.000211
0.000210
0.000183
```

