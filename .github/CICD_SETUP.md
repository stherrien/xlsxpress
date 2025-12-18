# CI/CD Setup Guide for XlsXpress

This guide explains how to set up and use the CI/CD pipelines for building and releasing XlsXpress to PyPI.

## Overview

The CI/CD system consists of four main workflows:

1. **CI (`ci.yml`)** - Runs tests on every push/PR
2. **Build (`build.yml`)** - Builds wheels for all platforms
3. **Test Release (`test-release.yml`)** - Manual testing of builds
4. **Release (`release.yml`)** - Publishes to PyPI

## Prerequisites

### GitHub Repository Setup

1. **Enable GitHub Actions**
   - Go to your repository Settings → Actions → General
   - Enable "Allow all actions and reusable workflows"

2. **Configure PyPI Trusted Publishing** (Recommended)

   **For PyPI (production):**
   - Go to [PyPI Account Settings](https://pypi.org/manage/account/)
   - Navigate to "Publishing" section
   - Click "Add a new pending publisher"
   - Fill in:
     - PyPI Project Name: `xlsxpress`
     - Owner: `yourusername` (your GitHub username/org)
     - Repository name: `xlsxpress`
     - Workflow name: `release.yml`
     - Environment name: `pypi`

   **For Test PyPI (testing):**
   - Go to [Test PyPI Account Settings](https://test.pypi.org/manage/account/)
   - Follow the same steps as above
   - Environment name: `test-pypi`

3. **Alternative: API Token Method** (if not using trusted publishing)

   If you prefer API tokens instead:
   - Generate a PyPI API token at https://pypi.org/manage/account/token/
   - Add it as a GitHub secret:
     - Repository Settings → Secrets and variables → Actions
     - New repository secret: `PYPI_API_TOKEN`
   - Update `release.yml` to use the token instead of trusted publishing

### Local Development Setup

1. **Install Rust and Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Python 3.10+**
   ```bash
   # macOS
   brew install python@3.12

   # Ubuntu
   sudo apt-get install python3.12
   ```

3. **Install Maturin**
   ```bash
   pip install maturin[patchelf]
   ```

4. **Install Development Dependencies**
   ```bash
   pip install -e ".[dev]"
   ```

## Workflows Explained

### 1. CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `dev` branches
- Pull requests to `main` or `dev`

**Jobs:**
- **rust-test**: Runs Rust tests, clippy, and formatting checks
- **python-test**: Builds Python package and runs Python tests (if they exist)
- **security-audit**: Runs `cargo audit` for security vulnerabilities
- **msrv**: Checks minimum supported Rust version (1.70)

**Platforms Tested:**
- Linux (ubuntu-latest)
- macOS (macos-latest)
- Windows (windows-latest)

**Python Versions:**
- 3.10, 3.11, 3.12

### 2. Build Workflow (`build.yml`)

**Triggers:**
- Push to `main` branch
- Git tags starting with `v*`
- Manual dispatch

**Jobs:**
- **build-linux**: Builds wheels for x86_64 and aarch64 Linux
- **build-macos**: Builds wheels for x86_64 and aarch64 macOS
- **build-windows**: Builds wheels for x64 Windows
- **build-sdist**: Builds source distribution
- **test-wheels**: Tests wheel installation on all platforms

**Artifacts:**
- All built wheels are uploaded as GitHub artifacts
- Retained for 90 days

### 3. Test Release Workflow (`test-release.yml`)

**Triggers:**
- Manual dispatch only

**Purpose:**
- Test building wheels locally before actual release
- Run comprehensive smoke tests
- Verify all functionality works in built wheels

**Use Cases:**
- Pre-release validation
- Testing changes to build configuration
- Debugging build issues

### 4. Release Workflow (`release.yml`)

**Triggers:**
- GitHub Release published
- Manual dispatch with option to publish to Test PyPI

**Jobs:**
- **build-wheels**: Builds wheels for all platforms
- **build-sdist**: Builds source distribution
- **publish**: Publishes to PyPI or Test PyPI
- **github-release**: Attaches artifacts to GitHub release

**Important Notes:**
- Uses trusted publishing (no API tokens needed)
- Publishes to Test PyPI when manually dispatched with `test-pypi` option
- Publishes to production PyPI only on GitHub releases

## Usage Guide

### Testing Changes (Before Release)

1. **Run CI Tests**
   ```bash
   # Push to dev branch or create PR
   git checkout -b feature/my-feature
   git push origin feature/my-feature
   ```
   - CI workflow runs automatically
   - Check Actions tab for results

2. **Test Local Build**
   ```bash
   # Build wheel locally
   maturin build --release

   # Install and test
   pip install target/wheels/*.whl
   python -c "import xlsxpress; print('Success!')"
   ```

3. **Run Test Release Workflow**
   - Go to Actions → Test Release Build
   - Click "Run workflow"
   - Select Python version (optional)
   - Check results and artifacts

### Creating a Release

#### Option 1: Using GitHub Releases (Recommended)

1. **Update Version Number**
   ```bash
   # Update version in Cargo.toml
   vim Cargo.toml
   # Change: version = "0.1.0" to version = "0.2.0"

   # Update version in pyproject.toml
   vim pyproject.toml
   # Change: version = "0.1.0" to version = "0.2.0"
   ```

2. **Commit and Tag**
   ```bash
   git add Cargo.toml pyproject.toml
   git commit -m "Bump version to 0.2.0"
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin main
   git push origin v0.2.0
   ```

3. **Create GitHub Release**
   - Go to Releases → Draft a new release
   - Choose tag: `v0.2.0`
   - Title: `v0.2.0`
   - Description: Add release notes
   - Click "Publish release"

4. **Verify Release**
   - Check Actions tab for release workflow
   - Verify PyPI publication: https://pypi.org/project/xlsxpress/
   - Test installation: `pip install xlsxpress==0.2.0`

#### Option 2: Manual Dispatch (Test PyPI)

1. **Test with Test PyPI First**
   - Go to Actions → Release to PyPI
   - Click "Run workflow"
   - Check "Publish to Test PyPI"
   - Click "Run workflow"

2. **Verify Test PyPI Installation**
   ```bash
   pip install --index-url https://test.pypi.org/simple/ xlsxpress
   ```

3. **If Successful, Release to Production PyPI**
   - Create GitHub Release (see Option 1)
   - Or run workflow without "test-pypi" option

### Troubleshooting

#### Build Failures

**Rust Compilation Errors:**
```bash
# Run locally to debug
cargo build --release
cargo clippy -- -D warnings
cargo test
```

**Python Build Errors:**
```bash
# Build with verbose output
maturin build --release -vv

# Check Python compatibility
python --version
pip list | grep maturin
```

#### PyPI Publishing Failures

**Trusted Publishing Not Configured:**
- Error: `Invalid or non-existent authentication information`
- Solution: Set up trusted publishing (see Prerequisites)

**Version Already Exists:**
- Error: `File already exists`
- Solution: Bump version number in `Cargo.toml` and `pyproject.toml`

**Network/Timeout Issues:**
- Check GitHub Actions status page
- Try manual dispatch again
- Contact PyPI support if persistent

#### Wheel Installation Failures

**Missing Platform:**
```bash
# Check available wheels on PyPI
pip index versions xlsxpress

# Build for specific platform
maturin build --release --target <platform>
```

**Import Errors:**
```python
# Test module structure
import xlsxpress
print(dir(xlsxpress))
print(xlsxpress.__version__)
```

## Security Considerations

1. **Trusted Publishing** (Recommended)
   - No API tokens stored in GitHub
   - OpenID Connect (OIDC) authentication
   - More secure than API tokens

2. **API Tokens** (Alternative)
   - Store in GitHub Secrets (encrypted)
   - Use scoped tokens (project-specific)
   - Rotate regularly

3. **Dependency Security**
   - `cargo audit` runs on every CI build
   - Dependabot enabled for automatic updates
   - Review security advisories regularly

## Best Practices

1. **Version Management**
   - Use semantic versioning (semver)
   - Update CHANGELOG.md for each release
   - Tag releases with `v` prefix (e.g., `v0.2.0`)

2. **Testing Before Release**
   - Always test with Test PyPI first
   - Run test-release workflow
   - Verify wheels on multiple platforms

3. **Release Notes**
   - Document breaking changes
   - List new features
   - Include upgrade instructions
   - Credit contributors

4. **Git Workflow**
   - Use feature branches
   - Require PR reviews
   - Run CI on all PRs
   - Merge to `main` only when CI passes

## Monitoring and Maintenance

### GitHub Actions Usage

- Check Actions usage: Settings → Billing → Plans and usage
- Free tier: 2,000 minutes/month for private repos
- Public repos: Unlimited

### PyPI Statistics

- View download stats: https://pypistats.org/packages/xlsxpress
- Monitor package health: https://libraries.io/pypi/xlsxpress

### Dependency Updates

1. **Rust Dependencies**
   ```bash
   cargo update
   cargo test
   ```

2. **Python Dependencies**
   ```bash
   pip install --upgrade maturin
   pip install -e ".[dev]" --upgrade
   ```

3. **GitHub Actions**
   - Dependabot creates PRs automatically
   - Review and merge updates

## Additional Resources

- [Maturin Documentation](https://www.maturin.rs/)
- [PyO3 Documentation](https://pyo3.rs/)
- [PyPI Trusted Publishers](https://docs.pypi.org/trusted-publishers/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Packaging Guide](https://doc.rust-lang.org/cargo/guide/)

## Support

For issues with CI/CD:
1. Check GitHub Actions logs
2. Review this documentation
3. Open an issue on GitHub
4. Contact maintainers

---

**Last Updated:** 2025-12-17
**Version:** 1.0
