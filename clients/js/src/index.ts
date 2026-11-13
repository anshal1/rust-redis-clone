import { MyHashClient } from "./client";
// const my_hash_server = new MyHashClient("127.0.0.1", 5000);

async function main() {
    for (let i = 0; i < 100; i++) {
        const client = new MyHashClient("127.0.0.1", 5000);

        await client.connect();

        await client.add(i.toString(), "Anshal" + i);

        const name = await client.get("name");
        console.log(name.message === "Anshal");

        await client.add("age", "24");

        console.log(await client.get("age"));

        await client.disconnect();
    }
}
main();
