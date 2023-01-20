> More about the protocol: [`protocol.md`](protocol.md)

Dalang is separated into two parts: the backend, and the frontend. Each of these two communicates to each other through a single websocket connection. Everything such as login, projects listing, editing, passing rendered results are done on the websocket connection.

Dalang is supposed to be used in a LAN environment where connection between the client and the server has low-latency. Something like a big company having one giant server running dalang and its clients connected over its intranet. Dalang sends the preview video stream right to its clients so the rendering and previewing are all done on the server to reduce the amount of resource usage in its client, and to use the full capability of a server to render the video.

## User flow

A dalang server acts as an instance of dalang, it stores and authenticates users. At start, the user may create or login to an account registered on the dalang server. Account creation may be disabled for users through its configuration files, users can be managed on the server side.

On the first time the user opens dalang's endpoint, they will be greeted with the login page. Since dalang is a single page application, we maintain a single (secure) websocket connetion for the whole session.

At the event of the initiation of the websocket connection, the client will be sent a text package by the server the version of the server using semver. E.

Even though dalang is meant to be a web-based video editor---where the editor itself is being sent to the user by the backend---we want to open up the possiblity of other people creating different clients for dalang. Sending the backend version will make compatibility between versions easier.

### Authentication

As the user opens dalang's interface, the interface must display a login or a register prompt (if enabled, can be checked to the packet `0x21` in the category of `0x1`).

If in the case that the user is already logged in, the client must use the token that was sent by the server after a successful login.

### User "space"

After logging into the account, the user enters a "userspace", a place where they view or edit their user data such as listing projects, viewing profile, uploading user-wide data, or opening projects.

### Project Editing

At the event the user opens a project, the server remembers the project that's opened on the websocket connection. The client will be able to use opcodes in the project category to edit the project such as doing timeline editing, adding effects, uploading files, exporting project, editing project settings, and control the preview.

The video preview of the project editing is the most interesting part of dalang. The preview video being shown is basically a video stream directly from the server, rendered in real-time using server's hardware and reacting to changes in the timeline. For in-video editing like transforming a text element, the handle overlay will be shown in the client side over the streamed video, and the client will send generic updates to the effect data of the element being edited.

#### Elements

In dalang, anything that can be placed in a timeline is called an "Element". An element is a generative video stream. Such as a regular plain old video, a procedurally-generating noise, or anything that can output a video stream.

There's also something called as "objects", which inherits Element (able to output a video stream). But rather than a dynamic data---where pretty much every frame it needs to be "recalculated"---it is static, in a way where it wouldn't change when the input data wasn't changed. Think of text, a shape, an image, or anything else that is static throughout the whole timeline. In dalang's internal implementation, objects gets transformed into a video stream as it gets treated as an Element, but rather than taking long calculations on each frame, it returns the same frame for every frame.

Objects are planned to be a some kind of a container of "objects". Think of a single object having multiple elements like a picture, a text all flattened in the single frame. It will give out really big performance gains as MLT (the video framework used by dalang's server) wouldn't have to process a whole other track that only contains an object with another object beneath.
