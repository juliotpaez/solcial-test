# dTwitter

This example shows a decentralized twitter-like app.

There are two nodes:

- **Clients**: represent users and their interaction through a simple command CLI.
- **Beacons**: auxiliary nodes to keep client information when they disconnect. These nodes have no CLI.

## Client CLI

The available commands are:

- `GET`: returns current client's messages.
- `GET <peer_id>`: returns peer_id client's messages.
- `POST <text>`: adds a message to the current client and its associated beacon if any.
- `LINK_BEACON <peer_id>`: links this client to a beacon unlinking to the previous one if any. An error will be thrown
  if `peer_id` identifies a client.
- `UNLINK_BEACON`: unlinks the client from its associated beacon if any.

## Example execution

1. Run `cargo run` in `/client` to launch the first client.
2. Run `cargo run` in `/client` to launch the second client.
3. Type `POST A` in the first client to add a new message in it.
4. Type `GET` in the first client to see the message you have just inserted.
5. Type `GET <first_client_peer_id>` in the second client to see the message of the first client.
6. Run `cargo run` in `/beacon` to launch a beacon.
7. Type `LINK_BEACON <beacon_peer_id>` in the first client to link it with the beacon.
8. Type `POST B` in the first client to add another message in the client and beacon.
9. Type `GET <first_client_peer_id>` in the second client to see the messages of the first client. This time it is
   answered by the beacon.
10. Close the fist client.
11. Type `GET <first_client_peer_id>` in the second client to see the messages of the first client, answered again by
    the beacon.

> You can find the `peer_id`s in the first log.