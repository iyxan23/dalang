<img src=".github/icon.svg" align=left height="150px" width="150px">
<h1>Dalang</h1>

[![Rust](https://github.com/Iyxan23/dalang/actions/workflows/rust.yml/badge.svg)](https://github.com/Iyxan23/dalang/actions/workflows/rust.yml)
[![Hits-of-Code](https://hitsofcode.com/github/iyxan23/dalang?branch=main&label=Hits-of-Code)](https://hitsofcode.com/github/iyxan23/dalang/view?branch=main&label=Hits-of-Code)

A video editor with a client-server based approach. Powered with MLT

## What is this?

A hobby project where I try to understand how video works and to make something that's somewhat production-ready. The dalang project aims to be a web-based video editor where the process happens in the server. The server is written in rust with the help of `actix-web` and good ol' `actix` actors, with the frontend being powered by vue. They use websocket and msgpack to communicate in between.

It's more of a project where I could throw anything I'm really curious about. The combination of alien framework and libraries like vue, actix (and its actor framework) with msgpack is pretty far from my current knowledge, and I want to try to learn those things.

## Overview

Dalang is a web-based cloud video editor. Users logs in or registers an account of a dalang server, and start editing right on the browser. It does not use much resources to run in the client, because pretty much everything heavy is handled on the server; things such as video preview, rendering, and storage are done in the server. The client only acts as a facade to the server. It is primarily targets studios where they have big servers on LAN connections (where latency is pretty much zero). Imagine editing at that server's speed on just your regular work laptop. Despite that, it will also have options connections that has low-mid latency.

It will also be **distributed**. With the use of the actor programming paradigm, we could distribute these actors to multiple servers, and to connect them together to distribute load between servers and to not have a single-point-of-failure.

## The Name

The name "dalang" comes from the Javanese culture of _wayang_ performance, a traditional puppeteer show, where the person that sits and performs the performance is called the "dalang". It's as if you're the "dalang" of the show (the editor), moving puppets (clips) together to form a single video (a wayang performance).

## Contributing?

I don't really think anybody would contribute to this; this is something of a hobby project of mine, that -might- will somehow, someday, will vanish in existence (like many of my other hobby projects). But if you do, that'd be awesome, since I'm looking for experiences working with someone else! :>

If you really do want to, I haven't done much documentation yet, so you'd have to read through the spaghetti code for now..
