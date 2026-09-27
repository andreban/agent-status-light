# agent-status-light

Physical status lights for AI coding agents.

Connects AI coding agents (**Google Antigravity**, **Anthropic Claude Code**, and **OpenAI Codex**) to physical hardware status lights via USB serial, providing real-time ambient feedback on your desk while agents are thinking, executing tools, waiting for approval, succeeding, or failing.

---

## Visual Status Guide

| Normalized State | MicroPython Reference Animation | Agent Lifecycle Meaning |
| :--- | :--- | :--- |
| **`thinking`** | **Breathing Cyan** (2.0s sine cycle) | Agent processing prompt / reasoning |
| **`executing`** | **Pulsing Purple/Blue** (0.8s pulse) | Agent executing a tool (terminal command, file edit) |
| **`waiting`** | **Double-Blink Amber** (alert flash) | Needs user confirmation or input |
| **`done`** | **Solid Green** (auto-off after 20s) | Task or turn complete; ready for user |
| **`error`** | **Flashing Red** (0.4s warning alert) | Execution failure or command error |
| **`off`** | **All LEDs Off** | Session closed or idle |

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
   agent-status-light --port /dev/cu.usbmodem3301 set thinking
   ```

2. **Option B: Environment Variable (`ASL_PORT`, Recommended)**
   Export `ASL_PORT` in your shell startup file (`~/.zshrc`, `~/.bashrc`, or Windows Environment Variables):
   ```bash
   export ASL_PORT="/dev/cu.usbmodem3301"  # Replace with your serial port
   ```
   When `ASL_PORT` is set, you can omit the `--port` argument completely:
   ```bash
   agent-status-light set thinking
   ```

---

## 4. Manual Testing

Test the physical status light directly from your terminal:

```bash
# Trigger specific states manually (with ASL_PORT exported):
agent-status-light set thinking
agent-status-light set executing
agent-status-light set waiting
agent-status-light set done
agent-status-light set error
agent-status-light set off

# Or with an explicit port flag:
agent-status-light --port /dev/cu.usbmodem3301 set thinking

# Dry-run test to stdout without physical hardware:
agent-status-light --sink stdout set thinking
```

---

## 5. Agent Setup Guides

### 5.1. Google Antigravity

Antigravity executes lifecycle hooks defined in `.agents/hooks.json` in your workspace or globally in `~/.gemini/config/hooks.json`.

1. Copy [`integrations/antigravity/hooks.json`](integrations/antigravity/hooks.json) to `.agents/hooks.json`:
   ```bash
   mkdir -p .agents
   cp integrations/antigravity/hooks.json .agents/hooks.json
   ```

2. Configuration content:
   ```json
   {
     "status-light": {
       "PreInvocation": [
         {
           "type": "command",
           "command": "agent-status-light --event PreInvocation --port /dev/ttyACM0"
         }
       ],
       "PreToolUse": [
         {
           "matcher": "*",
           "hooks": [
             {
               "type": "command",
               "command": "agent-status-light --event PreToolUse --port /dev/ttyACM0"
             }
           ]
         }
       ],
       "PostToolUse": [
         {
           "matcher": "*",
           "hooks": [
             {
               "type": "command",
               "command": "agent-status-light --event PostToolUse --port /dev/ttyACM0"
             }
           ]
         }
       ],
       "PostInvocation": [
         {
           "type": "command",
           "command": "agent-status-light --event PostInvocation --port /dev/ttyACM0"
         }
       ],
       "Stop": [
         {
           "type": "command",
           "command": "agent-status-light --event Stop --port /dev/ttyACM0"
         }
       ]
     }
   }
   ```

3. Replace `/dev/ttyACM0` with your serial port (e.g. `/dev/cu.usbmodem3301` or `COM3`).
   *(If you export `ASL_PORT` in your shell profile, you can simplify the command to `"agent-status-light"`.)*

---

### 5.2. Anthropic Claude Code

Claude Code supports lifecycle hooks configured in your global settings (`~/.claude/settings.json` or `.claude/settings.json` in your project).

1. Add the hook configuration from [`integrations/claude-code/settings.snippet.json`](integrations/claude-code/settings.snippet.json) into your `~/.claude/settings.json`:
   ```json
   {
     "hooks": {
       "UserPromptSubmit": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "PreToolUse": [
         {
           "matcher": "*",
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "PostToolUse": [
         {
           "matcher": "*",
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "Notification": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "Stop": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "SessionEnd": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ]
     }
   }
   ```

2. Replace `/dev/ttyACM0` with your serial port (or set `ASL_PORT` and use `"agent-status-light"`).
3. When Claude Code submits a prompt, runs a tool, prompts for permission, or finishes a turn, `agent-status-light` parses the incoming hook JSON from stdin and updates the light immediately.

---

### 5.3. OpenAI Codex

Codex lifecycle hooks are defined in `hooks.json` in your workspace.

1. Copy [`integrations/codex/hooks.json`](integrations/codex/hooks.json) to your workspace `hooks.json`:
   ```json
   {
     "hooks": {
       "beforeSubmitPrompt": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "tool_execute_before": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "permission_asked": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "stop": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ],
       "turn_end": [
         {
           "command": "agent-status-light --port /dev/ttyACM0"
         }
       ]
     }
   }
   ```

2. Replace `/dev/ttyACM0` with your serial port (or set `ASL_PORT` and use `"agent-status-light"`).

---

## 6. Testing Hook Payloads Manually

You can simulate agent hook payloads from the command line by piping JSON into `agent-status-light`:

```bash
# Simulate Antigravity PreInvocation (Thinking)
echo '{"event":"PreInvocation","conversationId":"test-123"}' | agent-status-light --port /dev/cu.usbmodem3301

# Simulate Antigravity PreToolUse (Executing)
echo '{"event":"PreToolUse","toolCall":{"name":"run_command"}}' | agent-status-light --port /dev/cu.usbmodem3301

# Simulate Claude Code Permission Prompt (Waiting)
echo '{"hook_event_name":"Notification","notification_type":"permission_prompt","session_id":"s-1"}' | agent-status-light --port /dev/cu.usbmodem3301

# Simulate Claude Code Stop (Done)
echo '{"hook_event_name":"Stop","session_id":"s-1"}' | agent-status-light --port /dev/cu.usbmodem3301
```

*(Note: If `ASL_PORT` is exported in your environment, the `--port` argument in the commands above can be omitted.)*

---

## 7. Capturing Hook Payloads for Debugging

If the light doesn't match what the agent is doing, record exactly what each hook sends by setting `ASL_LOG` (or passing `--log <PATH>`):

```bash
export ASL_LOG="$HOME/asl-hooks.jsonl"
```

Every invocation appends one JSON line with the arguments, the raw stdin payload, the resulting state, whether the sink write succeeded, and any error:

```json
{"timestamp_ms":1790000000000,"args":["--event","PreToolUse"],"input":"{\"conversationId\":\"c-1\",...}","state":"executing","sink_ok":true,"error":null}
```

Logging is best-effort: if the file can't be written, the hook still runs normally. Extract the raw payloads with `jq -r .input ~/asl-hooks.jsonl`.

---

## License

[Apache 2.0](LICENSE)