# Ψ PSI: Portable scripting interface

A minimal, sensible, scripting language for configuring, extending and controlling your application.

## Example

```psi
/*
  A demonstration of the PSI language
  This file is using 0.3.0
*/

// Define PI
pi = 3.141592

// Define a radius
radius = 5

// Calculate the area and circumference of the circle
area = pi * radius ^ 2
circumference = pi * (radius * 2)

// Print out the results
print("A circle with a radius {radius} has an area of {area} and a circumference of {circumference}")
```

