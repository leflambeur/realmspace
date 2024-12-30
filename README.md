# Realmspace

Project to learn Rust, UX Design, Reactice Webdev, Data Structure/API Design, FullStack Application building etc


## Get Started:

To get started you need the rust toolchain and some extras:

### Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
This project is built against `Stable` not `Nightly`.

### Install Extras:

```bash
cargo install cargo-leptos
cargo install tauri-cli
```

### Setup the Correct Targets:

```bash
rustup target add wasm32-unknown-unknown
```

### Build the project

```bash
cargo tauri dev
```

## What's going on here?

This project is for me to learn several disciplines I am interested in. It's been a while since I have done any development on my own, and truth be told I have only ever been a contributor to other projects. 

I don't think I have ever done a project for my own sake, so I wanted to get back into it at the deep end building a Rust-based Fullstack app.

### Project Structure


Majority of work and contributions are here:
* `app/src/lib.rs` - the main GUI application
* `app/src/components` - components
* `app/src/styles` - all stylesheets
* `backend/` - async functions that can be called by the GUI for server-side logic

You also have these which are mostly invisible:

* `frontend` - backup/otel leftover from the template I am too afraid to drop
* `server` - main entrypoint for starting the application
* `src-tauri` - configuration and build scripts for tauri to build different applications properly


### Disclaimers: 
* I started with Rust the same day I started this project (as of writing this ReadMe that makes it 2 days)
* I have some limited Frontend experience but not a lot so I am kind of learning this at the same time
* While I have a ton of devops/platform eng experience, it's usually setting up CI/CD etc not actually structuring projects correctly so forgive the mess
* I use an M3 mac and rely on Tauri giving me a native application window to test - your mileage may vary

This is all to say I am probably making a big mess and what works for me may not work for you but it's part of the journey.


### Important Packages

* Leptos - a Rust-based Web-Framework that compiles into WASM
  * Specifically this is the SSR Flavour - this is the bulk of the logic
  * I chose this as I wanted something as performant and small as possible and Leptos seemed to be a favourite for most people and I love the idea of WebAssembly being a big fan of some of the projects I have come across in the Kubernetes/Platform Eng space
* Tauri - A Rust-Based Application Builder
  * This builds my application into both a browser app but also a native app (native being relative ofc)
* Tailwind CSS - very common CSS post-processor
  * I have never been into Tailwind but it keeps coming up and I do a lot of UX work in my day job so wanted some more experience with it and your options for components librariesin Rust are limited at best

I started with the very [impressive template](https://github.com/Alt-iOS/tailwind-leptos-tauri-template) found here:
```bash
https://github.com/Alt-iOS/tailwind-leptos-tauri-template
```

You'll notice some of the project structure is similar at first glance - I have made a number of updates:

* The template mostly relies on syntax from pre-0.7 Leptos and 0.7 Leptos introduced some very significant changes 
  * I have updated this project to the latest packages and syntax
* I have introduced a components folder with 'base' and will be extending this as I bring in more components myself
* I have ported a couple of components from JSX to Rust from [RetroUI](https://www.retroui.io/components) - mainly because I thought they were cute and I thought it would be a good first test of what I learned
* I have changed the dependency structure to be a bit cleaner using `cargo autoinherit` because I got annoyed updating leptos in 5 cargo files - I do recommend cargo-autoinherit a lot actually ([details here](https://github.com/mainmatter/cargo-autoinherit))
* I have cleaned up a lot of bits and pieces including moving around stylesheets and adding the idea of split out css modules per component and making sure tailwind is prepared for that (not totally happy with the current layout but I will get there)
* I added some extra improvements and QoL fixes around RustRover/Rust-Toolchain as well
* I took out all the tracing and otel integrations and will probably restore them at a later date

### Future Integrations:
* [Redb](https://github.com/cberner/redb)
  * Another highly performant Rust Service - A K/V Store that I am excited to begin learning data structuring for
* Trying to run on a Pi or PiZero (or pref Pico) 
  * One of the reasons I have chosen WebAssembly is for the optimal binary sizes and security to try and fit some or all of the project on a tiny device - this is a stretch goal and I need more progress to get there
Thanks to the leptos and tauri teams for the amazing work.
