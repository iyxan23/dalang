// All the opcodes used in dalang

// Auth Category =========
export const CATEGORY_AUTH = 0x1;

// Client opcodes
export const C_OPCODE_AUTH_SUCCESS = 0x00;
export const C_OPCODE_AUTH_LOGIN = 0x10; // data: { username: str, password: str }
export const C_OPCODE_AUTH_LOGIN_WITH_TOKEN = 0x11; // data: { token: str }
export const C_OPCODE_AUTH_REGISTER = 0x20; // data: { username: str, password: str }
export const C_OPCODE_AUTH_CHECK_REGISTER_ENABLED = 0x21;
export const C_OPCODE_AUTH_CHECK_USERNAME_EXIST = 0xf0; // data: { username: str }
export const C_OPCODE_AUTH_LOGOUT = 0xff;

// Server opcodes
export const S_OPCODE_AUTH_SUCCESS = 0x00;
export const S_OPCODE_AUTH_LOGIN_FAILED_INVALID = 0x10; // invalid username/password
export const S_OPCODE_AUTH_LOGIN_FAILED_TOK_EXPIRED = 0x11;
export const S_OPCODE_AUTH_LOGIN_SUCCESS = 0x12; // data: { token: str }
export const S_OPCODE_AUTH_REGISTER_FAILED_USERNAME_TAKEN = 0x20;
export const S_OPCODE_AUTH_REGISTER_FAILED_DISABLED = 0x21; // registering is disabled
export const S_OPCODE_AUTH_ERR_ALREADY_LOGGED_IN = 0xffff;

// User Category ==========
export const CATEGORY_USER = 0x2;

// Client opcodes
export const C_OPCODE_USER_SUCCESS = 0x00;
export const C_OPCODE_USER_GET_USERNAME = 0x01;
export const C_OPCODE_USER_PROJECTS_RETRIEVE = 0x10;
export const C_OPCODE_USER_PROJECTS_RETRIEVE_PAGED = 0x11;
export const C_OPCODE_USER_PROJECTS_RETRIEVE_TOTAL = 0x12;
export const C_OPCODE_USER_PROJECTS_RETRIEVE_IMAGE = 0x13; // data: { imgid: u32 }
export const C_OPCODE_USER_PROJECT_OPEN = 0x1f;

// Server opcodes
export const S_OPCODE_USER_SUCCESS = 0x00;
export const S_OPCODE_USER_USERNAME_RESPONSE = 0x01; // data: { username: str }
export const S_OPCODE_USER_PROJECTS_RESPONSE = 0x10; // data: { projects: [{ id: u32, title: str, lastedit: u64, created: u64, imgid: u32 }] }
export const S_OPCODE_USER_PROJECTS_TOTAL_RESPONSE = 0x11; // data: { total: u32 }
export const S_OPCODE_USER_PROJECTS_IMAGE_RESPONSE = 0x12; // data: { total: u32 }
export const S_OPCODE_USER_ERR_NOT_AUTHENTICATED = 0xffff;

// Editor Category ==========
export const CATEGORY_EDITOR = 0x3;

// Client opcodes
export const C_OPCODE_EDITOR_SUCCESS = 0x00;
export const C_OPCODE_EDITOR_CLOSE_PROJECT = 0xff;

// Server opcodes
export const S_OPCODE_EDITOR_SUCCESS = 0x00;
