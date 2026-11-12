*From my notes of August 2023.*

In August 2023, I was helping a friend with some exercises to prepare for the admission exam at Universidad Nacional de Colombia. We were working with the greatest common divisor and the least common multiple (mcd and mcm in Spanish, or gcd and lcm here), and I ended up getting curious about something quite simple.

I never really liked memorizing the formulas for these things. While going through the exercises, I noticed how the gcd takes the minimum of the prime exponents and the lcm takes the maximum. I wanted to follow that idea a little further and see whether I could write everything in the same way.

What I liked was the possibility of giving every prime a place in the representation of a number. Even a prime that does not divide it can stay there with exponent zero (contributing 1, the multiplicative identity). We can build the description from all the prime pieces, while only some of them actually change the number.

I know this is not a big result. It was just a small curiosity I came across while trying to help a friend, but I enjoyed seeing a familiar formula come out of that way of looking at numbers. These are the example and the argument I wrote down.

## Starting with 48 and 36

I started with 48 and 36, writing out their prime factorizations:

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

So lcm(48, 36) = (48 · 36) / 12 = 144. Seeing the factors rearrange like this made me want to write down why it would work for any pair of positive integers.

## Giving every prime a place

To put all the prime pieces in the same order, I called the ith prime p<sub>i</sub>, so p<sub>1</sub> = 2, p<sub>2</sub> = 3, p<sub>3</sub> = 5, and so on. By unique prime factorization, any two **positive integers** a and b can be written as

> a = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>nᵢ</sup>,
>
> b = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>mᵢ</sup>,

Here, every n<sub>i</sub> and m<sub>i</sub> is a nonnegative integer. If a prime does not divide a number, I give it exponent zero. Its contribution is then p<sup>0</sup> = 1 (so including it leaves the number unchanged).

For example, 48 starts with exponents (4, 1, 0, 0, …), while 36 starts with (2, 2, 0, 0, …). The number 1 has zero in every position.

Although the product notation runs over infinitely many primes, only finitely many exponents are nonzero for any given positive integer. These are ordinary finite prime factorizations written in a common notation; there is no convergence issue. Zero itself is outside this representation.

Multiplication now amounts to adding exponents in each position:

> ab = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>nᵢ + mᵢ</sup>.

This was the part I found especially nice. Both numbers now use the same prime pieces, including the ones that only contribute the identity. I could look at multiplication, gcd, and lcm as different ways of working with the exponents in those same positions.

## Minimum for gcd, maximum for lcm

A positive integer d divides a exactly when, for every prime, the exponent in d is at most the exponent in a. For d to divide both a and b, its ith exponent must therefore be at most min(n<sub>i</sub>, m<sub>i</sub>).

Choosing that minimum in every position produces a common divisor that every other common divisor divides. Hence

> gcd(a, b) = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>min(nᵢ, mᵢ)</sup>.

Similarly, a positive common multiple must have an ith exponent at least max(n<sub>i</sub>, m<sub>i</sub>). Choosing exactly those maxima produces a common multiple that divides every other common multiple. Hence

> lcm(a, b) = ∏<sub>i = 1</sub><sup>∞</sup> p<sub>i</sub><sup>max(nᵢ, mᵢ)</sup>.

The zero exponents make primes absent from one factorization fit the same rule. For instance, if a prime divides a but not b, its exponent in the gcd is zero, while its exponent in the lcm is the one from a.

## The formula I wanted to understand

At this point, the observation I needed was very small: the minimum and maximum of two exponents are just those same two values in increasing order. So

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

That was enough to recover the formula I had not wanted to memorize. For each prime, taking the minimum and maximum and adding them gives the same exponent as multiplying the original numbers. Once I saw it that way, I felt I could find the formula again whenever I needed it.

*Transcription note: the handwritten expanded expressions correctly use minima for gcd and maxima for lcm, but the compact product expressions swap those labels in two places. They are corrected here.*

## Why I kept these notes

I kept these notes because I liked the way a choice of representation made the relationship easier to see. We start with all the prime pieces, let the unused ones contribute 1, and describe the operations through their exponents. Nothing complicated comes out of it, but I found that way of modeling the problem satisfying.

Later, in my Theory of Computation course, I used a similar property when constructing Turing machines. I enjoyed finding a connection with something I had first thought about while helping a friend with admission exercises.

This is just a small mathematical curiosity, and that is how I wanted to share it. Sometimes I keep a page of notes simply because I had a good time figuring something out.
