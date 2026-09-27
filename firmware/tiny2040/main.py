"""
Pimoroni Tiny 2040 firmware for agent-status-light.

Drives the onboard active-low RGB LED (GP18 red, GP19 green, GP20 blue) from
status updates received over USB CDC serial (115200 baud, newline-delimited JSON):

    {"state":"working","agent":"claude","session_id":"..."}\\n

States and animations live in `status_light.py`, which must be copied to the
board alongside this file.
"""

import select
import sys
import time

from machine import PWM, Pin

from status_light import LineBuffer, StatusLight, parse_state

# Pin mappings for Pimoroni Tiny 2040 onboard RGB LED
PIN_RED = 18
PIN_GREEN = 19
PIN_BLUE = 20

# PWM frequency (Hz)
PWM_FREQ = 1000


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


def main():
    """Main execution loop: non-blocking USB CDC line reader and animation driver."""
    led = RGBController()
    light = StatusLight(time.ticks_ms(), time.ticks_diff)

    # Configure non-blocking poll on sys.stdin
    poll = select.poll()
    poll.register(sys.stdin, select.POLLIN)

    lines = LineBuffer()

    while True:
        # Drain all available serial bytes non-blocking
        while poll.poll(0):
            try:
                char = sys.stdin.read(1)
                if not char:
                    break
                line = lines.feed(char)
                if line:
                    state = parse_state(line)
                    # Ignore corrupted frames or partial fragments
                    if state is not None:
                        light.set_state(state, time.ticks_ms())
            except Exception:
                break

        # Update animation frame
        led.set_rgb(*light.colour(time.ticks_ms()))

        # Yield to USB background tasks and maintain ~100 FPS
        time.sleep_ms(10)


if __name__ == "__main__":
    main()
