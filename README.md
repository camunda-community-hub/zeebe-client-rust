
![Community Extension](https://img.shields.io/badge/Community%20Extension-An%20open%20source%20community%20maintained%20project-FF4700)
[![](https://img.shields.io/badge/Lifecycle-Proof%20of%20Concept-blueviolet)](https://github.com/Camunda-Community-Hub/community/blob/main/extension-lifecycle.md#proof-of-concept-)
![Compatible with: Camunda Platform 8](https://img.shields.io/badge/Compatible%20with-Camunda%20Platform%208-0072Ce)


# Zeebe Rust Client
Rust client for Zeebe in early stages. Suitable for early adopters

Features
* CLI for all commands based on Zeebe Client 8.0
* Support for OAuth authentication

Next Steps
1. Publish crates
2. Worker implementation
3. Build an application that uses this Rust client

## Cli Tool

> **Warning**
> The Cli Tool might leak credentials in log messages

Run `cargo run -- help` to see available commands and options.

**Authentication for Camunda Cloud**

First, [generate and download](https://docs.camunda.io/docs/next/components/console/manage-clusters/manage-api-clients/) client credentials for Camunda Cloud. Let's assume they are in a file called credentials.txt.

The `credentials.txt` file contains environment variables, so let's source them:
```shell
$ source credentials.txt
```

Finally, run the cli tool: 

```shell
$ cargo run -- status
TopologyResponse {
    brokers: [
        BrokerInfo {
            node_id: 0,
...
```

Alternatively, you can also provide your credentials as arguments:

```shell
$ cargo run -- --address <ADDRESS> --client-id <CLIENT_ID> --client_secret <CLIENT_SECRET> --authorization-server <AUTH_SERVER> status 
TopologyResponse {
    brokers: [
        BrokerInfo {
            node_id: 0,
...
```

## Prior Work/Alternatives

These repositories also implement Zeebe clients for Rust. Most of them are more feature-complete than this repository currently, but also a little older.
* https://github.com/camunda-community-hub/zeebest
* https://github.com/xmclark/zeebe-client-rust-1
* https://github.com/OutThereLabs/zeebe-rust

Your best choice is probably: https://github.com/OutThereLabs/zeebe-rust

# Developer Guide

## CLI

**Conventions for CLI Argument Annotations**
* Named parameters by default
* Use positional parameters only if there is just one parameter, or just one required parameter
* For required parameters use short and long version `#[clap(short, long)]`
* For optional parameters use long version only `#[clap(long, default_value_t = 1)]`
