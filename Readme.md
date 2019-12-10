# Nirah

**WIP** Not much to see here yet, This project is currently commited
stupidly to back up my current progress. Changes frequently and isn't garenteed to work.

## Project Structure
    .
    ├── core                   # Core Library Files
    ├── data                   # example data used for testing
    ├── gjs                    # Development Frontend
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

  #### Notifier
  This provider send's notificatitions when a message/call is received.

  #### SIP Provider
  This provider handles the SIP protocol messages for Nirah.

  #### Streaming Provider
  This provider handles playing audio streams.

## gjs
  This is a frontend i use for playing around with nirah. **It will never be stabilized**. I would like to eventually rewrite this in rust using relm and stabilize that.

## nirah
  This is the actual SoftPhone it runs as a daemon, with a custom RPC interface for monitoring and controlling the softphone.
