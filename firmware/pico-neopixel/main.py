"""
Raspberry Pi Pico NeoPixel firmware for agent-status-light.

Drives a strip or ring of WS2812 NeoPixels on GP16 from status updates received
over USB CDC serial (115200 baud, newline-delimited JSON):

    {"state":"working","agent":"claude","session_id":"..."}\\n

Every pixel shows the same colour. States and animations live in
`status_light.py` (in `firmware/tiny2040/`), which must be copied to the board
alongside this file.
"""

import select
import sys
import time

from machine import Pin
from neopixel import NeoPixel

from status_light import LineBuffer, StatusLight, parse_state

# Data pin the NeoPixels are connected to.
PIN_PIXELS = 16

# Number of NeoPixels chained on the data pin (1 for a single pixel, 7 for an Adafruit Jewel).
NUM_PIXELS = 1

# Peak brightness in [0.0, 1.0]. NeoPixels at full power are glaring up close.
BRIGHTNESS = 0.5


class PixelController:
    """Shows one colour on every NeoPixel, writing only when the colour changes."""

    def __init__(self, pin=PIN_PIXELS, count=NUM_PIXELS, brightness=BRIGHTNESS):
        self.pixels = NeoPixel(Pin(pin), count)
        self.brightness = brightness
        self.current = None
        self.set_rgb(0.0, 0.0, 0.0)

    def set_rgb(self, r, g, b):
        """Sets normalized colour levels [0.0, 1.0] with gamma correction and brightness scaling."""
        colour = tuple(self._level(c) for c in (r, g, b))
        if colour == self.current:
            return
        self.current = colour
        self.pixels.fill(colour)
        self.pixels.write()

    def _level(self, c):
        c = max(0.0, min(1.0, float(c)))
        # Quadratic gamma correction for natural human brightness perception
        return round(c * c * self.brightness * 255)


def main():
    """Main execution loop: non-blocking USB CDC line reader and animation driver."""
    pixels = PixelController()
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
        pixels.set_rgb(*light.colour(time.ticks_ms()))

        # Yield to USB background tasks and maintain ~100 FPS
        time.sleep_ms(10)


if __name__ == "__main__":
    main()
