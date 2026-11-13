// NOTE - The comments are for me to understand what I am doing so I will write them in a way that's easier for me to understand
import net from "node:net";

export interface Info<T> {
    message: T;
}

interface PendingRequest {
    resolve: (data: string) => void;
    reject: (error: Error) => void;
}

export class MyHashClient {
    private host: string;
    private port: number;
    private socket: net.Socket | null = null;
    private connecting: Promise<net.Socket> | null = null;

    private buffer = "";
    private pending: PendingRequest[] = [];

    constructor(host: string, port: number) {
        this.host = host;
        this.port = port;
    }

    async connect(): Promise<net.Socket> {
        if (
            this.socket &&
            !this.socket.destroyed &&
            this.socket.writable &&
            !this.socket.closed
        ) {
            return this.socket;
        }

        if (this.connecting) {
            return this.connecting;
        }

        this.connecting = new Promise((resolve, reject) => {
            const socket = net.createConnection({
                host: this.host,
                port: this.port,
            });

            this.socket = socket;
            socket.setEncoding("utf8");

            socket.on("connect", () => {
                console.log("Server connected");
                resolve(socket);
            });

            socket.on("data", (data: string) => {
                this.handleData(data);
            });

            socket.on("error", (err) => {
                this.failAll(err);

                if (this.socket === socket) {
                    this.socket = null;
                }

                reject(err);
            });
            // fires when server shutdowns write/read
            socket.on("end", () => {
                console.log("Server finished sending data");
                // doesn't close the connection right away, just tells the server that I will no longer send any data to you
                socket.end();
            });
            socket.on("close", () => {
                console.log("Connection closed");
                if (this.socket === socket) {
                    this.socket = null;
                }

                this.failAll(new Error("Connection closed"));
            });
        });

        try {
            return await this.connecting;
        } finally {
            this.connecting = null;
        }
    }

    private handleData(data: string): void {
        this.buffer += data;

        let newlineIndex: number;

        while ((newlineIndex = this.buffer.indexOf("\n")) !== -1) {
            const response = this.buffer
                .slice(0, newlineIndex)
                .replace(/\r$/, "");
            // we do not want the new line character in the buffer as it's from the previous iteration
            this.buffer = this.buffer.slice(newlineIndex + 1);

            const request = this.pending.shift();

            if (request) {
                request.resolve(response);
            }
        }
    }

    private failAll(error: Error): void {
        this.buffer = "";

        while (this.pending.length > 0) {
            this.pending.shift()?.reject(error);
        }
    }

    private async request(command: string): Promise<string> {
        const socket = await this.connect();

        return new Promise((resolve, reject) => {
            this.pending.push({ resolve, reject });

            socket.write(`${command}\n`, (err) => {
                if (err) {
                    this.failAll(err);
                }
            });
        });
    }

    // doesn't return anything, throws error if err occured
    async add(key: string, value: string): Promise<void> {
        await this.request(`add ${key} "${value}"`);
    }
    // returns the value if exists otherwise "None", throws error if err occured
    async get(key: string): Promise<Info<string>> {
        const response = await this.request(`get ${key}`);
        return {
            message: response,
        };
    }
    // doesn't return anything, throws error if err occured
    async delete(key: string): Promise<void> {
        await this.request(`del ${key}`);
    }
    // always call disconnect at the end other wise the server or client maybe both crash
    async disconnect(): Promise<void> {
        if (!this.socket || this.socket.destroyed) return;

        const goodbye = await this.request("end");
        console.log(goodbye);

        // Wait for the server's FIN and socket closure.
        await new Promise<void>((resolve) => {
            this.socket?.once("close", () => resolve());
        });
    }
}
