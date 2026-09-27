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

## 2. States

| State | Colour / Pattern | Meaning |
| :--- | :--- | :--- |
| **`working`** | Blue, slow breathe (2.5 s, never fully dark) | Agent busy: reasoning or running tools |
| **`waiting`** | Amber, double-blink | Agent needs you: permission prompt or question |
| **`done`** | Green, solid | Turn finished, your turn |
| **`off`** | Dark | Session ended |

Any other state, including the removed `thinking`, `executing` and `error`, turns the LED off. Re-sending the current state doesn't restart its animation.

The logic lives in `status_light.py`, which has no hardware dependencies. `main.py` only drives the LED and reads USB serial.

---

## 3. Installation Guide

### Step 1: Flash MicroPython to the Tiny 2040
1. Hold down the **BOOT** button on the Tiny 2040 while plugging the USB-C cable into your computer.
2. The board will mount as a mass storage volume named `RPI-RP2`.
3. Download the latest MicroPython UF2 firmware for RP2040:
   - [Official MicroPython RP2040 Downloads](https://micropython.org/download/RPI_PICO/) or Pimoroni Tiny 2040 UF2.
4. Drag and drop the `.uf2` file onto the `RPI-RP2` drive. The board will automatically reboot running MicroPython.

### Step 2: Copy the Firmware to the Device

Copy both `main.py` and `status_light.py` using [`mpremote`](https://docs.micropython.org/en/latest/reference/mpremote.html):

```bash
# Install mpremote if not already installed
pip install mpremote

# Copy both files to the root filesystem of the board
mpremote cp firmware/tiny2040/main.py :main.py + cp firmware/tiny2040/status_light.py :status_light.py

# Soft-reboot the microcontroller to run main.py
mpremote soft-reset
```

Alternatively, open each file in [Thonny IDE](https://thonny.org/) and select **File -> Save As -> MicroPython Device**, keeping the same file names.

---

## 4. Testing & Verification

The state and animation logic runs on desktop Python too:

```bash
python firmware/tiny2040/test_status_light.py
```

Once the firmware is running on the Tiny 2040, send states over serial:

```bash
# Replace /dev/ttyACM0 with your board's serial port (e.g. COM3 on Windows or /dev/cu.usbmodem* on macOS)
agent-status-light --port /dev/ttyACM0 set working
agent-status-light --port /dev/ttyACM0 set waiting
agent-status-light --port /dev/ttyACM0 set done
agent-status-light --port /dev/ttyACM0 set off
```

Or send raw JSON directly:

```bash
echo '{"state":"working","agent":"cli","session_id":"test"}' > /dev/ttyACM0
```
