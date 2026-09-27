"""
Agent status light state machine and animations, independent of hardware.

Runs on MicroPython and on desktop CPython, so the tests exercise the same code
that runs on the board.

States:
    - working : blue, slow breathe (agent busy)
    - waiting : amber, double-blink (agent needs the user)
    - done    : green, solid (turn finished)
    - off     : dark (session ended, or unknown state)
"""

import json
import math

WORKING = "working"
WAITING = "waiting"
DONE = "done"
OFF = "off"
STATES = (WORKING, WAITING, DONE, OFF)

# Colours as normalized (r, g, b) levels in [0.0, 1.0].
BLUE = (0.0, 0.0, 1.0)
AMBER = (1.0, 0.4, 0.0)
GREEN = (0.0, 1.0, 0.0)
DARK = (0.0, 0.0, 0.0)

# Breathe period for `working`, in milliseconds.
BREATHE_PERIOD_MS = 2500
# Lowest breathe brightness, so `working` never looks like `off`.
BREATHE_MIN = 0.15

# Double-blink cycle for `waiting`, in milliseconds, and the windows in it where the LED is on.
BLINK_PERIOD_MS = 1500
BLINK_ON_WINDOWS_MS = ((0, 100), (250, 350))


def parse_state(line):
    """Returns the state from a JSON status line, or None if the line is invalid."""
    try:
        data = json.loads(line)
    except ValueError:
        return None
    if not isinstance(data, dict):
        return None
    state = data.get("state")
    return state.lower() if isinstance(state, str) else None


def _scale(colour, level):
    return (colour[0] * level, colour[1] * level, colour[2] * level)


class StatusLight:
    """Tracks the current state and computes the colour to show at a given time."""

    def __init__(self, now_ms=0, ticks_diff=None):
        # MicroPython tick counters wrap around, so the board passes `time.ticks_diff`.
        self._ticks_diff = ticks_diff or (lambda a, b: a - b)
        self.state = OFF
        self._since_ms = now_ms

    def set_state(self, state, now_ms):
        """Switches to `state`. Unknown states turn the light off. Repeating the current
        state doesn't restart its animation."""
        if state not in STATES:
            state = OFF
        if state != self.state:
            self.state = state
            self._since_ms = now_ms

    def colour(self, now_ms):
        """Returns the (r, g, b) levels to show at `now_ms`."""
        elapsed = self._ticks_diff(now_ms, self._since_ms)
        if self.state == WORKING:
            # Starts at full brightness, so entering `working` never begins from dark.
            wave = 0.5 + 0.5 * math.cos(2.0 * math.pi * (elapsed % BREATHE_PERIOD_MS) / BREATHE_PERIOD_MS)
            return _scale(BLUE, BREATHE_MIN + (1.0 - BREATHE_MIN) * wave)
        if self.state == WAITING:
            cycle = elapsed % BLINK_PERIOD_MS
            for start, end in BLINK_ON_WINDOWS_MS:
                if start <= cycle < end:
                    return AMBER
            return DARK
        if self.state == DONE:
            return GREEN
        return DARK
