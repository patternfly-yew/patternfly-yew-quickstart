# PatternFly quick start for Yew

This is a quickstart template to get you started with PatternFly and Yew.

Here you can see this page in action: https://ctron.github.io/patternfly-yew-quickstart

## Pre-requisites

* Rust

  ```shell
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

* Trunk

  ```shell
  cargo install trunk
  ```

* NodeJS `npm`

## Initialize

Fetch the PatternFly dependencies:

```shell
npm ci
```

## Run local developer setup

Start a local development server, which re-builds every time you make changes to the code:

```shell
trunk serve
```

Direct your web browser to: http://localhost:8080

## Perform a release build

To build the Rust components and package up the page, run:

```shell
trunk build --release
```

The release is in the `dist/` folder.
