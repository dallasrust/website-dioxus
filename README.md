# Dallas Rust User Meetup website using Dioxus

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/dallasrust/website-dioxus/blob/main/LICENSE.txt

- Uses static prerendering with hydration

## Utilities Installation

- Install the Rust command line utility "cargo"
  - cargo is installed when you install Rust
  - https://www.rust-lang.org/
- Install the Dioxus Command Line Interface (CLI) "dx"
  - cargo install dioxus-cli --locked
  - https://github.com/DioxusLabs/dioxus/tree/master/packages/cli
- Install npm
  - npm installs utilities such as prettier
  - npm scripts run the dx and cargo commands
  - npm can be installed by installing node.js
  - https://nodejs.org/

## Quick Start

- npm install
- npm test

## npm scripts

- npm start
  - Used during development
  - Builds, watches, and serves with hot reloading
  - Automatically opens a browser window
- npm run clean
  - Deletes the build output and distribution directories
- npm run build
  - Builds a release version with static site generation (SSG)
- npm run merge
  - Makes the distribution directory dist/
  - Merges the release build into dist/
  - Merges the generated SSG files into dist/
  - Merges the static files in merge/ into dist/
- npm run dist
  - Runs the clean, build, and merge scripts
  - Used to generate an SSG distribution in the dist/ directory
  - The dist/ files can be hosted on a Content Delivery Network (CDN)
- npm run serve
  - Serves the files in the distribution directory dist/
  - Automatically opens a browser window
- npm test
  - Runs the dist and serve scripts
  - Used to test the SSG distribution prior to hosting on a CDN
- npm run format
  - Runs the "prettier" utility to format the generated HTML files
  - Useful for analyzing or debugging the generated HTML files

## History

- 2023-08-12: Project launch
- 2025-03-15: Updated from Dioxus v0.4 to v0.6
