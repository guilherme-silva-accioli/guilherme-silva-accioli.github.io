## The Geometry of Modern Layouts

CSS Grid revolutionized web styling by introducing true two-dimensional layout orchestration. Unlike Flexbox (which excels at 1D rows or columns), Grid manages both rows and columns simultaneously.

---

### The 3-Column Specimen Grid

To build our vintage 3-column field journal:

```css
.thumbnail-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 2rem;
}

@media (max-width: 960px) {
    .thumbnail-grid {
        grid-template-columns: repeat(2, 1fr);
    }
}

@media (max-width: 620px) {
    .thumbnail-grid {
        grid-template-columns: 1fr;
    }
}
```

### Aesthetic Takeaways

* **`aspect-ratio: 16 / 9`** prevents cumulative layout shift (CLS) as images stream in.
* **`object-fit: cover`** guarantees thumbnails fill the frame uniformly regardless of source dimensions.
* **Warm subtle borders & shadows** replace harsh modern box models with artisanal depth.
