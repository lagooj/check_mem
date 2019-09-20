# check_mem
[![Build Status](https://travis-ci.com/lagooj/check_mem.svg?branch=master)](https://travis-ci.com/lagooj/check_mem)

# Purpose
* Check memory and swap sage, should be runned by icinga, nagios
* Still playing around with easy rust

## Usage

```
USAGE:
    check_mem <TYPE> --critical <CRITICAL> --warning <WARNING>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --critical <CRITICAL>    Set critical threshold
    -w, --warning <WARNING>      Set warning threshold

ARGS:
    <TYPE>    type mem or swap
```
