# BLE - Apple Manufacturer Data

## Proximity Protocol

### Pairing Mode (0x00)

| Index | Byte Length | Description |
|:-----:|:-----------:|:------------|
| 0x0 | 0x1 | `0x07` - Protocol ID |
| 0x1 | 0x1 | Protocol Length |
| 0x2 | 0x1 | `0x00` - Pairing Mode |
| 0x3 | 0x2 | Device Model |
| 0x5 | 0x6 | Bluetooth address |
| 0xB | 0x1 | *Unknown field* |
| 0xC | 0x1 | Right Battery |
| 0xD | 0x1 | Left Battery |
| 0xE | 0x1 | Case Battery |
| 0xF | 0x1 | Device Color |


### Paired Mode (0x01)

Byte Length: 27

| Byte Index | Byte Length | Description |
|:----------:|:-----------:|:------------|
| 0x0 | 0x1 | `0x07` - Protocol ID |
| 0x1 | 0x1 | `0x19` - Protocol Length (without Protocol ID and Length|
| 0x2 | 0x1 | `0x01` - Pairing Mode |
| 0x3 | 0x2 | Device Model |
| 0x4 | 0x1 | UTP |
| 0x5 | 0x2 | Battery Indicaton |
| 0x6 | 0x1 | Lid Indication |
| 0xC | 0x1 | Device Color |
| 0xD | 0x1 | `0x00` - *Unknown* |
| 0xE | 0xA | *Unknown* - Encrypted Payload |


### Device Color

Byte Length: 1

| Code | Color |
|:----:|:-----:|
| 0x00 | White |
| 0x01 | Black |
| 0x02 | Red |
| 0x03 | Blue |
| 0x04 | Pink |
| 0x05 | Gray |
| 0x06 | Silver |
| 0x07 | Gold |
| 0x08 | RoseGold |
| 0x09 | SpaceGray |
| 0x10 | *Unknown* - seen with AirPodsMax |
| 0x11 | *Unknown* - seen with PowerbeatsPro |
| 0x0A | DarkBlue |
| 0x0B | LightBlue |
| 0x0C | Yellow |

### Device Models

Byte Length: 2

| Code | Device |
|:----:|:------:|
| 0x0220 | AirPods1 |
| 0x0f20 | AirPods2 |
| 0x1320 | AirPods3 |
| 0x0e20 | AirPodsPro |
| 0x1420 | AirPodsPro2 | 
| 0x0a20 | AirPodsMax |
| 0x0b20 | PowerbeatsPro |
| 0x0520 | BeatsX |
| 0x1020 | BeatsFlex |
| 0x1120 | BeatsStudioBuds |
| 0x0620 | BeatsSolo3 |
| 0x0920 | BeatsStudio3 |
| 0x0320 | PowerBeats3 |
| 0x0c20 | BeatsSoloPro |

### Battery Indicaton

Byte Length: 2

* Battery Level: `0..10` for 10% steps
* Battery Level: `15`
  * disconnected
  * unknown
  * not supported
* Battery Charging: 
  * `0x1` - charging
  * `0x0` - not charging
* Single-device headphones uses:
  * Right Airpod - Battery Level
  * Right Airpod - Charging *(assumed)*

| Bit Index | Bit Length | Description |
|:---------:|:----------:|:------------|
| 0x0 | 0x4 | Case Battery Level |
| 0x4 | 0x1 | Left Airpod - Charging |
| 0x5 | 0x1 | Right Airpod - Charging |
| 0x6 | 0x1 | Case Charging |
| 0x7 | 0x1 | *Unknown* |
| 0x8 | 0x4 | Right Airpod - Battery Level |
| 0xC | 0x4 | Left Airpod - Battery Level  |


### Lid Indication

Byte Length: 1

* The lid count has to be interpreted in context with the opened/closed flag.
* The lid count is increased after each open and close. For example:
  1. Lid opens -> Open(1)
  2. Lid closes -> Close(1)
  3. Lid opens -> Open(2)
* If no airpod is in the case, the lid count is not updated. This is due to the fact that the 
  proximity messages are sent by the airpods and if these are no in the case, the state of the lid can not be known.

| Bit Index | Bit Length | Description |
|:---------:|:----------:|:------------|
| 0x0 | 0x3 | Count of opening/closing the case lid. |
| 0x3 | 0x1 | `0x0` if lid is closed, `0x1` if lid is opened |
| 0x4 | 0x4 | *Unknown* |


### UTP

Byte Length: 1

| Bit Index | Bit Length | Description |
|:---------:|:----------:|:------------|
| 0x0 | 0x1 | *Unknown* |
| 0x1 | 0x1 | One or both airpods are in ear |
| 0x2 | 0x1 | Both airpods are in case |
| 0x3 | 0x1 | Both airepods are in ears |
| 0x4 | 0x1 | One or both airpods are in case |
| 0x5 | 0x1 | `0x1` set for left airpod, `0x0` set or right airpod |
| 0x6 | 0x1 | `0x1` if battery level and charge values for left and right airpods are flipped |
| 0x7 | 0x1 | *Unknown* |

## References

* [Beats by Der - Red Ballon Security](https://redballoonsecurity.com/baets/) - Contributed Pairing Mode
* [Handoff All Your Privacy](https://petsymposium.org/2019/files/papers/issue4/popets-2019-0057.pdf) - Initial research
* [Discontinued Privacy](https://petsymposium.org/2020/files/papers/issue1/popets-2020-0003.pdf) - Contributed general proximity protocol 