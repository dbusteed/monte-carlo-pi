# Monte Carlo Pi Simulation

A [Monte Carlo](https://en.wikipedia.org/wiki/Monte_Carlo_method) Simulation built with Bevy + Rust that estimates Pi.

## Approach

Imagine I had a two containers. One is a circular red container with a radius of 3 feet (or whatever unit you'd like), the other is a yellow square container with a side length of 3 feet.

Now let's say we place these two container outside to collect rainwater. After a sufficient amount of rainwater is gathered, we can use a ratio of these two container to estimate Pi.

The area of a circle is $\pi r^2$, so our red circular container has an area of $\pi 3^2$.

The area of a square is simply the side length squared. For our purposes let's call the side length $r$ so that the area of the squre is $r^2$, or in the case of the yellow container, $3^2$.

When we divide the area of the circle with that of the square we are left with Pi.

$$ \frac{circle}{square} = \frac{\pi r^2}{r^2} = \pi $$

In the simulation, we put the two containers on the ground, and let them collect the rainwater. Many of the droplets don't go in either bucket, but some will go in either the circular one, or the square one. Throughout the simulation, we can divide the number of rain droplets caught in the circular container by that of the square and get our estimate of Pi!