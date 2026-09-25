"""Host-side unit tests for Pimoroni Tiny 2040 status light logic."""

import json
import unittest


class MockLED:
    def __init__(self):
        self.r = 0.0
        self.g = 0.0
        self.b = 0.0
        self.is_off = True

    def set_rgb(self, r, g, b):
        self.r = r
        self.g = g
        self.b = b
        self.is_off = False

    def off(self):
        self.r = 0.0
        self.g = 0.0
        self.b = 0.0
        self.is_off = True


class MockStatusLightApp:
    def __init__(self, led):
        self.led = led
        self.state = "off"
        self.state_start_ms = 0
        self.current_time_ms = 0

    def set_state(self, new_state):
        if self.state != new_state:
            self.state = new_state
            self.state_start_ms = self.current_time_ms

    def update(self):
        elapsed = self.current_time_ms - self.state_start_ms

        if self.state == "thinking":
            self.led.set_rgb(0.0, 0.8, 1.0)
        elif self.state == "executing":
            self.led.set_rgb(0.6, 0.0, 1.0)
        elif self.state == "waiting":
            cycle = elapsed % 1500
            if cycle < 100 or (250 <= cycle < 350):
                self.led.set_rgb(1.0, 0.4, 0.0)
            else:
                self.led.off()
        elif self.state == "done":
            if elapsed >= 20_000:
                self.state = "off"
                self.led.off()
            else:
                self.led.set_rgb(0.0, 1.0, 0.0)
        elif self.state == "error":
            cycle = elapsed % 400
            if cycle < 200:
                self.led.set_rgb(1.0, 0.0, 0.0)
            else:
                self.led.off()
        else:
            self.led.off()


class TestFirmwareLogic(unittest.TestCase):
    def test_json_payload_parsing(self):
        valid_json = '{"state":"thinking","agent":"antigravity","session_id":"test-1"}'
        data = json.loads(valid_json)
        self.assertEqual(data.get("state"), "thinking")

    def test_state_transitions(self):
        led = MockLED()
        app = MockStatusLightApp(led)

        app.set_state("thinking")
        app.update()
        self.assertEqual(app.state, "thinking")
        self.assertFalse(led.is_off)

        app.set_state("executing")
        app.update()
        self.assertEqual(app.state, "executing")

        app.set_state("off")
        app.update()
        self.assertEqual(app.state, "off")
        self.assertTrue(led.is_off)

    def test_done_auto_off(self):
        led = MockLED()
        app = MockStatusLightApp(led)

        app.set_state("done")
        app.current_time_ms = 0
        app.update()
        self.assertEqual(app.state, "done")
        self.assertFalse(led.is_off)

        # Before 20s
        app.current_time_ms = 19_000
        app.update()
        self.assertEqual(app.state, "done")

        # After 20s
        app.current_time_ms = 20_001
        app.update()
        self.assertEqual(app.state, "off")
        self.assertTrue(led.is_off)

    def test_waiting_double_blink_timing(self):
        led = MockLED()
        app = MockStatusLightApp(led)

        app.set_state("waiting")

        # 50ms -> First blink (ON)
        app.current_time_ms = 50
        app.update()
        self.assertFalse(led.is_off)

        # 150ms -> Gap between blinks (OFF)
        app.current_time_ms = 150
        app.update()
        self.assertTrue(led.is_off)

        # 300ms -> Second blink (ON)
        app.current_time_ms = 300
        app.update()
        self.assertFalse(led.is_off)

        # 500ms -> Rest of cycle (OFF)
        app.current_time_ms = 500
        app.update()
        self.assertTrue(led.is_off)


if __name__ == "__main__":
    unittest.main()
