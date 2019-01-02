## Supermutation

An attempt to improve on the work I did previously in [this repository](https://github.com/tuzz/leaps-and-bounds).

Unfortunately, this didn't really work out but I think this introduced some
novel ideas that could be useful to others.

## Setup

To run the test suite:

```
cargo test && cargo test --features four_symbols
```

To run the application:

```
cargo run --release
```

## Overview

This is my second attempt at the superpermutation problem. This time I decided
to represent each state in the search space as a node in a directed graph and
run an incremental A\* search on this graph.

Ultimately, the added complexity of this approach slows things down and this
isn't as effective as my first attempt at this problem - though it does
introduce some nice ideas that could be used by other approaches.

[Here are a few slides](https://github.com/tuzz/supermutation/blob/master/slides.pdf)
that you may wish to refer to for the following section.

## Key ideas

Below is a brief summary of the ideas explored in this project.

---

**1) We can reduce the problem to a shortest path problem**

Each candidate is represented by a BitSet that stores which permutations have
been seen so far in the string. A superpermutation is a candidate where all bits
are set in the BitSet. That means we can reduce the problem of finding a minimal
superpermutation to the problem of finding a shortest path through a graph.

In its current form, the problem is a search on the tree of all possible string
expansions, but arguably, these should all be considered the same state:

```
'12321' and '32123' and '123333321' and '321111123'
```

They contain the same set of permutations. The only difference is how they're
labelled and the number of 'wasted' symbols, which would already be captured in
the depth of the candidate in the search algorithm's data structures.

---

**2) We can incrementally learn a heuristic function to guide the search**

[This blog post](http://www.njohnston.ca/2014/08/all-minimal-superpermutations-on-five-symbols-have-been-found/)
describes a method to discover the maximum number of permutations that fit into
strings with a specific number of wasted symbols. This information can be used
to guide the search. For a shortest path problem, we can use the A\* algorithm
which has a heuristic function for this purpose.

We can break up the search into a series of searches where the goal is to reach
any candidate that contains some number of permutations. Once we've found the
shortest path to that candidate, we can improve our heuristic function to more
accurately guide the search.

This is a form of [incremental heuristic search](https://en.wikipedia.org/wiki/Incremental_heuristic_search).
See [this test](https://github.com/tuzz/supermutation/blob/master/src/heuristic/test.rs)
for more explanation on the heuristic function and
[this module](https://github.com/tuzz/supermutation/blob/master/src/incremental/mod.rs)
for how the incremental process works.

---

**3) We can relabel candidates without changing correctness**

Superpermutations have the property that their symbols can be relabelled and
the result is still a superpermutation, provided the relabelling is a
permutation of the symbols. For example, the candidate string '123412' can be
relabelled to '321432' according to the permutation (3214).

---

**4) We can 'canonicalise' candidates by relabelling them at each expansion**

Each candidate contains a BitSet that records which permutations it contains. We
also need to know which symbols are at the 'tail' of the string for subsequent
expansions. If we relabel candidates when they are expanded so their tail
symbols become ascending numbers, it simplifes this expansion.

For example, if we have the candidate '123412', we'd relabel this to '34**1234**'
according to the permutation (3412). This has some advantages:

- It [breaks symmetries](https://en.wikipedia.org/wiki/Symmetry_breaking) in the
problem space because multiple states are represented by a single canonicalised
state, changing the problem from a tree search to a smaller graph search

- We can store less information in each candidate since we know the tail of the
string will always be ascending numbers and we don't need to track this
separately

- We can immediately tell the result of expanding different symbols. The only
way to add a permutation to the string is to expand the first symbol, since
expanding any other must mean it occurs in the tail of the string. For example
if we have the string '1234' the only way to add a new permutation is to expand
a '1' to add '2341'

- If a permutation is added, it will always be '2341' before the candidate is
canonicalised. This means we can pre-compute the function that canonicalises
candidates, transforming an expansion operation into a mapping operation

There's one caveat to the above which is when the tail of a string contains
duplicate symbols, e.g. '1232'. In this case, the next symbol won't add a new
permutation and the relabelling is ambiguous. How this is handled is explained
in the subsequent ideas:

---

**5) We can store the length of unique tail symbols in the BitSet**

In addition to storing all the permutations a candidate string contains, there's
one more piece of information we need to store, which is the number of unique
symbols in the tail of the string. In the '1232' example the '32' is the longest
tail that contains unique symbols, so we'd store the number 2 (in unary) at the
end of the BitSet.

When a candidate is expanded, we first need to check if the string is 'ready' to
accept new permutations. If the counter bits are not 'full' then the next symbol
won't add a new permutation, but it might affect which counter bits are set.
Again, we can immediately tell the result of expanding different symbols on
these counter bits, which means we can pre-compute these mappings.

See [this test](https://github.com/tuzz/supermutation/blob/master/src/symmetry/test.rs#L81)
for more explanation on how counter bit mappings work.

---

**6) We can make relabelling unambiguous and break more symmetries**

When expanding candidates, sometimes we'll have a 'choice' as to how the tail of
the string is relabelled. For example, if the tail is '1232' then the longest
tail that contains unique symbols is '32'. This does not contain '1' or '4' and
this results in multiple possible relabellings.

Depending on which relabelling we choose, this affects the permutations a
candidate string contains and results in different BitSets. In the worst case,
there are (N-2)! possible relabellings for a string, meaning for N=6 there are
24 relabellings for the string '1234565' because '65' does not contain '1', '2',
'3' and '4' resulting in 4! degrees of freedom.

To decide which to use, we arbitrarily introduce an ordering of candidates. This
works by treating the BitSet as a number in binary. We choose the relabelling
that results in the largest number.

See [this test](https://github.com/tuzz/supermutation/blob/master/src/symmetry/test.rs#L156)
for more details on how this works.

## Closing remarks

This attempt turned out to be slower than I'd hoped and I'm still not really
sure why that is. I thought that by reducing the problem to the shortest path
problem, I'd be able to use A\* to rapidly work towards a solution, but perhaps
the added complexity of symmetry breaking and canonicalisation adds considerable
overhead and increases the likelihood of introducing hard-to-find bugs in the
code.

I did some benchmarking and the rate of exansion of candidates is about 40x
slower in this project than in my first attempt which is disappointing. I spent
a while trying to understand why that is but didn't make much progress.

Here are some other projects I've worked on to try and solve this problem:

- https://github.com/tuzz/leaps-and-bounds (my first attempt)
- https://github.com/tuzz/supersat (an attempt to reduce the problem to SAT)

I'm sorry this write-up is brief and there is no formal presentation of these ideas.

If you have any questions, please DM [me on twitter](https://twitter.com/chrispatuzzo). Thanks.
