## Opcodes

The opcodes of packets.

Version `0.0.1`

## Rule of Thumb

The rule of thumb of creating opcodes to make things consistent:
 - Categorize smaller parts with higher bytes. Like `0x03` is categorized for general things, and `0x12`, `0x18` might belong on the same category (like project retrieving).
 - Have `0x00` as a success response to anything
 - Opcodes ranging `0xff00`-`0xffff` are treated as errors, these are the possible responses of any opcodes.
 - Anything that's too contrasted from its category, it should be placed at the back (`0xf`). For example `0x1f` to open projects (but in the category of projects).

### Category: Authentication `0x01`

Anything related to authentication.

Client:
 - `0x00`: Success Response

 - `0x10`: Login
   Fields:
    - `username`: str
    - `password`: str
   Responses: Server `0x12`, `0x02`

 - `0x11`: Login with token
   Fields:
    - `token`: str
   Responses: Server `0x12`

 - `0x20`: Register
   Fields:
    - `username`: str
    - `password`: str
   Responses: Server `0x01`, `0x03`

 - `0x21`: Check if register is enabled
   Responses: Server `0x21`, `0x00`
 
 - `0xf0`: Check if username exists
   Fields:
    - `username`: str
   Responses: Server `0x00`, `0x02`

 - `0x00ff`: Logout

> todo: create a convention to differentiate between error responses and success responses

Server:
 - `0x00`: Success response
 - `0x10`: Login failed (unknown username/wrong password)
 - `0x11`: Login failed (token expired)
 - `0x12`: Login success
   Fields:
    - `token`: str
 - `0x20`: Register failed (username taken)
 - `0x21`: Register failed (feature disabled)
 - `0xffff`: Error: Already logged in

### Category: User `0x02`

Anything related to user operations, such as listing projects, retrieving settings or metadata.

Client:
 - `0x00`: Success response

 - `0x01`: Get username
   Responses: Server `0x00`

 - `0x10`: Retrieve projects
   Responses: Server `0x01`
 - `0x11`: Retrieve projects paged
   Field:
    - `offset`: u32
    - `count`: u32
   Responses: Server `0x01`
 - `0x12`: Retrieve total projects
   Responses: Server `0x11`
 - `0x13`: Retrieve project image
   Fields:
    - `imgid`: u32
   Responses: Server `0x12`

 - `0x1f`: Open project
   Responses: Server `0x00` (on category editor `0x3`)

Server:
 - `0x00`: Success response

 - `0x01`: Username response
   Fields:
    - `username`: str

 - `0x10`: Projects list response
   Fields:
    - `projects`:
      List of:
       - `id`: u32
       - `title`: u32
       - `lastedit`: u64 (timestamp)
       - `created`: u64 (timestamp)
       - `imgid`: u32 (used on Client `0x13`)
 - `0x11`: Total projects response
   Fields:
    - `total`: u32
 - `0x12`: Project image response
    Fields:
     - `data`: [u8]

 - `0xffff`: Error (not authenticated)

### Category: Editor `0x3`

Anything related to the editor.

Client:
 - `0x00`: Success response
 - `0x01`: Project name
   Responses: Server `0x01`

 - `0x100`: Video Preview
 - `0x200`: Timeline
 - `0x300`: Effects
 - `0x400`: Elements
 - `0x500`: Settings
 - `0xf00`: Special

 - `0x00ff`: Close project
   Responses: `0x00`

Server:
 - `0x00`: Success response
 - `0x01`: Project name response
   Fields:
    - `name`: str

 - `0xffff`: Error: No project opened
