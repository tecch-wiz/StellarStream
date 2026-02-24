# Glitch Text Quick Reference

One-page reference for the Cyberpunk Glitch Text component.

## Import

```tsx
import GlitchText from "@/components/glitch-text";
```

## Props

| Prop | Type | Default | Options |
|------|------|---------|---------|
| `as` | string | `"h1"` | `"h1"` \| `"h2"` \| `"h3"` \| `"h4"` \| `"h5"` \| `"h6"` \| `"span"` \| `"p"` |
| `glitchOnHover` | boolean | `true` | `true` (hover) \| `false` (always on) |
| `glitchIntensity` | string | `"medium"` | `"low"` \| `"medium"` \| `"high"` |
| `className` | string | `""` | Any CSS class |

## Quick Examples

```tsx
// Basic
<GlitchText as="h1">Title</GlitchText>

// With intensity
<GlitchText as="h2" glitchIntensity="high">Hero</GlitchText>

// Always on
<GlitchText as="span" glitchOnHover={false}>LIVE</GlitchText>

// With gradient
<GlitchText 
  as="h1" 
  className="bg-gradient-to-r from-cyan-400 to-violet-600 bg-clip-text text-transparent"
>
  Cyberpunk
</GlitchText>
```

## Intensity Levels

| Level | Shift | Opacity | Use Case |
|-------|-------|---------|----------|
| Low | 2px | 0.6 | Subtle, small text |
| Medium | 3px | 0.7 | Balanced, most headings |
| High | 5px | 0.8 | Dramatic, hero titles |

## Colors

- **Cyan**: `#00e5ff`
- **Violet**: `#8a2be2`
- **Base**: `#e8eaf6`

## Animation

- **Duration**: 0.2s
- **Easing**: cubic-bezier(0.25, 0.46, 0.45, 0.94)
- **FPS**: 60fps

## Common Patterns

### Hero Title
```tsx
<GlitchText as="h1" glitchIntensity="high" className="text-6xl">
  Welcome
</GlitchText>
```

### Section Header
```tsx
<GlitchText as="h2" glitchIntensity="medium">
  Features
</GlitchText>
```

### Card Title
```tsx
<GlitchText as="h3" glitchIntensity="low">
  Stream #123
</GlitchText>
```

### Status Badge
```tsx
<GlitchText as="span" glitchOnHover={false} glitchIntensity="low">
  ACTIVE
</GlitchText>
```

### CTA Button
```tsx
<button>
  <GlitchText as="span">Start Now</GlitchText>
</button>
```

## Best Practices

✅ Use sparingly (key headings only)  
✅ Match intensity to hierarchy  
✅ Hover for interactive elements  
✅ Always-on for status indicators  
✅ Combine with gradients  
❌ Don't overuse  
❌ Don't use on body text  

## Accessibility

- ✅ Text remains legible
- ✅ Fast animation (0.2s)
- ✅ Screen reader friendly
- ✅ Semantic HTML

## Performance

- CSS-only animations
- ~1KB bundle size
- 60fps on modern browsers
- Hardware accelerated

## Browser Support

- Chrome 76+
- Firefox 103+
- Safari 9+
- Edge 79+

## Troubleshooting

**Not glitching?**
→ Check browser support, ensure hover is working

**Too subtle?**
→ Increase intensity to "high"

**Too intense?**
→ Decrease intensity to "low" or use hover mode

**Performance issues?**
→ Limit simultaneous effects, use hover mode

## Related

- [Full Documentation](./README_GLITCH_TEXT.md)
- [Interactive Demo](./glitch-text-example.tsx)
- [Nebula Skeleton](./README_NEBULA_SKELETON.md)
