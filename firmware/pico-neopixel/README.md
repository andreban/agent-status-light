# Raspberry Pi Pico NeoPixel Firmware

MicroPython firmware for a **Raspberry Pi Pico** (RP2040) driving one or more WS2812 NeoPixels on GP16, connected to `agent-status-light` via USB CDC serial.

---

## 1. Hardware

- **Microcontroller**: Raspberry Pi Pico (RP2040). A Pico W or Pico 2 should work too, with the matching MicroPython build.
- **USB Interface**: Native USB CDC Serial (115200 baud, 8N1).
- **NeoPixels**: WS2812 / WS2812B (GRB), data in on **GP16**.
  - **5V / VIN**: VBUS (pin 40), or 3V3 (pin 36) for a single pixel.
  - **GND**: any GND pin.
  - **DIN**: GP16 (pin 21).

Every pixel shows the same colour. Set the number of chained pixels with `NUM_PIXELS` at the top of `main.py`:

| Hardware | `NUM_PIXELS` |
| :--- | :--- |
| Single NeoPixel | `1` |
| Adafruit NeoPixel Jewel (7 × RGB) | `7` |

`BRIGHTNESS` (default `0.5`) caps the peak brightness. The RGBW Jewel variant needs 4 bytes per pixel and isn't supported.

---

## 2. States

The states, colours and 10-minute fallback to `done` are the same as the Tiny 2040 firmware; see [`firmware/tiny2040/README.md`](../tiny2040/README.md#2-states). The logic lives in `firmware/tiny2040/status_light.py`, which both boards share.

---

## 3. Installation

1. Hold **BOOTSEL** while plugging the Pico in, then drag the [MicroPython UF2 for the Pico](https://micropython.org/download/RPI_PICO/) onto the `RPI-RP2` drive.
2. Copy `main.py` and the shared `status_light.py` to the board:

   ```bash
   pip install mpremote
   mpremote cp firmware/pico-neopixel/main.py :main.py + cp firmware/tiny2040/status_light.py :status_light.py
   mpremote reset
   ```

---

## 4. Testing

```bash
# Replace COM6 with your board's serial port (e.g. /dev/ttyACM0 on Linux or /dev/cu.usbmodem* on macOS)
agent-status-light --port COM6 set working
agent-status-light --port COM6 set waiting
agent-status-light --port COM6 set done
agent-status-light --port COM6 set off
```
