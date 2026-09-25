# Pimoroni Tiny 2040 Reference Firmware

Reference MicroPython firmware for the **Pimoroni Tiny 2040** (Raspberry Pi RP2040), connecting its onboard active-low RGB LED to `agent-status-light` via USB CDC serial.

---

## 1. Hardware Specifications

- **Microcontroller**: Raspberry Pi RP2040 (dual ARM Cortex-M0+ @ 133MHz).
- **USB Interface**: Native USB CDC Serial (115200 baud, 8N1).
- **USB Identifiers**: VID `0x16D0`, PID `0x08C7`.
- **Onboard RGB LED**:
  - **Red**: GPIO 18
  - **Green**: GPIO 19
  - **Blue**: GPIO 20
  - **Polarity**: Active-Low (Common Anode: GPIO `0` = LED ON, GPIO `65535` = LED OFF).

---

## 2. Animation & State Mappings

| Agent State | Color / Pattern | Description |
| :--- | :--- | :--- |
| **`thinking`** | Breathing Cyan | Smooth 2.0s sinusoidal brightness breathe |
| **`executing`** | Pulsing Purple/Blue | Fast 0.8s pulsing animation |
| **`waiting`** | Double-Blink Amber | Sharp attention double-flash alert |
| **`done`** | Solid Green | Solid green with auto-fade and turn-off after 20s |
| **`error`** | Flashing Red | 0.4s fast flashing warning alert |
| **`off`** | All Off | All PWM channels disabled |

---

## 3. Installation Guide

### Step 1: Flash MicroPython to the Tiny 2040
1. Hold down the **BOOT** button on the Tiny 2040 while plugging the USB-C cable into your computer.
2. The board will mount as a mass storage volume named `RPI-RP2`.
3. Download the latest MicroPython UF2 firmware for RP2040:
   - [Official MicroPython RP2040 Downloads](https://micropython.org/download/RPI_PICO/) or Pimoroni Tiny 2040 UF2.
4. Drag and drop the `.uf2` file onto the `RPI-RP2` drive. The board will automatically reboot running MicroPython.

### Step 2: Copy `main.py` to the Device

You can install `main.py` onto the microcontroller using [`mpremote`](https://docs.micropython.org/en/latest/reference/mpremote.html):

```bash
# Install mpremote if not already installed
pip install mpremote

# Copy main.py to the root filesystem of the board
mpremote cp firmware/tiny2040/main.py :main.py

# Soft-reboot the microcontroller to run main.py
mpremote soft-reset
```

Alternatively, open `firmware/tiny2040/main.py` in [Thonny IDE](https://thonny.org/) and select **File -> Save As -> MicroPython Device -> main.py**.

---

## 4. Testing & Verification

Once `main.py` is running on the Tiny 2040, test it using `agent-status-light`:

```bash
# Replace /dev/ttyACM0 with your board's serial port (e.g. COM3 on Windows or /dev/cu.usbmodem* on macOS)
agent-status-light --port /dev/ttyACM0 set thinking
agent-status-light --port /dev/ttyACM0 set executing
agent-status-light --port /dev/ttyACM0 set waiting
agent-status-light --port /dev/ttyACM0 set done
agent-status-light --port /dev/ttyACM0 set error
agent-status-light --port /dev/ttyACM0 set off
```

Or test raw JSON directly over serial:

```bash
echo '{"state":"thinking","agent":"antigravity","session_id":"test"}' > /dev/ttyACM0
```
