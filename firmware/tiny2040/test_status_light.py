"""Host-side tests for status_light.py, the firmware's state and animation logic.

Run with: python firmware/tiny2040/test_status_light.py
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from status_light import (  # noqa: E402
    AMBER,
    BLINK_PERIOD_MS,
    BLUE,
    BREATHE_MIN,
    BREATHE_PERIOD_MS,
    DARK,
    DONE,
    GREEN,
    MAX_LINE_LENGTH,
    OFF,
    STALE_AFTER_MS,
    WAITING,
    WORKING,
    LineBuffer,
    StatusLight,
    parse_state,
)


def brightness(colour):
    return max(colour)


class TestParseState(unittest.TestCase):
    def test_valid_payload(self):
        line = '{"state":"working","agent":"claude","session_id":"s1"}'
        self.assertEqual(parse_state(line), WORKING)

    def test_state_is_lowercased(self):
        self.assertEqual(parse_state('{"state":"DONE"}'), DONE)

    def test_invalid_lines(self):
        for line in ("not json", '{"state":', "[1, 2]", '"working"', '{"agent":"claude"}', '{"state":3}'):
            self.assertIsNone(parse_state(line), line)


class TestStatusLight(unittest.TestCase):
    def test_starts_off(self):
        light = StatusLight()
        self.assertEqual(light.state, OFF)
        self.assertEqual(light.colour(0), DARK)

    def test_unknown_and_removed_states_turn_off(self):
        light = StatusLight()
        for state in ("thinking", "executing", "error", "bogus"):
            light.set_state(DONE, 0)
            light.set_state(state, 0)
            self.assertEqual(light.state, OFF, state)
            self.assertEqual(light.colour(0), DARK, state)

    def test_working_starts_at_full_brightness(self):
        light = StatusLight()
        light.set_state(WORKING, 1_000)
        self.assertEqual(light.colour(1_000), BLUE)

    def test_working_breathes_blue_and_never_goes_dark(self):
        light = StatusLight()
        light.set_state(WORKING, 0)
        for t in range(0, 2 * BREATHE_PERIOD_MS, 50):
            r, g, b = light.colour(t)
            self.assertEqual((r, g), (0.0, 0.0))
            self.assertGreaterEqual(b, BREATHE_MIN - 1e-9)
            self.assertLessEqual(b, 1.0 + 1e-9)
        self.assertAlmostEqual(brightness(light.colour(BREATHE_PERIOD_MS // 2)), BREATHE_MIN)

    def test_repeated_state_does_not_restart_animation(self):
        # Every tool call re-sends `working`; that must not reset the breathe (#16).
        light = StatusLight()
        light.set_state(WORKING, 0)
        mid = BREATHE_PERIOD_MS // 2
        before = light.colour(mid)
        light.set_state(WORKING, mid)
        self.assertEqual(light.colour(mid), before)

    def test_waiting_double_blinks_amber(self):
        light = StatusLight()
        light.set_state(WAITING, 0)
        expected = {50: AMBER, 150: DARK, 300: AMBER, 500: DARK, BLINK_PERIOD_MS + 50: AMBER}
        for t, colour in expected.items():
            self.assertEqual(light.colour(t), colour, t)

    def test_done_stays_green(self):
        light = StatusLight()
        light.set_state(DONE, 0)
        self.assertEqual(light.colour(0), GREEN)
        self.assertEqual(light.colour(3_600_000), GREEN)

    def test_off_is_dark(self):
        light = StatusLight()
        light.set_state(WORKING, 0)
        light.set_state(OFF, 10)
        self.assertEqual(light.colour(10), DARK)

    def test_uses_ticks_diff_for_wraparound(self):
        # MicroPython tick counters wrap; elapsed time must come from the supplied ticks_diff.
        period = 1 << 30
        light = StatusLight(ticks_diff=lambda a, b: (a - b) % period)
        light.set_state(WAITING, period - 20)
        self.assertEqual(light.colour(30), AMBER)  # 50 ms after entering, across the wrap
        self.assertEqual(light.colour(130), DARK)  # 150 ms after entering


class TestStaleFallback(unittest.TestCase):
    def test_working_and_waiting_become_done_after_silence(self):
        for state in (WORKING, WAITING):
            light = StatusLight()
            light.set_state(state, 0)
            light.colour(STALE_AFTER_MS - 1)
            self.assertEqual(light.state, state, state)
            self.assertEqual(light.colour(STALE_AFTER_MS), GREEN, state)
            self.assertEqual(light.state, DONE, state)

    def test_any_message_resets_the_clock(self):
        # A repeated `working` (every tool call) counts as activity.
        light = StatusLight()
        light.set_state(WORKING, 0)
        light.set_state(WORKING, STALE_AFTER_MS - 1)
        light.colour(STALE_AFTER_MS + 1)
        self.assertEqual(light.state, WORKING)
        light.colour(2 * STALE_AFTER_MS - 1)
        self.assertEqual(light.state, DONE)

    def test_done_and_off_never_expire(self):
        for state in (DONE, OFF):
            light = StatusLight()
            light.set_state(state, 0)
            light.colour(10 * STALE_AFTER_MS)
            self.assertEqual(light.state, state)

    def test_new_message_after_fallback(self):
        light = StatusLight(stale_after_ms=1_000)
        light.set_state(WAITING, 0)
        light.colour(1_000)
        self.assertEqual(light.state, DONE)
        light.set_state(WORKING, 1_500)
        self.assertEqual(light.colour(1_500), BLUE)

    def test_stale_timer_across_tick_wraparound(self):
        period = 1 << 30
        light = StatusLight(ticks_diff=lambda a, b: (a - b) % period, stale_after_ms=1_000)
        light.set_state(WORKING, period - 500)
        light.colour(400)  # 900 ms later, across the wrap
        self.assertEqual(light.state, WORKING)
        light.colour(500)  # 1000 ms later
        self.assertEqual(light.state, DONE)


class TestLineBuffer(unittest.TestCase):
    def feed(self, buffer, text):
        return [line for line in (buffer.feed(c) for c in text) if line is not None]

    def test_splits_lines_and_skips_blank_ones(self):
        buffer = LineBuffer()
        lines = self.feed(buffer, '{"a":1}\r\n\n  {"b":2}\n')
        self.assertEqual(lines, ['{"a":1}', '{"b":2}'])

    def test_partial_line_waits_for_line_break(self):
        buffer = LineBuffer()
        self.assertEqual(self.feed(buffer, '{"state":'), [])
        self.assertEqual(self.feed(buffer, '"done"}\n'), ['{"state":"done"}'])

    def test_overlong_line_is_discarded_whole(self):
        # Without discarding the whole line, its tail would be read as a separate line.
        buffer = LineBuffer()
        overlong = "x" * (MAX_LINE_LENGTH + 5) + '{"state":"working"}'
        lines = self.feed(buffer, overlong + '\n{"state":"done"}\n')
        self.assertEqual(lines, ['{"state":"done"}'])

    def test_line_at_max_length_is_kept(self):
        buffer = LineBuffer()
        line = "y" * MAX_LINE_LENGTH
        self.assertEqual(self.feed(buffer, line + "\n"), [line])


if __name__ == "__main__":
    unittest.main()
