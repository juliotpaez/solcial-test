# Name-service program analysis

The name-service program allows to define a namespace hierarchy similar to DNS domains. The namespaces can have a
parent, which defines the hierarchy, and also a class that define the type of the namespace, e.g. domain-like, twitter
names, etc.

Once you own a namespace you are the only one that can create sub-namespaces under your own.

- **Address**: `namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX`
- **Owner**: `NAMESwRMGCgyHhZoMZHXz1jNt4VhGUQ69FBiEKPZTSr`. This account is empty, meaning its only purpose is to be the
  owner of the program.
- **Last update**: one month ago

## On-chain data

The program keeps on-chain a set of data structured following the pattern:

- Header:
    - Parent address
    - Owner address
    - Class
- Body: user-defined data

## Features

The program allows a user to:

1. Create a top-level-domain namespace optionally belonging to a class.
2. Create a sub-level namespace. Once created, the owner can also remove it, transfer it to another owner or remove it.
3. Retrieve all direct sub-level namespace belonging to a specific namespace.
4. Inspect the hierarchy of a sub-level namespace until reaching its top-level-domain namespace.

## Potential issues / Attacking vectors

After inspecting the code I have discovered a potential issue when a parent namespace is removed still having children.
It does not cause any security problem but can cause problems when trying to inspect the hierarchy of a sub-level
namespace.

## Business

There are platforms that are using this program to make their own services. An example of this
is [Bonfida](https://bonfida.org) which is the owner of the `.sol` top-level domain
and [sell](https://naming.bonfida.org/#/auctions) the subdomains using in auctions.

# Stats

There are some available stats for `.sol` domains:

- [https://naming.bonfida.org/#/leaderboard]
- [https://solanafloor.com/domains]