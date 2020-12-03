# Patternfly quick start for Yew

This is a quickstart template to get you started with Patternfly and Yew.

Here you can see this page in action: https://ctron.github.io/patternfly-yew-quickstart

## Pre-requisites

* Rust

      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

* Cargo `wasm-pack`

      cargo install wasm-pack

* NodeJS `npm`

## Initialize

Fetch all dependencies:

    npm install

## Run local developer setup

Start a local development server, which re-builds every time you make changes to the code:

    npm run start:dev

Direct your web browser to: http://localhost:8000

## Perform a release build

To build the Rust components and package up the NPM dependencies, run:

    npm run build

The release is in the `dist/` folder.
