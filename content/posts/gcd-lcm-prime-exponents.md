*From my notes of August 2023.*

I never liked memorizing formulas for the greatest common divisor and the least common multiple—mcd and mcm in my Spanish-language notes, or gcd and lcm here. I wanted to see why they worked.

What caught my attention was the rule for prime factorizations: take the minimum of the exponents to find the gcd, and the maximum to find the lcm. I tried to write that observation more generally, giving every prime a place, even when it did not divide the number. The result made a familiar formula feel much more natural to me.

This is an elementary property, but I found the way it fits together interesting enough to share. What follows is a cleaned-up transcription of the example and proof in my notes.

## Starting with 48 and 36

The example begins with their prime factorizations:

> 48 = 2 · 2 · 2 · 2 · 3 = 2<sup>4</sup> · 3
>
> 36 = 2 · 2 · 3 · 3 = 2<sup>2</sup> · 3<sup>2</sup>

A common divisor cannot contain more copies of either prime than either number has. Taking the smaller exponent for each prime gives the greatest common divisor:

> gcd(48, 36) = 2<sup>min(4, 2)</sup> · 3<sup>min(1, 2)</sup> = 2<sup>2</sup> · 3 = 12.

A common multiple must contain enough copies of each prime for both numbers to divide it. Taking the larger exponent gives the least common multiple:

> lcm(48, 36) = 2<sup>max(4, 2)</sup> · 3<sup>max(1, 2)</sup> = 2<sup>4</sup> · 3<sup>2</sup> = 144.

Multiplying the original numbers and regrouping their factors gives:

> 48 · 36 = (2<sup>4</sup> · 3)(2<sup>2</sup> · 3<sup>2</sup>)
>
> = (2<sup>2</sup> · 3)(2<sup>4</sup> · 3<sup>2</sup>)
>
> = gcd(48, 36) · lcm(48, 36).

So lcm(48, 36) = (48 · 36) / 12 = 144. The next step was to express why the same rearrangement always works.

## Giving every prime a place

Let p<sub>i</sub> be the ith prime, so p<sub>1</sub> = 2, p<sub>2</sub> = 3, p<sub>3</sub> = 5, and so on. By unique prime factorization, any two **positive integers** a and b can be written as

> a = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>nᵢ</sup>,
>
> b = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>mᵢ</sup>,

where every n<sub>i</sub> and m<sub>i</sub> is a nonnegative integer. If a prime does not divide a number, its exponent is zero: its contribution is simply p<sup>0</sup> = 1.

For example, 48 starts with exponents (4, 1, 0, 0, …), while 36 starts with (2, 2, 0, 0, …). The number 1 has zero in every position.

Although the product notation runs over infinitely many primes, only finitely many exponents are nonzero for any given positive integer. These are ordinary finite prime factorizations written in a common notation; there is no convergence issue. Zero itself is outside this representation.

Multiplication now amounts to adding exponents in each position:

> ab = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>nᵢ + mᵢ</sup>.

That was the part I liked: the same list of primes lets us describe multiplication, gcd, and lcm by what they do to the exponents.

## Minimum for gcd, maximum for lcm

A positive integer d divides a exactly when, for every prime, the exponent in d is at most the exponent in a. For d to divide both a and b, its ith exponent must therefore be at most min(n<sub>i</sub>, m<sub>i</sub>).

Choosing that minimum in every position produces a common divisor that every other common divisor divides. Hence

> gcd(a, b) = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>min(nᵢ, mᵢ)</sup>.

Similarly, a positive common multiple must have an ith exponent at least max(n<sub>i</sub>, m<sub>i</sub>). Choosing exactly those maxima produces a common multiple that divides every other common multiple. Hence

> lcm(a, b) = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>max(nᵢ, mᵢ)</sup>.

The zero exponents make primes absent from one factorization fit the same rule. For instance, if a prime divides a but not b, its exponent in the gcd is zero, while its exponent in the lcm is the one from a.

## The identity falls out

For each pair of exponents, the minimum and maximum are just the same two values in increasing order. Therefore

> min(n<sub>i</sub>, m<sub>i</sub>) + max(n<sub>i</sub>, m<sub>i</sub>) = n<sub>i</sub> + m<sub>i</sub>.

Multiplying the gcd and lcm adds their exponents, so

> gcd(a, b) · lcm(a, b)
>
> = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>min(nᵢ, mᵢ) + max(nᵢ, mᵢ)</sup>
>
> = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>nᵢ + mᵢ</sup>
>
> = ab.

Since a and b are positive, their gcd is nonzero, and we can rearrange this as

> **lcm(a, b) = ab / gcd(a, b).**

The formula I had not wanted to memorize had become a consequence of the simple identity min(x, y) + max(x, y) = x + y, applied to every prime exponent.

*Transcription note: the handwritten expanded expressions correctly use minima for gcd and maxima for lcm, but the compact product expressions swap those labels in two places. They are corrected here.*

## Why I kept these notes

There is nothing new or especially deep about this result. What stayed with me was the experience of turning a rule I could memorize into something I could reconstruct: write the numbers through their prime exponents, then look at what the operations do in each position.

Later, in my Theory of Computation course, I used a similar property when constructing Turing machines. That connection is another reason I remember these notes fondly. A small observation about numbers had become useful in a different setting, and that was enough to make it worth keeping—and sharing.
