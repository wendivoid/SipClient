# Nirah

**WIP** Not much to see here yet Changes frequently and isn't guaranteed to work.

## Project Structure
    .
    ├── core                   # Core Library Files
    ├── data                   # example data used for testing
    └── nirah                  # The actual daemon binary

### Core
  The core library holds all the provider abstractions. A `Provider` is just a rust object that implements a specific Provider Trait as well as the main `nirah_core::core::Provider` trait.

  #### AccountsProvider
  This provider manages the accounts Nirah can connect to.

  #### ConfigProvider
  This provider manages configuration variables for Nirah.

  #### ContactsProvider
  This provider manages contacts for Nirah.

  #### DatabaseProvider
  This provider handle's saving message data for Nirah.

  #### NotifierProvider
  This provider sends notifications when a message/call is received.

  #### RpcProvider
  Handles sending and receiving rpc messages.

  #### RpcHandlerProvider
  Performs functions with received rpc messages.

  #### SIP Provider
  This provider handles the SIP protocol messages for Nirah.

  #### Streaming Provider
  This provider handles playing audio streams.

## nirah
  This is the actual soft phone it consists of a daemon & cli program for interacting
  with the daemon.
