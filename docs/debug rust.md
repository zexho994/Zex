# Debug Rust from vscode

to edit the json:

```json
{
  "name": "(OSX) Launch",
  "type": "lldb",
  "request": "launch",
  "program": "${workspaceRoot}/cmd/target/debug/cmd",
  "args": [],
  "cwd": "${workspaceRoot}"
}
```

program 的路径为可执行文件的路径，参考本项目的配置
