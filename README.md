# Porcupette

Porcupette is a program that is intended to be set as the default browser. Sometimes you don't want to choose a default browser, but instead, you want the URL to be copied or a command to get executed when you click a link inside a program. Because you don't want to accidentally open a link that you don't want to, or you have a more complex use case than just opening the default browser.

Porcupette is [Porcupine](https://github.com/micahflee/porcupine), but with Rust. The Porcupine is written in Python and uses Qt. Although Qt is a good framework, for a program which needs to swiftly do a simple task, Qt is a heavy overhead, and Python is slow. I noticed a latency when using Porcupine, so I decided to rewrite it in Rust.

## Prerequisites

Only `xdg-settings` (as part of the `xdg-utils` package) is needed on Linux systems.

## Usage

For setup, run porcupette with no arguments:

```
porcupette
```

Test porcupette with a URL:

```
porcupette https://www.youtube.com/watch?v=KMU0tzLwhbE
```

Lastly, set it as your default browser.
