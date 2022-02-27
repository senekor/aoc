# Review Checklist

### Project structure
- [ ] Is the project split into a library and a binary?
- [ ] Are there integration tests in the `tests` directory?
- [ ] Is the library's [module structure appropriate](clean_code_ref#module-size)?
- [ ] Is the library's [API sufficiently documented](clean_code_ref#api-documentation)?
  - [ ] Is `#![deny(missing_docs)]` present?

### Readability
- [ ] Is it intuitively clear how the code should work, just by reading it? Be VERY STRICT on this point, it is arguably the most important one.
- [ ] Are all [naming conventions](clean_code_ref#naming-conventions) upheld?

### Clean Code
- [ ] Has code duplication been kept to a minimum?
- [ ] Are all functions so small that they only do [one thing](clean_code_ref#function-size)?
- [ ] Are [abstraction layers](clean_code_ref#abstraction-layers) cleanly separated?

### Test coverage
- [ ] Are there enough integration tests?
  - [ ] sample input
  - [ ] regular input
- [ ] Are there [enough unit tests](clean_code_ref#unit-testing)?
