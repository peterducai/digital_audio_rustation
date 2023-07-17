enum DeviceType {
    MIDI,
    AUDIO,
    EFFECT
}

enum DeviceContent {
    MIDI,
    WAV,
    FLAC,
    DEVICE
}


enum InputType {
    AUDIO,
    MIDI,
    BINARY,
    VALUE
}


//Track can be MIDI, Audio, Device
struct DeviceBox {
    kind: DeviceType,
    address: String,
    timestamp: String,
    buffer: DeviceContent,
    ins:  [u64; 64],  // 64 inputs
    outs:  [u64; 64], // 64 outputs
    list:  [u64; 128],
    description: String,
    author: String
}
