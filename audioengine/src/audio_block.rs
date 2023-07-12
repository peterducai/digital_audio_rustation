enum AudioBlock {
    MIDI,
    AUDIO,
    DEVICE
}

enum Content {
    MIDI,
    WAV,
    FLAC,
    DEVICE
}

//Track can be MIDI, Audio, Device
struct Track {
    kind: AudioBlock,
    address: String,
    buffer: Content
}
