*From my notes of August 2023.*

In August 2023, while helping a friend prepare for the Universidad Nacional de Colombia admission exam, we kept running into problems involving the greatest common divisor and the least common multiple (mcd and mcm in Spanish, or gcd and lcm here). Rather than memorizing the identity relating them, I wanted to see it directly from prime factorizations.

The key idea is to give every prime a fixed position in the representation of a positive integer, assigning exponent zero to primes that do not divide it (so they contribute $1$, the multiplicative identity). Multiplication, $\operatorname{gcd}$, and $\operatorname{lcm}$ then reduce to simple operations on exponent sequences.

## Starting with 48 and 36

Consider $48$ and $36$ written in prime-factored form:

$$
\begin{aligned}
48 &= 2 \cdot 2 \cdot 2 \cdot 2 \cdot 3 = 2^4 \cdot 3 \\
36 &= 2 \cdot 2 \cdot 3 \cdot 3 = 2^2 \cdot 3^2
\end{aligned}
$$

Taking the smaller exponent for each prime gives the greatest common divisor, and taking the larger exponent gives the least common multiple:

$$
\begin{aligned}
\operatorname{gcd}(48,36) &= 2^{\min(4,2)} \cdot 3^{\min(1,2)} = 2^2 \cdot 3 = 12 \\
\operatorname{lcm}(48,36) &= 2^{\max(4,2)} \cdot 3^{\max(1,2)} = 2^4 \cdot 3^2 = 144
\end{aligned}
$$

Multiplying the two numbers and regrouping their prime factors yields:

$$
\begin{aligned}
48 \cdot 36 &= (2^4 \cdot 3)(2^2 \cdot 3^2) \\
&= (2^2 \cdot 3)(2^4 \cdot 3^2) \\
&= \operatorname{gcd}(48,36) \cdot \operatorname{lcm}(48,36)
\end{aligned}
$$

Hence $\operatorname{lcm}(48,36) = (48 \cdot 36)/12 = 144$. The rest of this note formalizes why this holds for all positive integers.

## Prime-exponent representation

Let $p_1 < p_2 < p_3 < \dots$ be the sequence of all prime numbers ($p_1 = 2$, $p_2 = 3$, $p_3 = 5$, and so on). By the **Fundamental Theorem of Arithmetic**, every positive integer $a \in \mathbb{Z}_{>0}$ has a unique prime factorization, which we can write as

$$
a = \prod_{i=1}^{\infty} p_i^{n_i}
$$

where each exponent $n_i \in \mathbb{Z}_{\ge 0}$ is a nonnegative integer and $n_i = 0$ for all but finitely many indices $i$ (finite support). Any prime that does not divide $a$ receives exponent $0$, contributing $p_i^0 = 1$ and leaving the product unchanged. In particular, $a = 1$ has $n_i = 0$ for all $i$. Zero itself is excluded from this representation.

For example, $48$ corresponds to the exponent sequence $(4, 1, 0, 0, \dots)$, and $36$ corresponds to $(2, 2, 0, 0, \dots)$. Because only finitely many exponents are nonzero, the infinite product is simply a finite product written in a uniform index set, with no convergence issues.

Given a second positive integer $b = \prod_{i=1}^{\infty} p_i^{m_i}$, multiplication adds exponents componentwise:

$$
ab = \prod_{i=1}^{\infty} p_i^{n_i + m_i}
$$

## Divisibility, gcd, and lcm

For positive integers $d$ and $a$, we say that $d$ **divides** $a$ (written $d \mid a$) if there exists a positive integer $k$ such that $a = dk$.

**Lemma (Divisibility in prime exponents).** *Let $d = \prod_{i=1}^{\infty} p_i^{d_i}$ and $a = \prod_{i=1}^{\infty} p_i^{n_i}$ be positive integers. Then $d \mid a$ if and only if $d_i \le n_i$ for all $i \ge 1$.*

*Proof.* If $d \mid a$, then $a = dk$ for some positive integer $k = \prod_{i=1}^{\infty} p_i^{k_i}$ with $k_i \ge 0$. Multiplying gives $a = \prod_{i=1}^{\infty} p_i^{d_i + k_i}$, so uniqueness of prime factorization implies $n_i = d_i + k_i \ge d_i$ for every $i$. Conversely, if $d_i \le n_i$ for all $i$, define $k_i = n_i - d_i \ge 0$. Since $n_i = 0$ for all but finitely many $i$, the same holds for $k_i$, so $k = \prod_{i=1}^{\infty} p_i^{k_i}$ is a well-defined positive integer satisfying $dk = a$, hence $d \mid a$.

Recall the standard definitions for positive integers $a$ and $b$:

- A **common divisor** of $a$ and $b$ is a positive integer $d$ such that $d \mid a$ and $d \mid b$. The **greatest common divisor** $\operatorname{gcd}(a,b)$ is the largest such integer.
- A **common multiple** of $a$ and $b$ is a positive integer $m$ such that $a \mid m$ and $b \mid m$. The **least common multiple** $\operatorname{lcm}(a,b)$ is the smallest such integer.

**Proposition (Prime-exponent formulas).** *For any positive integers $a = \prod_{i=1}^{\infty} p_i^{n_i}$ and $b = \prod_{i=1}^{\infty} p_i^{m_i}$,*

$$
\operatorname{gcd}(a,b) = \prod_{i=1}^{\infty} p_i^{\min(n_i,m_i)}, \qquad
\operatorname{lcm}(a,b) = \prod_{i=1}^{\infty} p_i^{\max(n_i,m_i)}
$$

*Proof.* Both products have finite support because $0 \le \min(n_i, m_i) \le n_i$ and $0 \le \max(n_i, m_i) \le n_i + m_i$, so each defines a positive integer.

Let $g = \prod_{i=1}^{\infty} p_i^{\min(n_i,m_i)}$. Since $\min(n_i,m_i) \le n_i$ and $\min(n_i,m_i) \le m_i$ for every $i$, the lemma gives $g \mid a$ and $g \mid b$, so $g$ is a common divisor. If $d = \prod_{i=1}^{\infty} p_i^{d_i}$ is any common divisor of $a$ and $b$, then by the lemma $d_i \le n_i$ and $d_i \le m_i$, whence $d_i \le \min(n_i,m_i)$ for all $i$. Applying the lemma again yields $d \mid g$, and since both are positive integers, $d \le g$. Therefore $g = \operatorname{gcd}(a,b)$.

Similarly, let $L = \prod_{i=1}^{\infty} p_i^{\max(n_i,m_i)}$. Because $n_i \le \max(n_i,m_i)$ and $m_i \le \max(n_i,m_i)$ for all $i$, we have $a \mid L$ and $b \mid L$, so $L$ is a common multiple. If $m = \prod_{i=1}^{\infty} p_i^{m'_i}$ is any common multiple of $a$ and $b$, then $n_i \le m'_i$ and $m_i \le m'_i$, so $\max(n_i,m_i) \le m'_i$ for all $i$. By the lemma, $L \mid m$, which implies $L \le m$. Therefore $L = \operatorname{lcm}(a,b)$.

## The product identity

**Theorem.** *For all positive integers $a$ and $b$,*

$$
\operatorname{gcd}(a,b) \cdot \operatorname{lcm}(a,b) = ab
$$

*Proof.* For any two numbers $x$ and $y$, either $x \le y$ (so $\min(x,y) = x$ and $\max(x,y) = y$) or $x > y$ (so $\min(x,y) = y$ and $\max(x,y) = x$). In either case,

$$
\min(n_i,m_i) + \max(n_i,m_i) = n_i + m_i
$$

Multiplying the prime-exponent formulas for $\operatorname{gcd}(a,b)$ and $\operatorname{lcm}(a,b)$ adds their exponents in each position:

$$
\begin{aligned}
\operatorname{gcd}(a,b) \cdot \operatorname{lcm}(a,b)
&= \prod_{i=1}^{\infty} p_i^{\min(n_i,m_i)+\max(n_i,m_i)} \\
&= \prod_{i=1}^{\infty} p_i^{n_i+m_i} \\
&= ab
\end{aligned}
$$

Since $a$ and $b$ are positive, $\operatorname{gcd}(a,b) \ge 1$ is nonzero, and we obtain

$$
\operatorname{lcm}(a,b) = \frac{ab}{\operatorname{gcd}(a,b)}
$$
