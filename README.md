# Simpson's Rule

[Simpson's Rule](https://en.m.wikipedia.org/wiki/Simpson%27s_rule) is a method of quadrature, or approximating integrals,
named after the English mathematician Thomas Simpson (1710 - 1761). It works by evaluating the integrand at end- and mid-points,
and interpolating it to a parabola. This specific implementation allows specifying a number *n* to subdivide the interval by.
This allows much higher precision. Accuracy returns for *n* > 10 000 000 at 64-bit floating-point precision appear minimal.