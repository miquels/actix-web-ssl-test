# actix-web-ssl-test

A simple setup to see how the different TLS and HTTP options compare.

plaintext vs openssl vs native_tls and HTTP/1.1 vs HTTP2.

Result: on linux, it appears (except for connection setup) that TLS is about just as
fast as plaintext. Openssl and native_ssl are the same, which makes sense, since
native_tls is just a wrapper around openssl on Linux.

However, it appears that HTTP/2 is quite a bit slower than HTTP/1.1, which is
unexpected since a design goal of HTTP/2 is to be faster than HTTP1.1.

Testing with `ab` (apachebench) gives some weird results. The performance is quite
disappointing. This is because `ab` still uses HTTP/1.0. The `-k` command line
switch makes it send the `Connection: keep-alive` header, but `actix-web` ignores that
header and bases keep-alive only on the HTTP version (<= 1.0: off, >= 1.1: on).
The code has been fixed now to force `actix-web` to use a keep-alive connection even
on HTTP/1.0 if there is a `Connection: keep-alive` header.

## Performance testing using `ab` before the actix-web keep-alive fix.

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
$ ab -n 100 -k https://localhost.xs4all.net:8082/
Time taken for tests:   1.917 seconds
Complete requests:      100
Requests per second:    52.16 [#/sec] (mean)
Time per request:       19.173 [ms] (mean)
```

## Performance testing using `ab` after the actix-web keep-alive fix.

### `ab` plaintext HTTP/1.1
```
$ ab -k -n 10000 http://localhost.xs4all.net:8080/
Time taken for tests:   0.293 seconds
Complete requests:      10000
Requests per second:    34158.72 [#/sec] (mean)
Time per request:       0.029 [ms] (mean)
```

### `ab` TLS HTTP/1.1
```
$ ab -k -n 10000 https://localhost.xs4all.net:8082/
Time taken for tests:   0.437 seconds
Complete requests:      10000
Requests per second:    22899.91 [#/sec] (mean)
Time per request:       0.044 [ms] (mean)
```

## Performance testing using `ab` with Keep-Alive, and concurrency:

### `ab` plaintext HTTP/1.1, 32 threads
```
$ ab -c 32 -k -n 10000 http://localhost.xs4all.net:8080/
Concurrency Level:      32
Time taken for tests:   0.102 seconds
Complete requests:      10000
Requests per second:    98489.18 [#/sec] (mean)
Time per request:       0.010 [ms] (mean, across all concurrent requests)
```

### `ab` TLS HTTP/1.1, 32 threads
```
$ ab -c 32 -k -n 10000 https://localhost.xs4all.net:8082/
Concurrency Level:      32
Time taken for tests:   0.168 seconds
Complete requests:      10000
Requests per second:    59394.77 [#/sec] (mean)
Time per request:       0.017 [ms] (mean, across all concurrent requests)
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

