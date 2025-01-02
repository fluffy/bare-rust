# Arch



## Outbound Text Message Data Flow

```mermaid
flowchart LR
    A[Keyboard] --> |Keypress| B[TextEdit]
    B --> |TextInput| C[Chat]
    C --> |TxtMsg| D[Crypto]
    D --> |EncTxtMsg| F[NetLink]
    C --> |print| J[Render]
    J --> |bitmap| K[Display]
```

## Inbound Text Message Data Flow

```mermaid
flowchart LR
    A[NetLink] --> |EncTxtMsgIn| B[Crypto]
    B --> |TxtMsgIn| C[Chat]
    C --> |print| J[Render]
    J --> |bitmap| K[Display]
```

## Device and Hardware Abstraction Layers

The following diagram shows how the applications uses 
modules from the DEV layer which uses the HAL layer
to interact with the hardware.

```mermaid
block-beta
columns 1
  Application
  block:DEV
      Display
      Storage
      Audio
      Console
      NetLink
      Keyboard
      Buttons
      Battery
  end
  space
  block:HAL
    SPI
    I2C
    I2S
    UART
    GPIO
    ADC
  end
  block:Hardware
        LCD
        EEPROM
        WM8960
        USB
        NetCPU
        Switches
  end
  Keyboard --> GPIO
  Display --> SPI
  Audio --> I2S
  Audio --> I2C
  Storage --> I2C
  Battery --> ADC
  Console --> UART
  Buttons --> GPIO
  NetLink --> UART
  SPI --> LCD
  UART --> NetCPU
  I2C --> WM8960
  I2S --> WM8960
  I2C --> EEPROM
  GPIO --> Switches
 UART --> USB
```

