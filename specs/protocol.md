The protocol used by dalang uses a packet-based approach to communicate data between the server and the client.

A packet is an [msgpack](https://msgpack.org) array with two items:
 - `0` (u32): The opcode of the packet
 - `1` (any): The payload for this packet, can be anything depending on the opcode

### Opcode

Opcodes for server and clients are not shared by each other (one opcode sent by the server may not mean the same thing as the one sent from the client).

The opcode is a 32-bit number, which are divided into two 16-bit numbers. The first being the actual opcode, the second being the category of the opcode.

```
11010000000000000100000000000000 - 32-bit num

11010000000000000100000000000000 - 2x 16-bit num
└───────┬──────┘└──────┬───────┘
     opcode         category

opcode: 1101000000000000
category: 0100000000000000
```

A category is the category of an opcode. For example, the category `0x00` might define that the opcode is an authentication opcode, then the "opcode value" is what's processed.

The first 4096 categories (0-4095 inclusive) are reserved for use for dalang. The many other categories can be used to create extensions for custom implementors of dalang, or plugins (if it will exist in the future).

Different opcodes for both the client and the server are defined in the [`opcodes.md`](opcodes.md) file.
