# Architecture

The design follows a dataflow architecture where multiple tasks
communicate by sending messages to each other. This system is built on
top of BSP (Board Support Package) that interface with the HAL 
(Hardware
Abstraction Layer), which in turn interacts with the hardware.

* Tasks:
  Tasks are individual units of work that are executed periodically.
  Each task has specific information such execution interval, time
  budget, and memory budget.

* Channel:
  Channels are used for inter-task communication. They allow tasks to
  send and receive messages asynchronously.

* Metrics:
  Metrics are used to monitor the performance and resource usage of
  tasks as well as any other metrics in the system as a hole. The
  MetricsTask is responsible for periodically printing these metrics
  to the console and then resetting them.

The following diagram shows the basic architecture of the applications
components.

```mermaid
block-beta
    columns 1
    block: BSP:1
        columns 4
        KeyboardTask
        TextEditTask
        ChatTask
        CryptoTask
        RenderTask
        NetLinkTask
        DisplayTask
        MetricsTask
      PttTask
      CodecTask
    end
    block: base:1
        columns 4
        TaskMgr
        Channel
        Metrics
    end
```

## Data Flows

The following diagrams show the data flow between tasks for various
operation. The labels on the arrows show the message type used in
the channel for communication.

### Outbound Text Message Data Flow

```mermaid
flowchart LR
    A[Keyboard] -->|Keypress| B[TextEdit]
    B -->|TextInput| C[Chat]
    C -->|TxtMsg| D[Crypto]
    D -->|EncTxtMsg| F[NetLink]
    C -->|print| J[Render]
    J -->|bitmap| K[Display]
```

### Inbound Text Message Data Flow

```mermaid
flowchart LR
    A[NetLink] -->|EncTxtMsgIn| B[Crypto]
    B -->|TxtMsgIn| C[Chat]
    C -->|print| J[Render]
    J -->|bitmap| K[Display]
```

### Outbound Audio Data Flow

```mermaid
flowchart LR
    A[Audio] -->|AudOut| B[Codec]
    B -->|EncAudOut| C[PTT]
    C -->|AudMsgOut| D[Crypto]
    D -->|EncAudMsgOut| F[NetLink]
```

### Inbound Audio Data Flow

```mermaid
flowchart LR
    A[NetLink] -->|EncAudMsgIn| B[Crypto]
    B -->|AudMsgIn| C[PTT]
    C -->|EncAudIn| J[Codec]
    J -->|AudIn| K[Audio]
```

### Group Selection Data Flow

TODO - describe how the group selection works

### Remote Storage Data Flows

TODO - describe flows for remote storage

# Board Support and Hardware Abstraction Layers

The following diagram shows how the applications uses modules from the
BSP layer which in turn use the HAL layer to interact with the
hardware.

```mermaid
block-beta
    columns 1
    Application
    block: BSP
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
    block: HAL
        SPI
        I2C
        I2S
        UART
        GPIO
        ADC
    end
    block: Hardware
        LCD
        M24C02
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
    I2C --> M24C02
    GPIO --> Switches
    UART --> USB
```

