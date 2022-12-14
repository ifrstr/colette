# colette

[![Go Reference](https://pkg.go.dev/badge/gopkg.ilharper.com/x/colette.svg)](https://pkg.go.dev/gopkg.ilharper.com/x/colette)
[![Go Report Card](https://goreportcard.com/badge/gopkg.ilharper.com/x/colette)](https://goreportcard.com/report/gopkg.ilharper.com/x/colette)
[![codecov](https://codecov.io/gh/ifrstr/colette/branch/master/graph/badge.svg?token=I4FWALY4EV)](https://codecov.io/gh/ifrstr/colette)

Convert between truecolor, xterm(1) 256 color and 16 color. Single-header library. Arranged from [tmux](https://github.com/tmux/tmux).

## C

### Install

Using prebuilt packages:

Download artifacts from [Releases](https://github.com/ifrstr/colette/releases), including:

- The header source

- Deb package

- Rpm package

Or using `make install`:

```sh
mkdir build && cd build && cmake .. && make && make install
```

Or using submodule:

```sh
git submodule add --name colette https://github.com/ifrstr/colette.git colette
```

```cmake
add_subdirectory(colette)
```

### Usage

Checkout [colette.h](https://github.com/ifrstr/colette/blob/master/include/colette/colette.h).

## Go

## Install

```sh
go get gopkg.ilharper.com/x/colette
```

### Usage

[![Go Reference](https://pkg.go.dev/badge/gopkg.ilharper.com/x/colette.svg)](https://pkg.go.dev/gopkg.ilharper.com/x/colette)

## LICENSE

[ISC](https://github.com/ifrstr/colette/blob/master/LICENSE), the same as [tmux](https://github.com/tmux/tmux)
