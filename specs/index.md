> More about the protocol: [`protocol.md`](protocol.md)

Dalang is separated into two parts: the backend, and the frontend. Each of these two communicates to each other through a single websocket connection. Everything such as login, projects listing, editing, passing rendered results are done on the websocket connection.

Dalang is supposed to be used in a LAN environment where connection between the client and the server has low-latency. Something like a big company having one giant server running dalang and its clients connected over its intranet. Dalang sends the preview video stream right to its clients so the rendering and previewing are all done on the server to reduce the amount of resource usage in its client, and to use the full capability of a server to render the video.

## User flow

A dalang server acts as an instance of dalang, it stores and authenticates users. At start, the user may create or login to an account registered on the dalang server. Account creation may be disabled for users through its configuration files, users can be managed on the server side.

On the first time the user opens dalang's endpoint, they will be greeted with the login page. Since dalang is a single page application, we maintain a single (secure) websocket connetion for the whole session.

At the event of the initiation of the websocket connection, the client will be sent a text package by the server the version of the server using semver. E.

Even though dalang is meant to be a web-based video editor---where the editor itself is being sent to the user by the backend---we want to open up the possiblity of other people creating different clients for dalang. Sending the backend version will make compatibility between versions easier.

If the user has already logged in, 
