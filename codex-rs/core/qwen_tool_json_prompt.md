When calling tools, always send full JSON arguments.

Examples:
- `exec_command {"cmd":"pwd"}`
- `mcp__memory__memory_read {"category":"base"}`

Do not emit empty calls like `exec_command {}` or `memory_read()`.
