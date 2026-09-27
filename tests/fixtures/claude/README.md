# Claude Code hook fixtures

Real Claude Code hook payloads captured with `ASL_LOG` (2026-09-27, in `auto` and then `default` permission mode). To keep them small and free of conversation content:

- the username is replaced with a placeholder;
- `prompt` and `last_assistant_message` are replaced with placeholder text;
- strings longer than 160 characters are cut and end with `…`.

Unlike Antigravity, every payload names its own event in `hook_event_name`.

- `*.json`: one stdin payload per event, tool and notification type.
- `parallel_tool_calls.jsonl`: two Bash calls issued in one response, as `{"event", "t_ms", "sink_ok", "payload"}` lines.
- `permission_prompt_and_question.jsonl` (same format, `default` permission mode):
  - an approved Bash command;
  - an answered `AskUserQuestion`;
  - an approved `Read` that then failed.
- `interrupted_at_permission_prompt.jsonl`: the user pressed Esc while a permission prompt was showing.

## Observations

- **`PermissionRequest` is the timely "waiting" signal.** It fires about 150 ms after `PreToolUse`, when the prompt appears. `Notification` with `permission_prompt` arrives about 6 s later. `AskUserQuestion` also goes through `PermissionRequest`, so one mapping covers both approvals and questions. PostToolUse fires once the user approves or answers.
- **An interrupt fires no hook.** After Esc at a permission prompt there was no PostToolUse, `PostToolUseFailure` or `Stop`; the next event was the user's next `UserPromptSubmit`. The light stays `waiting` until then.
- **Read-only commands (e.g. `echo`) skip `PermissionRequest`**, even in `default` mode.
- **`SessionEnd` carries a `reason`.** `prompt_input_exit` came from `/exit`. `resume` came from a fresh session that ended when the user resumed an older one, so it fired for a session that did nothing. `SessionEnd` from any session turns the light off, even if another session is busy.
- **Parallel tool calls run their hooks concurrently.** The two PreToolUse hooks started 5 ms apart. The second failed to open the serial port ("Access is denied" on Windows), so that update was dropped (`"sink_ok": false`).
- **Tool completion order isn't call order.** When the short call's PostToolUse arrived first, the light went back to `thinking` while the other tool was still running.
- **Routine command failures fire `PostToolUseFailure`.** `error` holds the exit code and stderr (`"Exit code 1 …"`), and `is_interrupt` is `false`. Mapping this to the `error` state flashes red for ordinary failures such as a script typo.
- **`Notification` with `idle_prompt`** arrives about 60 s after `Stop` while the prompt sits idle ("Claude is waiting for your input"). It's redundant after `Stop`.
- **`Stop` includes `background_tasks` and `session_crons`.** A non-empty `background_tasks` may mean the session isn't really idle.
- PreToolUse/PostToolUse pairs share a `tool_use_id`, so overlapping calls can be matched up.

Not yet captured:
- a denied permission prompt;
- an interrupt while a tool is running, which may produce `PostToolUseFailure` with `is_interrupt: true`;
- subagent events.
