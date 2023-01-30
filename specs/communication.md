How the opcodes interact with each other

### Logging in

```
C -> S: 0x10 { username: str, password: str }

Success:
S -> C: 0x12 { token: str }

Failure:
S -> C: 0x10 (invalid username/password)
```

### Post-login

After logging in, the server will directly send the first 10 projects metadata to the user through opcode `0x10` category `0x2` (projects list response).

The client shall load the images or the project data if needed. Or try to load more projects using the opcode `0x11` in a paged way, `0x10` to load all projects.
