# Porcupette

Porcupette is a "default browser". Sometimes you don't want the link you clicked to open in the browser, instead you want the link to be copied, or you want a command to get executed. Because you don't want to accidentally open a link that you don't want to, or you have a more complex use case than just opening the browser (scripting etc.).

Porcupette is [Porcupine](https://github.com/micahflee/porcupine), but with Rust. The Porcupine is written in Python and uses Qt. Although Qt is a good framework, for a program which needs to swiftly do a simple task, Qt is a heavy overhead, and Python is slow. I noticed a latency when using Porcupine, so I decided to rewrite it in Rust.

## Prerequisites

Porcupette is tested and supported on Linux. `xdg-settings` (as part of the `xdg-utils` package) is needed (but not required) to set porcupette as your default browser on Linux systems.

## Usage

For setup, run porcupette with no arguments:

```
porcupette
```

Test porcupette with a URL:

```
porcupette https://www.youtube.com/watch?v=KMU0tzLwhbE
```
