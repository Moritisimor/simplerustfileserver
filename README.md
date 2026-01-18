# SimpleRustFileServer
A simple program for hosting static files written in rust using the actix-web framework.

## What is this project about?
SRFS is a successor to my old SimpleGoFileServer, except that it is written in Rust.

Although a rewrite was not necessary, it did bring some benefits, such as smaller binary sizes and more performance.

## How do I use it?
### Compilation
The only dependency is cargo to build the source code into a binary.

First, you will need to clone the source code:
```bash
git clone https://github.com/Moritisimor/simplerustfileserver
```

Then, cd into the directory:
```bash
cd simplerustfileserver
```

And finally compile the program:
```bash
cargo build -r
```

You can find the compiled program in ```simplerustfileserver/target/release```, it is called ```srfs```

### Usage
To use it you can simply start it with ```srfs``` or ```./srfs``` depending on where it is relative to your current working directory.

By default it...
- listens on ```0.0.0.0:8080```
- serves the current working directory
- binds on ```/```
- uses ```index.html``` as its index file and
- falls back to showing the files as a list if the index file is not present

### Flags

| ```-s```       | ```-d```  | ```-u``` | ```-i```   |
|----------------|-----------|----------|------------|
| Server Address | Directory | URL Path | Index File |

### Examples
Start the server only on localhost on port 7000 using ```hello.html``` as the index file:
```bash
srfs -s localhost:7000 
```

Start the server on all interfaces on port 8000, serving files on the url path ```/files``` and serving files from the directory ```static```
```bash
srfs -s 0.0.0.0:8000 -u /files -d ./static
```
