# agent-status-light

Physical status lights for AI coding agents.

Connects AI coding agents (**Google Antigravity**, **Anthropic Claude Code**, and **OpenAI Codex**) to physical hardware status lights via USB serial, providing real-time ambient feedback on your desk showing whether an agent is working, waiting for you, or finished.

---

## Visual Status Guide

| State | Light | Meaning |
| :--- | :--- | :--- |
| **`working`** | **Blue**, slow breathe | The agent is busy (reasoning or running tools) |
| **`waiting`** | **Amber**, double-blink | The agent needs you: a permission prompt or a question |
| **`done`** | **Green**, solid | The turn is finished; it's your turn |
| **`off`** | Dark | The session ended |

Some events can't be seen through hooks, so the light may lag in these cases:
- Antigravity shows `working` during tool approval prompts.
- Cancelling a turn or denying a prompt in Antigravity or Claude Code leaves the previous state until your next prompt.

---

## 1. Hardware Setup (Pimoroni Tiny 2040)

The reference firmware targets the **Pimoroni Tiny 2040** (Raspberry Pi RP2040) using its onboard active-low RGB LED on GP18 (Red), GP19 (Green), and GP20 (Blue).

### Flashing MicroPython & Firmware
1. Hold down the **BOOT** button on the Tiny 2040 and plug it into your computer via USB-C.
2. Drag and drop the RP2040 MicroPython UF2 file onto the mounted `RPI-RP2` drive.
3. Install `firmware/tiny2040/main.py` onto the board:
   ```bash
   # Using mpremote (recommended)
   pip install mpremote
   mpremote cp firmware/tiny2040/main.py :main.py
   mpremote soft-reset
   ```
   *(Alternatively, open `firmware/tiny2040/main.py` in Thonny and save it to the MicroPython device as `main.py`.)*

For detailed hardware specifications, see [`firmware/tiny2040/README.md`](firmware/tiny2040/README.md).

---

## 2. Building & Installing the CLI

Build and install `agent-status-light` locally using Cargo:

```bash
# Clone the repository
git clone https://github.com/andreban/agent-status-light.git
cd agent-status-light

# Build and install binary into ~/.cargo/bin
cargo install --path .
```

Verify that `agent-status-light` is on your `$PATH`:
```bash
agent-status-light --version
```

---

## 3. Configuring the Serial Port

### Find Your Device's Port
Plug in your microcontroller and identify the serial port:

- **macOS**:
  ```bash
  ls /dev/cu.usbmodem*
  # Example: /dev/cu.usbmodem3301
  ```
- **Linux**:
  ```bash
  ls /dev/ttyACM* /dev/ttyUSB*
  # Example: /dev/ttyACM0
  # Note: Ensure your user is in the dialout/uucp group: sudo usermod -aG dialout $USER
  ```
- **Windows**:
  Check Device Manager under **Ports (COM & LPT)** (e.g. `COM3` or `COM4`).

### Two Ways to Configure the Port

1. **Option A: Explicit CLI Flag (`--port`)**
   Specify `--port <PATH>` directly in your command or agent hook configuration:
   ```bash
   agent-status-light --port /dev/cu.usbmodem3301 set working
   ```

2. **Option B: Environment Variable (`ASL_PORT`, Recommended)**
   Export `ASL_PORT` in your shell startup file (`~/.zshrc`, `~/.bashrc`, or Windows Environment Variables):
   ```bash
   export ASL_PORT="/dev/cu.usbmodem3301"  # Replace with your serial port
   ```
   When `ASL_PORT` is set, you can omit the `--port` argument completely:
   ```bash
   agent-status-light set working
   ```

---

## 4. Manual Testing

Test the physical status light directly from your terminal:

```bash
# Trigger specific states manually (with ASL_PORT exported):
agent-status-light set working
agent-status-light set waiting
agent-status-light set done
agent-status-light set off

# Or with an explicit port flag:
agent-status-light --port /dev/cu.usbmodem3301 set working

# Dry-run test to stdout without physical hardware:
agent-status-light --sink stdout set working
```

---

## 5. Agent Setup Guides

Each agent has a ready-made hook configuration in [`integrations/`](integrations/). Every command passes `--agent`, and replace `/dev/ttyACM0` with your serial port (e.g. `COM3`). If you export `ASL_PORT`, you can drop `--port` entirely.

Hooks never fail or block the agent: `agent-status-light` always exits 0 in hook mode, and only prints the reply Antigravity expects.

### 5.1. Google Antigravity

Copy [`integrations/antigravity/hooks.json`](integrations/antigravity/hooks.json) to `~/.gemini/config/hooks.json` (global) or `.agents/hooks.json` (one workspace).

Antigravity payloads don't name their event, so each command also passes `--event`. `PostInvocation` is deliberately left out: it fires after every model call, not at the end of a turn.

### 5.2. Anthropic Claude Code

Merge the `hooks` object from [`integrations/claude-code/settings.snippet.json`](integrations/claude-code/settings.snippet.json) into `~/.claude/settings.json` (global) or `.claude/settings.json` (one project). If you already have hooks for these events, add the entries alongside them.

`PermissionRequest` drives the amber light for both permission prompts and questions.

### 5.3. OpenAI Codex

Copy [`integrations/codex/hooks.json`](integrations/codex/hooks.json) to `~/.codex/hooks.json` (global) or `<repo>/.codex/hooks.json`. Codex hooks use the same format as Claude Code.

Codex fires `Interrupt` when you deny a prompt or press Esc, so the light turns green immediately.

---

## 6. Testing Hook Payloads Manually

Pipe a hook payload into `agent-status-light` to simulate an agent (with `ASL_PORT` exported, or add `--port`):

```bash
# Antigravity PreToolUse (working); Antigravity needs --event
echo '{"conversationId":"test-123","toolCall":{"name":"run_command"}}' | agent-status-light --agent antigravity --event PreToolUse

# Claude Code permission prompt (waiting)
echo '{"hook_event_name":"PermissionRequest","session_id":"s-1"}' | agent-status-light --agent claude

# Codex Stop (done)
echo '{"hook_event_name":"Stop","session_id":"s-1"}' | agent-status-light --agent codex
```

Real captured payloads for every agent are in [`tests/fixtures/`](tests/fixtures/).

---

## 7. Capturing Hook Payloads for Debugging

If the light doesn't match what the agent is doing, record exactly what each hook sends by setting `ASL_LOG` (or passing `--log <PATH>`):

```bash
export ASL_LOG="$HOME/asl-hooks.jsonl"
```

Every invocation appends one JSON line with the arguments, the raw stdin payload, the resulting state, whether the sink write succeeded, and any error:

```json
{"timestamp_ms":1790000000000,"args":["--agent","antigravity","--event","PreToolUse"],"input":"{\"conversationId\":\"c-1\",...}","state":"working","sink_ok":true,"error":null}
```

Logging is best-effort: if the file can't be written, the hook still runs normally. Extract the raw payloads with `jq -r .input ~/asl-hooks.jsonl`.

---

## License

[Apache 2.0](LICENSE)