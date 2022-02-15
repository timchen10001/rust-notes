# Http Server Implementation

## Testing Tool - `netcat`

```bash
  # listening localhost:8080
  netcat -l -s 127.0.0.1 -p 8080

  # send http request to locale server
  echo "GET / HTTP/1.1\r\nTESTING" | netcat 127.0.0.1 8080
```