# Release Checklist for XlsXpress

Use this checklist when preparing a new release.

## Pre-Release (1-2 days before)

### Code Preparation
- [ ] All tests passing on `main` branch
- [ ] No open blocking issues
- [ ] All PRs for this release merged
- [ ] Documentation updated
- [ ] CHANGELOG.md updated with release notes
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] No rustfmt issues: `cargo fmt -- --check`

### Version Bump
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `pyproject.toml`
- [ ] Update version in `README.md` (if mentioned)
- [ ] Update version in `CHANGELOG.md` header
- [ ] Commit: `git commit -m "Bump version to X.Y.Z"`

### Pre-Release Testing
- [ ] Build locally: `maturin build --release`
- [ ] Test local wheel: `pip install target/wheels/*.whl`
- [ ] Run test suite: `cargo test`
- [ ] Run integration tests (if available)
- [ ] Run manual smoke tests
- [ ] Test on Linux (via Docker or VM)
- [ ] Test on macOS
- [ ] Test on Windows (via VM or GitHub Actions)

### Test PyPI Release
- [ ] Push to `main`: `git push origin main`
- [ ] Run "Test Release Build" workflow manually
- [ ] Verify workflow completes successfully
- [ ] Run "Release to PyPI" workflow with `test-pypi` option checked
- [ ] Verify Test PyPI publication: https://test.pypi.org/project/xlsxpress/
- [ ] Test installation from Test PyPI:
  ```bash
  pip install --index-url https://test.pypi.org/simple/ xlsxpress==X.Y.Z
  python -c "import xlsxpress; print(xlsxpress.__version__)"
  ```
- [ ] Run smoke tests with Test PyPI package

## Release Day

### Create Release
- [ ] Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Go to GitHub Releases
- [ ] Click "Draft a new release"
- [ ] Choose tag: `vX.Y.Z`
- [ ] Release title: `vX.Y.Z`
- [ ] Copy release notes from CHANGELOG.md
- [ ] Mark as pre-release if beta/alpha
- [ ] Click "Publish release"

### Monitor Release
- [ ] Watch "Release to PyPI" workflow in Actions tab
- [ ] Verify all platform builds succeed:
  - [ ] Linux x86_64
  - [ ] Linux aarch64
  - [ ] macOS x86_64
  - [ ] macOS aarch64 (Apple Silicon)
  - [ ] Windows x64
  - [ ] Source distribution (sdist)
- [ ] Verify PyPI publication: https://pypi.org/project/xlsxpress/
- [ ] Verify all wheels uploaded to PyPI
- [ ] Verify GitHub release has artifacts attached

### Post-Release Testing
- [ ] Install from PyPI: `pip install xlsxpress==X.Y.Z`
- [ ] Verify version: `python -c "import xlsxpress; print(xlsxpress.__version__)"`
- [ ] Run basic smoke test:
  ```python
  import xlsxpress
  writer = xlsxpress.Writer()
  sheet = writer.add_worksheet('Test')
  writer.write_string(sheet, 0, 0, 'Release X.Y.Z')
  writer.save('release_test.xlsx')
  print('Success!')
  ```
- [ ] Test on fresh virtual environment
- [ ] Test with Python 3.10
- [ ] Test with Python 3.11
- [ ] Test with Python 3.12

## Post-Release

### Communication
- [ ] Announce release on GitHub Discussions (if enabled)
- [ ] Tweet/post about release (if applicable)
- [ ] Update documentation site (if applicable)
- [ ] Notify users in Discord/Slack (if applicable)
- [ ] Email mailing list (if applicable)

### Maintenance
- [ ] Close milestone in GitHub (if using milestones)
- [ ] Move open issues to next milestone
- [ ] Update project board (if using projects)
- [ ] Review and close related issues
- [ ] Monitor PyPI download stats
- [ ] Monitor GitHub issue tracker for bug reports

### Preparation for Next Release
- [ ] Create CHANGELOG.md entry for next version
- [ ] Create milestone for next version (if using milestones)
- [ ] Review roadmap and prioritize features
- [ ] Update TODO.md (if applicable)

## Emergency Rollback Procedure

If critical bugs are discovered:

1. **Immediate Actions**
   - [ ] Document the issue in GitHub Issues
   - [ ] Mark as "Critical" or "Blocker"
   - [ ] Notify users via GitHub announcement

2. **Yank Release from PyPI** (if necessary)
   ```bash
   # Login to PyPI
   # Go to https://pypi.org/manage/project/xlsxpress/releases/
   # Click "Delete" on the broken release
   # Or mark as "Yanked" (preferred - doesn't break existing installs)
   ```

3. **Hotfix Release**
   - [ ] Create hotfix branch: `git checkout -b hotfix/X.Y.Z+1`
   - [ ] Fix the critical issue
   - [ ] Bump patch version
   - [ ] Test thoroughly
   - [ ] Follow fast-track release process
   - [ ] Release as X.Y.Z+1

4. **Communication**
   - [ ] Post update on GitHub Releases
   - [ ] Notify via all channels used for original announcement
   - [ ] Document in CHANGELOG.md

## Version Numbering Guide

Follow Semantic Versioning (semver):

- **MAJOR** (X.0.0): Breaking changes
  - API changes that break backward compatibility
  - Removal of deprecated features
  - Major architectural changes

- **MINOR** (0.X.0): New features (backward compatible)
  - New chart types
  - New validation features
  - New styling options
  - Performance improvements

- **PATCH** (0.0.X): Bug fixes (backward compatible)
  - Bug fixes
  - Documentation updates
  - Security patches

### Pre-Release Versions
- **Alpha**: `X.Y.Z-alpha.N` - Early testing
- **Beta**: `X.Y.Z-beta.N` - Feature complete, testing
- **RC**: `X.Y.Z-rc.N` - Release candidate

## Release Frequency

Recommended schedule:
- **Major releases**: 6-12 months
- **Minor releases**: 1-3 months
- **Patch releases**: As needed (critical bugs)
- **Security patches**: Immediate

## Notes

- Always test on Test PyPI first
- Never rush a release - quality over speed
- Keep CHANGELOG.md updated continuously
- Document breaking changes clearly
- Provide migration guides for major versions

---

**Last Updated:** 2025-12-17
