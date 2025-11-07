# Safari Skills Passport - Frontend UI

## 🎨 Modern, Accessible, User-Friendly Interface

A comprehensive web-based user interface for the Safari Skills Passport blockchain credentialing platform.

## ✨ Features

### 🎯 Core Functionality
- **Authentication**: Secure login and registration with JWT
- **Dashboard**: Personalized credential management
- **Verification**: Dual-mode credential verification (QR scan + manual entry)
- **Real-time Updates**: Live credential status and statistics
- **Responsive Design**: Mobile-first, works on all devices

### ♿ Accessibility Features
- **WCAG 2.1 AA Compliant**: Follows web accessibility guidelines
- **Screen Reader Support**: Proper ARIA labels and semantic HTML
- **Keyboard Navigation**: Full keyboard accessibility
- **Skip Links**: Quick navigation for screen reader users
- **Focus Indicators**: Clear visual focus states
- **Reduced Motion**: Respects user's motion preferences
- **Color Contrast**: AAA-rated text contrast ratios
- **Alt Text**: Descriptive alternatives for all images

### 🎨 User Experience
- **Modern Design**: Clean, professional interface
- **Smooth Animations**: Subtle, performant transitions
- **Toast Notifications**: Non-intrusive user feedback
- **Loading States**: Clear loading indicators
- **Empty States**: Helpful guidance when no data exists
- **Error Handling**: User-friendly error messages
- **Mobile Optimized**: Touch-friendly, responsive layout

### 🔒 Security
- **JWT Authentication**: Secure token-based auth
- **HTTPS Ready**: Secure communication
- **XSS Protection**: Content Security Policy headers
- **Input Validation**: Client-side validation
- **Secure Storage**: localStorage with expiration

## 📁 Project Structure

```
frontend/
├── index.html          # Main HTML file
├── css/
│   └── styles.css      # Comprehensive styles
├── js/
│   └── app.js          # Application logic
└── README.md           # This file
```

## 🚀 Quick Start

### 1. Local Development

Simply open `index.html` in a browser or serve with a local server:

```bash
# Using Python
cd frontend
python3 -m http.server 8000

# Using Node.js
npx serve -p 8000

# Using PHP
php -S localhost:8000
```

Then visit: `http://localhost:8000`

### 2. Configure API Endpoint

Edit `js/app.js` and update the API base URL:

```javascript
const CONFIG = {
    API_BASE_URL: 'http://localhost:3000/api',  // Change to your API server
    ...
};
```

### 3. Run Backend API

Ensure the backend API server is running:

```bash
cd ../
cargo run --bin api-server
```

## 📖 Usage Guide

### For Professionals

1. **Register**: Click "Get Started" → Fill registration form → Select "Professional"
2. **Login**: Use your email and password
3. **Dashboard**: View your credentials and statistics
4. **Share**: Download QR codes to share with employers

### For Institutions

1. **Register**: Select "Institution" during registration
2. **Issue Credentials**: Use the dashboard to issue credentials to professionals
3. **Manage**: Track issued credentials and their verification status

### For Employers

1. **Verify**: No registration needed
2. **Scan QR**: Use QR scanner or enter credential ID
3. **Instant Results**: See verification status in under 10 seconds

## 🎨 Customization

### Colors

Edit CSS variables in `css/styles.css`:

```css
:root {
    --primary-color: #10b981;      /* Brand color */
    --secondary-color: #3b82f6;    /* Secondary brand */
    --success-color: #10b981;      /* Success messages */
    --error-color: #ef4444;        /* Error messages */
    ...
}
```

### Typography

Change font family:

```css
:root {
    --font-family: 'Inter', sans-serif;  /* Update to your font */
}
```

### Layout

Adjust container width:

```css
.container {
    max-width: 1200px;  /* Change max width */
}
```

## 🔧 Advanced Features

### QR Code Scanner

The app uses [html5-qrcode](https://github.com/mebjas/html5-qrcode) for camera-based QR scanning:

```html
<script src="https://unpkg.com/html5-qrcode@2.3.8/html5-qrcode.min.js"></script>
```

**Browser Support**: Chrome, Firefox, Safari (iOS), Edge

### Local Storage

User authentication data is stored in localStorage:

- `ssp_auth_token`: JWT token
- `ssp_user_data`: User profile data

**Auto-logout**: Tokens expire after the backend-defined period

### API Integration

All API calls use the `apiRequest()` function:

```javascript
const data = await apiRequest('/credentials', {
    method: 'POST',
    body: JSON.stringify({ ... })
});
```

## 📱 Responsive Breakpoints

- **Mobile**: < 768px
- **Tablet**: 768px - 1024px
- **Desktop**: > 1024px

## ♿ Accessibility Compliance

### WCAG 2.1 AA Standards

✅ **Perceivable**
- Color contrast ratios meet AAA standard (7:1)
- All images have descriptive alt text
- Captions for video content (when applicable)

✅ **Operable**
- All functionality available via keyboard
- Skip links for navigation
- No time limits on interactions
- Clear focus indicators

✅ **Understandable**
- Clear, consistent navigation
- Error messages with suggestions
- Labels on all form inputs
- Predictable behavior

✅ **Robust**
- Valid HTML5 markup
- ARIA labels where needed
- Compatible with assistive technologies
- Cross-browser compatible

### Screen Reader Testing

Tested with:
- NVDA (Windows)
- JAWS (Windows)
- VoiceOver (macOS/iOS)
- TalkBack (Android)

## 🌐 Browser Support

| Browser | Version | Status |
|---------|---------|--------|
| Chrome  | 90+     | ✅ Full Support |
| Firefox | 88+     | ✅ Full Support |
| Safari  | 14+     | ✅ Full Support |
| Edge    | 90+     | ✅ Full Support |
| Opera   | 76+     | ✅ Full Support |

## 🔐 Security Best Practices

### Implemented

- ✅ HTTPS enforcement (production)
- ✅ JWT token authentication
- ✅ XSS protection via Content Security Policy
- ✅ Input sanitization
- ✅ Secure password requirements
- ✅ Auto-logout on token expiration

### Recommendations

1. **Use HTTPS**: Always serve over HTTPS in production
2. **CSP Headers**: Configure Content-Security-Policy headers
3. **Rate Limiting**: Implement on backend API
4. **Token Refresh**: Implement refresh token mechanism
5. **2FA**: Add two-factor authentication (future enhancement)

## 📊 Performance

### Optimization Techniques

- **Lazy Loading**: Images loaded on demand
- **Minification**: CSS/JS minified for production
- **Caching**: Browser caching for static assets
- **CDN**: External resources from CDN
- **Compression**: Gzip/Brotli compression

### Lighthouse Scores (Target)

- **Performance**: 95+
- **Accessibility**: 100
- **Best Practices**: 95+
- **SEO**: 95+

## 🎯 Roadmap

### Planned Features

- [ ] Progressive Web App (PWA) support
- [ ] Offline mode with service workers
- [ ] Multi-language support (i18n)
- [ ] Dark mode toggle
- [ ] Advanced search and filters
- [ ] Credential sharing via email/SMS
- [ ] Bulk verification for employers
- [ ] Analytics dashboard
- [ ] Notification system
- [ ] Export credentials to PDF

### Future Enhancements

- [ ] Blockchain explorer integration
- [ ] IPFS file viewer
- [ ] Video credential support
- [ ] AI-powered credential verification
- [ ] Integration with LinkedIn
- [ ] Mobile apps (React Native)

## 🐛 Troubleshooting

### Common Issues

**Q: QR scanner not working**
- Ensure camera permissions are granted
- Use HTTPS (camera API requires secure context)
- Check browser compatibility

**Q: Login not working**
- Verify backend API is running
- Check API_BASE_URL in config
- Open browser console for errors

**Q: Credentials not loading**
- Check network tab in DevTools
- Verify JWT token is valid
- Ensure backend database is seeded

**Q: Styles not applying**
- Clear browser cache
- Check CSS file path
- Verify no console errors

## 📝 Development

### Local Development Workflow

1. **Edit Code**: Make changes to HTML/CSS/JS
2. **Refresh Browser**: See changes instantly
3. **Test**: Check functionality and accessibility
4. **Commit**: Git commit with descriptive message

### Code Style

- **HTML**: Semantic, accessible markup
- **CSS**: BEM-like naming, CSS variables
- **JavaScript**: ES6+, async/await, clear comments
- **Comments**: Explain "why", not "what"

### Testing Checklist

- [ ] Keyboard navigation works
- [ ] Screen reader announces properly
- [ ] Mobile layout responsive
- [ ] All forms validate correctly
- [ ] Error states display properly
- [ ] Success notifications appear
- [ ] Loading states show/hide
- [ ] Cross-browser compatible

## 📄 License

Part of Safari Skills Passport project. See main repository for license details.

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📧 Support

For issues or questions:
- Open a GitHub issue
- Email: support@safariskillspassport.com
- Documentation: See main README

## 🙏 Acknowledgments

- **Inter Font**: Rasmus Andersson
- **Font Awesome**: Fonticons, Inc.
- **html5-qrcode**: Minhaz
- **Inspiration**: Modern web design trends and accessibility guidelines

---

**Built with ❤️ for African professionals**

*Last Updated: October 22, 2025*
