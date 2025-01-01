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

### Run the project in dev

#### Recommended (Browser Dev)
To just run the webserver and application code
```bash
cargo leptos watch
```
And then just open your browser at http://localhost:8000

This is the best for rapid testing as the initial build from a clean directory is ~2m with the Dev Profile and subsequent reloads are usually 0.10ms for me.

#### Alternative (Native Dev)
To run and test inside a native window:
```bash
cargo tauri dev
```
This will spawn a running instance for you on your native OS

>[!IMPORTANT]
>Tauri is a large project and can take 4-5x longer to build - it does run `cargo leptos watch` before it starts as well so is at least as long as that and tauri is at least 2x bigger than leptos.

## What's going on here?

This project is for me to learn several disciplines I am interested in. It's been a while since I have done any development on my own, and truth be told I have only ever been a minor contributor to projects I have worked under. 

I don't think I have ever done a project for my own sake, from scratch, or comprehensively, so I wanted to get back into it at the deep end building a Rust-based Fullstack app making use of several technologies that are probably overkill but I am interested in.

### Goal
Make a delightful, performant, OpenAI API compatible chat client, that can ideally run on the smallest footprint possible (ideally a pico or zero pi) with all the features I want with a consistent experience across all of my devices.

### Why?

I didn't jump on the chatgpt bandwagon immediately as I have worked in several security-focused companies where the idea of asking a cloud service for help with PM, Marketing, Requirements content could constitute a leak - not to mention the lack of knowledge about where you data is processed in the end or how it trains the underlying models at the lower tiers.

I wanted to run local LLMs as I knew they could help me with the really dull tasks like generating copy/content, first I got started with [Msty](https://msty.app), which is a really fantastic app and I heartily recommend and have even paid for.

Msty is awesome but there were some hiccups I had with exposing an instance from my laptop to other devices as I travel a lot and trusting other networks with no control is not an option, it's not opensource so I can't verify what telemetry is being sent to and from it and I believe in trust but verify (and I can't verify),
and it abstracts away a lot of what's going on under the hood and controls a lot of the detail on how LLMs are installed, operated or loaded so when things go wrong I can't really see what and why.

Also it's features are amazing but I struggled to then switch to non-Msty iOS clients in a way that gave me the same consistent experience.

If you want to get started with a powerful client that allows for some really fantastic features with using multiple models AND looks the part do seriously check it out.

However, I am not such a power-user in terms of how I use multiple LLMs, instead I just want the flexibility to run bigger LLMs that aren't restricted to my laptop without having to pay for a whole new device, and to expose them to my lab network wherever I am even if I am travelling.

In my case, I have a home lab so I can keep my laptop free and 24GB is not a lot to work with anyway when it comes to more powerful LLMs.

Additionally, I have a load of RPis, the ARM and RPi teams are local to my hometown, and it's nice to represent. Also I worked on K3s as a PM, so I have a lot of affection for edge/small IOT production deployments.

Pis are also nice as they are decent, affordable and accessible reference hardware so makes it easier to communicate the idea you are using and provide a tangible constraint on you design decisions.

This led me to a very exciting project called [Exo](https://github.com/exo-explore/exo) which lets you cluster up your devices and will split requests across them - essentially allowing you to make way higher memory pools for loading significant models - it isn't perfect as splitting the model into layers doesn't guarantee performance and with RPIs there's only so much throughput you can manage - but it will do the job.

### Project Structure


Majority of work and contributions are here:
* `app/src/lib.rs` - the main GUI application
* `app/src/components` - components
* `app/src/styles` - all stylesheets
* `backend/` - async functions that can be called by the GUI for server-side logic

You also have these which are mostly invisible:

* `frontend` - backup/otel leftover from the template I am too afraid to drop
* `server` - main entrypoint for starting the application - this is setup already
* `src-tauri` - configuration and build scripts for tauri to build different applications properly


### Disclaimers: 
* I started with Rust the same day I started this project, this means I likely have gotten a lot wrong
* I have some limited Frontend experience but not a lot, so I am learning Tailwind etc at the same time
* While I am experience with devops/platform eng, I never normally need to structure projects myself correctly so forgive the mess
* I use an M3 mac air 24GB - your mileage may vary

This is all to say I am probably making a big mess and what works for me may not work for you, but it's part of the journey.


### Important Packages

* Leptos - a Rust-based Web-Framework that compiles into WASM
  * Specifically this is the SSR Flavour - this is the bulk of the logic
  * I chose this as I wanted something as performant and small as possible and Leptos seemed to be a favourite for most people and I love the idea of WebAssembly.
* Tauri - A Rust-Based Application Builder
  * This builds my application into a native app (native being relative ofc)
* Tailwind CSS - popular CSS post-processor
  * I have never been into Tailwind but it keeps coming up so wanted some more experience with it

I started with the very [impressive template](https://github.com/Alt-iOS/tailwind-leptos-tauri-template) found here:
```bash
https://github.com/Alt-iOS/tailwind-leptos-tauri-template
```

You'll notice some of the project structure is similar at first glance - I have made a number of updates:

* The template mostly relies on syntax from pre-0.7 Leptos and 0.7 Leptos introduced some very significant changes 
  * I have updated this project to the latest packages and syntax - with only the effects/actions syntax not quite making sense to me
* I have introduced a components folder with 'base' and will be extending this as I bring in more components myself
* I have ported a couple of components from JSX to Rust from [RetroUI](https://www.retroui.io/components) - mainly because I thought they were cute and I thought it would be a good first test of what I learned
* I have changed the dependency structure to be a bit cleaner using `cargo autoinherit` because I got annoyed updating leptos in 5 cargo files - I do recommend `cargo-autoinherit` ([details here](https://github.com/mainmatter/cargo-autoinherit))
* I have cleaned up a lot of items including moving around stylesheets and adding the idea of split out css modules per component (not totally happy with the current layout but I will get there)
* I added some extra improvements and QoL fixes around RustRover/Rust-Toolchain as well
* I took out all the tracing and otel integrations and will probably restore them at a later date

### Future Integrations:
* ~[Redb](https://github.com/cberner/redb)~ [Bevy ECS]() + [Bevy PKV](https://github.com/johanhelsing/bevy_pkv)
  * ~Another highly performant Rust Service - A K/V Store that I am excited to begin learning data structuring for~
  * Most likely I am going for Bevy ECS with Bevy PKV which uses RedDB underneath and gives me ECS as a dataframework
* [Protobufs](https://github.com/tokio-rs/prost)
  * Not sure how feasible protobufs are for Leptos with it's server functions being inline, or with ECS handling data but I want to learn them and use them properly for personal reasons
* Trying to run on a Pi or PiZero (or pref Pico) 
  * One of the reasons I have chosen WebAssembly is for the optimal binary sizes and security to try and fit some or all of the project on a tiny device - this is a stretch goal and I need more progress to get there
* [Leptos Islands](https://book.leptos.dev/islands.html)
  * This feels like the correct way to optimise performance utilizing pure html+css where possible and then wasm for more complex data streaming tasks
Thanks to the leptos and tauri teams for the amazing work.
