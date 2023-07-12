enum DeviceType {
    MIDI,
    AUDIO,
    DEVICE
}

enum DeviceContent {
    MIDI,
    WAV,
    FLAC,
    DEVICE
}

//Track can be MIDI, Audio, Device
struct DeviceBox {
    kind: DeviceType,
    address: String,
    buffer: DeviceContent,
    ins:  [u64; 64],  // 64 inputs
    outs:  [u64; 64], // 64 outputs
    list:  [u64; 128]
}
