"""
Pimoroni Tiny 2040 Reference Firmware for agent-status-light.

Connects the Pimoroni Tiny 2040 onboard active-low RGB LED
(GP18 Red, GP19 Green, GP20 Blue) to incoming host status updates
over USB CDC serial (115200 baud, newline-delimited JSON).

Wire Protocol:
    {"state":"thinking","agent":"antigravity","session_id":"..."}\\n

Supported States:
    - thinking  : Breathing cyan
    - executing : Pulsing purple/blue
    - waiting   : Double-blink amber alert
    - done      : Solid green with auto-off (20s)
    - error     : Flashing red alert
    - off       : All LEDs off
"""

import math
import sys
import time
import ujson
import uselect
from machine import PWM, Pin

# Pin mappings for Pimoroni Tiny 2040 onboard RGB LED
PIN_RED = 18
PIN_GREEN = 19
PIN_BLUE = 20

# PWM frequency (Hz)
PWM_FREQ = 1000

# Auto-off timeout for 'done' state (milliseconds)
DONE_TIMEOUT_MS = 20_000
DONE_FADE_MS = 3_000


class RGBController:
    """Controls the active-low onboard RGB LED using hardware PWM."""

    def __init__(self, pin_red=PIN_RED, pin_green=PIN_GREEN, pin_blue=PIN_BLUE):
        self.pwm_r = PWM(Pin(pin_red))
        self.pwm_g = PWM(Pin(pin_green))
        self.pwm_b = PWM(Pin(pin_blue))

        self.pwm_r.freq(PWM_FREQ)
        self.pwm_g.freq(PWM_FREQ)
        self.pwm_b.freq(PWM_FREQ)

        self.off()

    def set_rgb(self, r, g, b):
        """Sets normalized color levels [0.0, 1.0] with gamma correction and active-low inversion."""
        # Clamp inputs
        r = max(0.0, min(1.0, float(r)))
        g = max(0.0, min(1.0, float(g)))
        b = max(0.0, min(1.0, float(b)))

        # Quadratic gamma correction for natural human brightness perception
        r_gamma = r * r
        g_gamma = g * g
        b_gamma = b * b

        # Active-low duty cycle: 0 -> full ON, 65535 -> full OFF
        duty_r = int((1.0 - r_gamma) * 65535)
        duty_g = int((1.0 - g_gamma) * 65535)
        duty_b = int((1.0 - b_gamma) * 65535)

        self.pwm_r.duty_u16(duty_r)
        self.pwm_g.duty_u16(duty_g)
        self.pwm_b.duty_u16(duty_b)

    def off(self):
        """Turns all RGB LED channels completely off."""
        self.pwm_r.duty_u16(65535)
        self.pwm_g.duty_u16(65535)
        self.pwm_b.duty_u16(65535)


class StatusLightApp:
    """Manages agent state machine and autonomous animations."""

    def __init__(self, led):
        self.led = led
        self.state = "off"
        self.state_start_ms = time.ticks_ms()

    def set_state(self, new_state):
        """Transitions to a new state and resets animation timer."""
        if self.state != new_state:
            self.state = new_state
            self.state_start_ms = time.ticks_ms()

    def update(self):
        """Calculates and renders current animation frame."""
        now = time.ticks_ms()
        elapsed = time.ticks_diff(now, self.state_start_ms)

        if self.state == "thinking":
            # 2.0s smooth sinusoidal breathing in cyan
            phase = (elapsed % 2000) / 2000.0
            brightness = 0.5 + 0.5 * math.sin(phase * 2.0 * math.pi - math.pi / 2.0)
            self.led.set_rgb(0.0, brightness * 0.8, brightness * 1.0)

        elif self.state == "executing":
            # 0.8s fast pulsing pulse in purple/blue
            phase = (elapsed % 800) / 800.0
            brightness = 0.5 + 0.5 * math.sin(phase * 2.0 * math.pi - math.pi / 2.0)
            self.led.set_rgb(brightness * 0.6, 0.0, brightness * 1.0)

        elif self.state == "waiting":
            # 1.5s attention double-blink in amber (Red + partial Green)
            cycle = elapsed % 1500
            if cycle < 100 or (250 <= cycle < 350):
                self.led.set_rgb(1.0, 0.4, 0.0)
            else:
                self.led.off()

        elif self.state == "done":
            # Solid green with auto-off after 20 seconds (fade in last 3s)
            if elapsed >= DONE_TIMEOUT_MS:
                self.state = "off"
                self.led.off()
            elif elapsed > (DONE_TIMEOUT_MS - DONE_FADE_MS):
                remaining = DONE_TIMEOUT_MS - elapsed
                fade = remaining / float(DONE_FADE_MS)
                self.led.set_rgb(0.0, fade * 1.0, 0.0)
            else:
                self.led.set_rgb(0.0, 1.0, 0.0)

        elif self.state == "error":
            # 0.4s flashing red warning alert
            cycle = elapsed % 400
            if cycle < 200:
                self.led.set_rgb(1.0, 0.0, 0.0)
            else:
                self.led.off()

        else:
            # "off" or unrecognized state
            self.led.off()


def main():
    """Main execution loop: non-blocking USB CDC line reader and animation driver."""
    led = RGBController()
    app = StatusLightApp(led)

    # Configure non-blocking poll on sys.stdin
    poll = uselect.poll()
    poll.register(sys.stdin, uselect.POLLIN)

    input_buffer = ""

    while True:
        # Drain all available serial bytes non-blocking
        while poll.poll(0):
            try:
                char = sys.stdin.read(1)
                if not char:
                    break
                if char in ("\n", "\r"):
                    line = input_buffer.strip()
                    input_buffer = ""
                    if line:
                        try:
                            data = ujson.loads(line)
                            if isinstance(data, dict):
                                state = data.get("state")
                                if state:
                                    app.set_state(str(state).lower())
                        except Exception:
                            # Silently ignore corrupted frames or partial fragments
                            pass
                else:
                    input_buffer += char
                    # Prevent buffer bloat if no newline received
                    if len(input_buffer) > 256:
                        input_buffer = ""
            except Exception:
                break

        # Update animation frame
        app.update()

        # Yield to USB background tasks and maintain ~100 FPS
        time.sleep_ms(10)


if __name__ == "__main__":
    main()
