# Visual thesis: glacial minimal ceramics

Supabase Exit Map should feel like a careful survey made on a cool ceramic workbench: quiet, tactile, and precise. The product is not a replacement cloud and does not sell urgency. Its visual world uses pale ice, chalky clay, fine graphite rules, and a single mineral blue to make a risky infrastructure decision feel inspectable.

## Palette

Light is the primary treatment; a dark, polar-night treatment follows the same material hierarchy.

| Token | Light | Dark | Role |
| --- | --- | --- | --- |
| `ice` | `#F2F6F4` | `#101917` | page ground |
| `porcelain` | `#FCFEFD` | `#17221F` | raised surface |
| `graphite` | `#142522` | `#ECF4F1` | primary text |
| `slate` | `#536661` | `#AEBDB8` | secondary text |
| `meltwater` | `#0A6963` | `#64CFC3` | actions, focus, links |
| `fjord` | `#0B3F3B` | `#B7F0E9` | deep accent / action text |
| `lichen` | `#377348` | `#88D49A` | verified / portable |
| `ochre` | `#8A5A0A` | `#F1C56E` | medium effort / caution |
| `iron` | `#A33B34` | `#FF9A92` | high effort / error |
| `hairline` | `#CAD8D3` | `#354943` | boundaries |

Body and interface combinations are chosen for WCAG AA contrast. Status always has an icon or text label as well as color.

## Type and spacing

- Headings: `Georgia`, `Iowan Old Style`, serif. Its carved terminals recall makers' stamps pressed into clay.
- UI and body: `Inter`, `Avenir Next`, system sans. No font download is required; the fast system stack keeps the CLI documentation immediate and private.
- Type scale: 14, 16, 20, 28, 44, and clamp(48–76) px. Body is 17 px on narrow screens and never below 16 px.
- Spacing uses a strict 4/8 px rhythm: 4, 8, 12, 16, 24, 32, 48, 64, 96. Text measure stays below 72 characters.

## Composition and interaction grammar

Large asymmetrical whitespace represents room to leave. Fine contour lines and numbered map marks turn the page into a survey rather than a dashboard. Rounded forms are used only for the generated ceramic objects, tags, and compact controls; content groups rely on proximity and rules instead of a grid of generic cards. The primary action is dark mineral green. Focus is a 3 px meltwater ring with a porcelain gap. Every tap target is at least 44 px.

The phone layout drops the wide terminal annotation, stacks the dependency route vertically, keeps commands horizontally scrollable, and presents the paid planning area after the complete free workflow.

## Motion

On entry, map marks resolve once with 180–260 ms opacity and translate transitions, following the top-to-bottom reading path. Buttons compress by 1 px when pressed. Nothing loops. With `prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed and state changes are immediate opacity swaps.

## Asset plan and provenance

The single hero asset is a generated still life: abstract glazed ceramic slabs connected by a thin route, moving from a luminous ice block to a plain dark clay database cylinder. It explains dependency unbundling without logos, product UI, or text. It will be generated with the factory `factory-image` deployment via `/opt/fleet/lib/gen-image.sh`, then cropped/converted locally to responsive WebP files of at most 300 KB.

Prompt (authored for this product):

> Use case: stylized-concept. Asset type: landing-page hero illustration. Scene/backdrop: a pale blue-gray seamless ceramic studio surface. Subject: five distinct handmade glazed ceramic forms connected by a single hair-thin dark green route, beginning at a translucent icy monolith and separating toward a plain matte charcoal database cylinder; the forms subtly suggest database, key, file, broadcast signal, and function without literal icons. Style/medium: editorial studio still-life photography of handcrafted porcelain and celadon clay, restrained and architectural. Composition: wide 3:2 landscape, objects concentrated through the center and right, calm negative space, eye-level three-quarter view. Lighting: overcast glacial daylight, soft long shadows, quiet and analytical. Palette: chalk porcelain, pale celadon, mineral teal, graphite, one tiny amber marker. Materials: visible fine clay grain, imperfect hand-finished glaze, frosted ice. Constraints: no text, no letters, no logos, no Supabase branding, no people, no screens, no neon, no generic 3D SaaS illustration, no watermark.

The generated image is an original project asset. Its `.png.json` generation receipt is retained in `site/public/assets/`; final WebP derivatives are project-authored transformations. CSS contour lines and map marks are hand-authored and require no external license.

The 1200×630 social image and 180×180 touch icon are center crops of that same generated hero image. The self-hosted demo-terminal.svg is hand-authored from the bundled CLI demo output and has a text transcript on the demo page. No external fonts, scripts, imagery, or icon sets are loaded.
