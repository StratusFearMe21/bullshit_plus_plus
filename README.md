# Bullshit++

The bullshit generator for the modern web!

## Origin

I thought of Bullshit++ after getting assigned a new unit in Dual Enrollment Advanced Programming. The unit included procedural generation, color theory, and more. I think this project covers all the bases.

## Features

*   Generates static sites for entire bullshit projects
*   Produces details about the bullshit project's creator and features
*   Generates a logo using Unsplash or files locally on your computer.
*   Seeded generation (except for logo if using Unsplash)

## Installation for Unsplash

Release builds of this app contain an Unsplash API key by default (demo key).
To build with Unsplash API support, create a `.env` file at the root of the project and put these contents in the file

```env
UNSPLASH_API_KEY=[API Key]
```

Now you can just build as you normally would

```shell
cargo build --release
```
