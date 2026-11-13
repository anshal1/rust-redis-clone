# Custom TCP Key-Value Store

A simple, lightweight key-value store server implemented in Rust that uses a custom text-based TCP protocol.

## Features

- **In-memory Storage**: Uses a `HashMap` to store key-value pairs.
- **Custom TCP Protocol**: A simple text-based protocol for interacting with the store.
- **Basic CRUD Operations**: Supports adding, retrieving, and deleting keys.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and `cargo` installed on your system.

### Running the Server

1. Clone the repository:

    ```bash
    git clone <repository-url>
    cd custom-tcp-protocol
    ```

2. Run the server using Cargo:
    ```bash
    cargo run
    ```
    The server will start listening for connections on `127.0.0.1:5000`.

## Protocol Specification

The server expects commands as single lines of text.

### Commands

| Command    | Syntax                | Description                                  | Response                        |
| :--------- | :-------------------- | :------------------------------------------- | :------------------------------ |
| **Add**    | `add <key> "<value>"` | Stores the value associated with the key.    | `Success`                       |
| **Get**    | `get <key>`           | Retrieves the value associated with the key. | `<value>` or `None`             |
| **Delete** | `del <key>`           | Removes the key and its associated value.    | `Key removed` or `No key found` |

### Notes on the `add` command:

- The value must be enclosed in double quotes (`"`).
- Example: `add mykey "my value"`

## Usage Examples

You can interact with the server using tools like `nc` (netcat) or `telnet`.

### Using Netcat (`nc`)

**Adding a key:**

```bash
echo 'add username "john_doe"' | nc 127.0.0.1 5000
# Output: Success
```

**Retrieving a key:**

```bash
echo 'get username' | nc 127.0.0.1 5000
# Output: john_doe
```

**Deleting a key:**

```bash
echo 'del username' | nc 127.0.0.1 5000
# Output: Key removed
```

### Using the JS Client

The project includes a TypeScript-based client in the `clients/js` directory.

**Setup:**

1. Navigate to the client directory:
    ```bash
    cd clients/js
    ```
2. Install dependencies:
    ```bash
    npm install
    ```

**Running the client:**
You can run the client in development mode using `tsx`:

```bash
npm run dev
```

Or build and run it:

```bash
npm run build
npm run start
```

**Example Usage in Code:**

```typescript
import { MyHashClient } from "./src/client";

async function main() {
    const client = new MyHashClient("127.0.0.1", 5000);

    try {
        await client.connect();

        // Adding a key
        await client.add("username", "john_doe");
        console.log("User added successfully");

        // Retrieving a key (returns Info<string>)
        const userResponse = await client.get("username");
        console.log(`Retrieved: ${userResponse.message}`);

        // Deleting a key
        await client.delete("username");
        console.log("User deleted successfully");
    } catch (error) {
        console.error(
            "An error occurred while interacting with the server:",
            error,
        );
    } finally {
        // Always disconnect to clean up resources
        await client.disconnect();
    }
}
main();
```

## Progress & Roadmap

- ✅ **Custom HashMap**: Implemented a simple hashmap from scratch to replace the standard library `HashMap`.
- ✅ **JS Client**: Created a JavaScript client to interact with the server.
- ⏳ **Rust Client**: Starting development on a native Rust client soon.

## Project Structure

- `src/main.rs`: Contains the server logic, command parsing, and request handling.
- `Cargo.toml`: Project dependencies and metadata.

## A Note from the Developer

I'm an average developer, and this was my first time implementing so much of a project myself in a low-level programming language. It was challenging, but I loved every step of the process!

I know my implementations might not be the most optimized or "best practice," so I would love to receive feedback from more experienced developers. Thanks for checking out my project!
