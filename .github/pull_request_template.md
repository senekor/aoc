# Review Checklist

⚠️ This checklist is not exhaustive. Going throught the checklist does not replace a "freestyle" review.

### Commit history
- [ ] Do all commits obey the commit message convention?

### Project structure
- [ ] Is the project split into a library and a binary?
- [ ] Is the library's [module structure appropriate](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#module-structure)?
- [ ] Is the library's [API sufficiently documented](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#api-documentation)?
  - [ ] Is `#![deny(missing_docs)]` present?

### Readability
- [ ] Is it intuitively clear how the code should work, just by reading it? Be VERY STRICT on this point, it is arguably the most important one.
- [ ] Are all [naming conventions](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#naming-conventions) upheld?

### Clean Code
- [ ] Has code duplication been kept to a minimum?
- [ ] Are all functions so small that they only do [one thing](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#function-size)?
- [ ] Are [abstraction layers](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#abstraction-layers) cleanly separated?

### Test coverage
- [ ] Are there integration tests in the `tests` directory?
- [ ] Are there enough integration tests?
  - [ ] sample input (if applicable)
  - [ ] regular input
- [ ] Are there [enough unit tests](https://github.com/remlse/aoc/blob/main/.github/clean_code_ref.md#unit-testing)?
