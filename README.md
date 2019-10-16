# Disturbance

A utility to test if there were any disturbances in a services
repsponse during deployment

## Usage
```
USAGE:
    disturbance [OPTIONS] <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --excludes <excludes>          Response should not match
    -m, --matches <matches>            Response should match
    -p, --parallelism <parallelism>    Parallelism [default: 2]
    -t, --timeout <timeout>            Request timeout in seconds [default: 5]

ARGS:
    <url>    The URL to hit
```

## Example

- Check a website's response distribution to simple `GET` requets.
  ```
  $ disturbance https://example.com/
  Success(200) => 14
  ```

- Check a website while also requring a pattern to be present.  
  With a pattern that isn't present in the responses:
  ```
  $ disturbance https://example.com/ -m test
  DoesNotMatch => 407
  ```
  With intermittent matches (potentially unstable website):
  ```
  $ disturbance https://example.com/ -m true
  Success(200) => 64, DoesNotMatch => 10
  ```
- Use an exlusion pattern, to ensure that responses never contain the
  string that you paas.
  ```
  $ disturbance https://example.com/ -e '"success":false'
  DoesNotExclude => 12
  ```
  Or a good service:
  ```
  $ disturbance https://example.com/ -e '"success":true'
  Success(200) => 18
  ```
- Configure parallelism to control the number of parallel workers
  (defaults to 2).
  ```
  $ disturbance https://example.com/ -p 4
  Success(200) => 128
  ```

- Set a custom timeout in seconds (default 5)
  ```
  $ disturbance https://example.com/ -t 1
  ErrorResponse("https://example.com/: timed out") => 4
  ```
