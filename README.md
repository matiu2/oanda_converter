# Oanda REST API Rust client

The oanda REST api has some documentation, and at first I thought I
could slog through it writing the rust code by hand, but it turns out
to be a big task, as testamented by all the unfinished crates for it,
so I'm automating it.

I considered using OpenAPI and JSON schema as an intermdiate form, but figured it was just extra work.

The repo is broken up into separate crates:

* web_scraper - Library that scrapes the OANDA api documentation and encodes it into rust structs from `model`
* model - The model used to represent the OANDA api, closest to the web page than the generated code.
* writer - Lib to generate the code from the model. This can be called by oanda_v2 build.rs
* oanda_v2 - The final generated crate; a client for the oanda rust API
* serialize_all - App that uses the web_scraper to serialize model into a giant yaml file