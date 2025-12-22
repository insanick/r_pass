# r_pass
Password Generator project built on Rust

## Usage

```
./r_pass
```

Creates a quick secure* secret key

_***[16 character string, alphanumeric and symbols]**_

## Planned Updates
- Graceful Failures
- Interactive mode
    - Will prompt the user for desired password options. (Complexity and length)

## Panics

This binary currently does **NOT** fail gracefully. 

However, the only reason this code would not run successfully is if stdout cannot be flushed. I will be adding graceful failures in a future release. 