**An adjustable fractal noise function using a modification of the diamond-square algorithm.**

[example](./noise.bmp)

Sharpness can be exponentially decayed to smoothly clamp the frequencies in the final texture.

The underlying fractal generated, without the blur, has been tuned to approximate critical systems like the ising model,
the isotropic property and scale invariance seem to be preserved.

This is currently a proof of concept, the algorithm generates the whole image at once,
although it can be trivially changed to generate just a portion based on cached values.
I believe this will make it very useful for terrain generation and texture work.
