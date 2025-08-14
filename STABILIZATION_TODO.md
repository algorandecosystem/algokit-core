# Stabilization TODO

Items to revisit during codebase stabilization phase.

## API Consistency & Code Generation

### Vendor Extension System Review
**Priority:** Medium  
**Context:** Completed elegant vendor extension approach for client field customization (commit a7d71c0)

**Current State:**
- Implemented robust vendor extension system in `api/scripts/convert-openapi.ts`
- Uses `x-algokit-field-rename` and `x-algokit-bytes-base64` extensions
- Python generator reads extensions instead of hardcoded schema names
- Survives upstream schema changes, maintainable architecture

**Stabilization Tasks:**
1. **Review Extension Naming Convention:** Consider if `x-algokit-*` is the best namespace or if we should use more specific prefixes
2. **Documentation:** Add comprehensive docs for the vendor extension system for future maintainers
3. **Extension Registry:** Consider creating a formal registry of all custom extensions we support
4. **Validation:** Add validation to ensure extensions are applied correctly during spec processing
5. **Testing:** Add unit tests for the vendor extension transformation functions
6. **Performance:** Review if the deep object traversal in JS preprocessing could be optimized

**Files to Review:**
- `api/scripts/convert-openapi.ts` (extension injection)
- `api/oas_generator/rust_oas_generator/parser/oas_parser.py` (extension consumption)
- Generated specs: `api/specs/algod.oas3.json`, `api/specs/indexer.oas3.json`

### r#type Pattern Consistency 
**Priority:** Low (Breaking Change)  
**Context:** PR review task #10 - Consider r#type pattern for API consistency
**Reviewer:** neilcampbell  
**Original Comment:** "I'm wondering if we should apply the r#type pattern elsewhere given that our API types use it. Maybe one for stabilisation."

**Current State:**
- **API Types (auto-generated):** Use `r#type` pattern (e.g., `TealValue.r#type`)
- **ABI Types (manual):** Use descriptive names (e.g., `ABIType.field_type`, `ABIMethod.method_type`)

**Analysis:**
- Both patterns serve their domains well
- Generated code appropriately uses `r#type` (matches OpenAPI field names)  
- Manual code appropriately uses semantic names (better developer experience)

**Stabilization Decision:**
- **Recommend:** DEFER or NO ACTION - current design is appropriate
- **Rationale:** Changing to `r#type` everywhere would be breaking and reduce readability in manual code
- **Alternative:** Document the pattern distinction as intentional design choice

**Files to Review if Action Taken:**
- `crates/algokit_abi/src/abi_type.rs`
- `crates/algokit_abi/src/method.rs`
- Any other manual types using descriptive field names

## Future Vendor Extension Opportunities

**Potential Extensions to Consider:**
1. **Field Type Overrides:** `x-algokit-rust-type` for custom type mapping
2. **Validation Extensions:** `x-algokit-validate` for custom validation rules
3. **Documentation Extensions:** `x-algokit-doc-example` for enhanced documentation
4. **Serde Extensions:** `x-algokit-serde-*` for custom serialization behavior

**Review Checklist for Stabilization:**
- [ ] Vendor extension system documentation complete
- [ ] Extension naming conventions documented
- [ ] r#type consistency decision documented
- [ ] Performance impact of extensions measured
- [ ] Breaking change policy for extensions defined
- [ ] Migration guide for extension changes written

---

*This file tracks items identified during development that should be revisited before the codebase is considered stable and ready for production use.*