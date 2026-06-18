# 🤝 Contributing to StellarPOS Surface

We love your input! We want to make contributing to StellarPOS as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## 🚀 Quick Start for Contributors

### Prerequisites
- Node.js 18+ and npm/yarn
- Git knowledge
- Basic React/TypeScript experience

### Setup Development Environment

```bash
# Fork and clone the repository
git clone https://github.com/YOUR-USERNAME/stellarPOS-surface.git
cd stellarPOS-surface

# Install dependencies
npm install

# Start development server
npm run dev

# Run tests
npm test
```

## 📋 Development Process

We use [GitHub Flow](https://guides.github.com/introduction/flow/index.html):

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Make** your changes
4. **Add tests** for new functionality
5. **Ensure** all tests pass
6. **Submit** a pull request

### Branch Naming Convention
- `feature/` - New features
- `bugfix/` - Bug fixes  
- `hotfix/` - Critical fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring

Examples:
- `feature/dark-mode-toggle`
- `bugfix/safari-compatibility`
- `docs/api-documentation`

## 🧪 Testing Guidelines

### Running Tests
```bash
# Unit tests
npm run test

# E2E tests
npm run test:e2e

# Test coverage
npm run test:coverage

# Test in watch mode
npm run test:watch
```

### Writing Tests
- Write tests for all new features
- Maintain >80% test coverage
- Use descriptive test names
- Follow AAA pattern (Arrange, Act, Assert)

## 🎨 Code Style Guidelines

### TypeScript/React
- Use TypeScript for all new code
- Follow React hooks best practices
- Use functional components over class components
- Implement proper error boundaries

### CSS/Styling
- Use Tailwind CSS classes
- Follow mobile-first responsive design
- Maintain consistent spacing
- Use semantic HTML elements

### Code Formatting
```bash
# Format code
npm run format

# Lint code
npm run lint

# Type check
npm run type-check
```

## 📝 Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

### Examples
```
feat(auth): add biometric authentication support

fix(pos): resolve transaction timeout issue

docs(api): update endpoint documentation

style(ui): improve button hover states
```

## 🐛 Bug Reports

Great Bug Reports tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening)

### Bug Report Template
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g. iOS]
- Browser [e.g. chrome, safari]
- Version [e.g. 22]
```

## ✨ Feature Requests

We use GitHub Issues to track feature requests. When proposing a feature:

1. **Check existing issues** to avoid duplicates
2. **Provide context** on the problem you're trying to solve
3. **Describe the solution** you'd like to see
4. **Consider alternatives** you've thought about
5. **Add mockups** or examples if helpful

## 🔍 Pull Request Process

### Before Submitting
- [ ] Tests pass locally (`npm test`)
- [ ] Code is properly formatted (`npm run format`)
- [ ] No linting errors (`npm run lint`)
- [ ] TypeScript checks pass (`npm run type-check`)
- [ ] Documentation is updated
- [ ] Screenshots for UI changes

### PR Description Template
```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] E2E tests added/updated
- [ ] Manual testing completed

## Screenshots (if applicable)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for hard-to-understand areas
- [ ] Documentation updated
- [ ] No new warnings introduced
```

## 🏷️ Issue Labels

We use labels to categorize issues:

### Priority
- `high-priority` - Critical issues
- `medium-priority` - Standard priority
- `low-priority` - Nice-to-have

### Type
- `bug` - Something isn't working
- `enhancement` - New feature request
- `documentation` - Documentation needs
- `question` - Further information needed

### Component
- `ui` - User interface
- `api` - API integration
- `performance` - Performance related
- `security` - Security concerns
- `accessibility` - Accessibility improvements

### Difficulty
- `good first issue` - Good for newcomers
- `help wanted` - Community help needed
- `advanced` - Requires deep knowledge

## 🌍 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

### Communication
- Use clear, concise language
- Provide context and examples
- Be patient with newcomers
- Ask questions when unsure

## 🏆 Recognition

Contributors are recognized through:
- README acknowledgments
- CONTRIBUTORS.md listing
- Release notes mentions
- Annual contributor highlights
- Special badges and recognition

## 📞 Getting Help

Need help contributing? Reach out through:
- **GitHub Issues** - For bugs and features
- **Discussions** - For questions and ideas
- **Discord** - For real-time chat
- **Email** - team@stellarpos.app

## 📚 Resources

- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [Stellar Development](https://developers.stellar.org/)
- [Testing Library](https://testing-library.com/docs/)

---

**Thank you for contributing to StellarPOS!** 🚀

Your contributions make this project better for everyone in the Stellar ecosystem.