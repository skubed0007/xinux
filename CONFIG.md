# Xinux Configuration File

The configuration file for Xinux is stored at:

```
~/.config/.xinuxos/config.toml
```

## Available Options

### `prompt_style`
Defines the style of the shell prompt.

**Example Values**:
- `"single_line"`
- `"two_line"`
- `"bold_frame"`
- `"classic"`
- `"arrowed"`

---

### `aliases`
A map of alias names to their corresponding commands.

**Example**:
```toml
[aliases]
ls = "ls -la"
g = "git"
h = "history"
```

---

### `autostart_commands`
A list of commands that will automatically run when Xinux starts.

**Example**:
```toml
autostart_commands = [
    "echo 'Welcome to Xinux!'",
    "ls",
    "cd ~/Documents"
]
```

---

## Example Configuration File

Here is an example configuration file:

```toml
# The style of the shell prompt
prompt_style = "bold_frame"

# Aliases for commands
[aliases]
ls = "ls -la"
g = "git"
h = "history"

# Commands to run automatically when Xinux starts
autostart_commands = [
    "echo 'Welcome to Xinux!'",
    "ls",
    "cd ~/Documents"
]
```
