# Disturbance

[![Crate](https://img.shields.io/crates/v/disturbance.svg)](https://crates.io/crates/disturbance)
[![Build Status](https://travis-ci.com/crodjer/disturbance.svg?branch=master)](https://travis-ci.com/crodjer/disturbance)

Monitor disturbances in a web service's behaviour.

You may use `disturbance` as a monitoring utility which is up over an
extended period of time to know if a service ever went down/unhealthy.
Eg: Uptime during a deployment.

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
    -w, --wait <wait>                  Wait time (in ms) between requests per worker. [default: 100]

ARGS:
    <url>    The web service's URL to monitor``
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

- Set a custom wait time between requests (default: 100 ms), per
  worker. If you want
  ```
  $ disturbance https://example.com/ -t 1 -w 500
  ErrorResponse("https://example.com/: timed out") => 4
  ```
