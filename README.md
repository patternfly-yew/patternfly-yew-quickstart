# PatternFly quick start for Yew

This is a quickstart template to get you started with PatternFly and Yew.

Here you can see this page in action: https://ctron.github.io/patternfly-yew-quickstart

## Pre-requisites

* Rust

      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

* Trunk

      cargo install trunk

* NodeJS `npm`

## Initialize

Fetch the PatternFly dependencies:

    npm install

## Run local developer setup

Start a local development server, which re-builds every time you make changes to the code:

    trunk serve

Direct your web browser to: http://localhost:8080

## Perform a release build

To build the Rust components and package up the page, run:

    trunk build

The release is in the `dist/` folder.
