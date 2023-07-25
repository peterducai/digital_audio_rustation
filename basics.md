Sampling Rate – this is the number of times per second an incoming signal is measured. Typical rates include 44.1 kHz (44,100) for audio CD and 48 kHz for audio for video productions. High-resolution audio can go upwards of 96 kHz or 192 kHz.

48kHz.
 

This sample rate is also used as a standard rate alongside 44.1kHz. Do check though, as audio recorded in one rate and played at another will be either speeded up or slowed down.

 

 

88.2kHz.
 

This is now the gold standard for hi-res recordings. Using this sample rate produces less distortion (called ‘aliasing’) when converting from analogue to digital and allows greater freedom when mixing and mastering. 

 

 

96kHz.
 

Similar to 88.2kHz, this sample rate provides more options when mixing and mastering the audio. But working at these higher rates could be an issue if your computer can’t handle the added information and storage needed. 

 

 

192kHz.
 

Some reports have suggested that recording at such a high sample rate can produce issues in your audio, such as jittering. It’s also hard to find computers that can handle it. Really, it’s only useful for slowing down high-frequency audio.
 



 

Nyquist Frequency – this is defined as the highest frequency that can accurately be reproduced by a digital system. It is equal to 1/2 of the sampling rate. For example, a 48 kHz sampling rate can theoretically accurately reproduce frequencies up to 24 kHz.





8-bit audio.
 

This is a fairly low-quality reproduction, producing audio at 46dB - around half the top level of human hearing.  

 

 

16-bit audio.
 

This is where the human ear can usually hear to at 96dB.

 

 

24-bit audio.
 

While at 145dB it’s well above the human hearing range, it can be useful to work at this level to reduce the ‘noise floor’ - essentially the digital white noise. 

 

 

32-bit float audio.
 

This can offer nearly infinite decibel levels and is really only used for super-high-quality audio - for example, sudden loud noises that need capturing without the use of limiters.

The audio bit depth determines the number of possible amplitude values we can record for each audio sample. The higher the bit depth, the more amplitude values per sample are captured to recreate the original audio signal.


The most common audio bit depths are 16-bit, 24-bit, and 32-bit. Each is a binary term, representing a number of possible values. Systems of higher audio bit depths are able to express more possible values:

16-bit: 65,536 values
24-bit: 16,777,216 values
32-bit: 4,294,967,296 values
Higher bit depths mean higher resolution audio; if the bit depth is too low, some information of the original audio signal will be lost. With a higher audio bit depth—and therefore a higher resolution—more amplitude values are available for us to record. As a result, the continuous analog wave’s exact amplitude is closer to an available value when sampled. Therefore, a digital approximation of the amplitude becomes closer to the original fluid analog wave.

16-bit: 65,536 amp. values
24-bit: 16,777,217 amp. values
32-bit: 4,284,967,296 amp. values
Increasing the audio bit depth, along with increasing the audio sample rate, creates more total points to reconstruct the analog wave.

https://www.izotope.com/en/learn/digital-audio-basics-sample-rate-and-bit-depth.html


Aliasing and Foldback – is a process that produces non-harmonic frequencies that are not part of the input signal. These frequencies are artifacts of the sampling and digitization process itself. They are folded back into the audio range and create ringing and other unwanted noise.

Anti-Alias Low Pass Filtering – this process attempts to filter out the frequencies that exceed the Nyquist frequency from the input signal. These frequencies would otherwise be folded back into the audio range. The effectiveness is contingent upon the current sampling rate. Higher oversampling rates ease the burden of the filter design which would otherwise require very steep slopes to be effective in reducing aliasing.

Oversampling Rate – this is a resampling rate based on the original sampling rate. For example, if the original sample rate is 48 kHz, an oversampling rate of 2x infers a resampling or upsampling rate of 96 kHz.

Upsampling – the process of resampling a signal at a higher rate than the incoming signal.

Downsampling – the process of reducing the sampling rate of a process back to the original rate of the system.

Bit Depth – the number of binary bits used to describe a particular sample. Typical bit depths include 16-bit (audio CD), 24-bit and 32-bit for higher resolution audio. The higher the bit depth the wider the dynamic range and the lower the noise floor will be. The number of possible values in a 16-bit system is 2^16 = 65,536. The number of possible values in a 24-bit system is 2^24 = 16,777,216. This means the samples measured will be far more accurate in a 24-bit system resulting in less quantization error and a higher signal to noise ratio: 96 dB for a 16-bit system versus 144 dB for a 24-bit system.

Harmonic Distortion – this occurs when frequencies that are harmonically related (integer multiples) to the incoming fundamental frequencies are generated using processes like overdriving, clipping, limiting, or saturation. The timbral effects produced are usually a pleasing and desirable coloration of the input signal.