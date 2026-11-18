# Deal Feed Density — design system (mydealz-inspired)

Selectable visual option for proposed feature UIs (`deal-feed` theme).

Open-source type substitute: **Mulish** (500/700) in place of Averta CY.

## Overview

A deals-community feed running at maximum density: a single cool-gray canvas stacked with white deal cards, each a compact bundle of thumbnail, title, price, and social signals. One confident brand green owns every price and primary CTA, plus a tight status vocabulary (hot/cold temperature) that keeps the eye moving.

**Key characteristics**

- One brand green owns every action and every price
- Signature hot/cold temperature control on every card
- Fully-round pills for buttons, search, chips, temperature; 8px cards
- Mint discount pills beside green prices
- Two type weights only (500 / 700)
- Cool-gray canvas behind stacked white cards
- Warm orange (rising) and premium violet kept strictly functional

## Colors (resolved tokens)

| Token | Value | Use |
|---|---|---|
| `primary` | `#30a83c` | CTAs, prices, active links |
| `on-primary` | `#ffffff` | Text on primary |
| `green-strong` | `#1f7a2d` | Green text on light surfaces |
| `hot` | `#e53935` | Temperature reading + up-vote |
| `cold` | `#1e88e5` | Down-vote |
| `warm` | `#fb8c00` | Rising / trending |
| `premium` | `#7e57c2` | Featured markers |
| `discount-surface` | `#e6f6e9` | Savings pill background |
| `canvas` | `#eef1f4` | Page ground |
| `surface` | `#ffffff` | Cards, nav, buttons |
| `hairline` | `#d9dee5` | 1px borders |
| `ink` | `#0b0b0b` | Titles |
| `body` | `#5c6670` | Secondary copy |
| `muted` | `#9aa3ad` | Placeholders / struck prices |
| `warning-surface` | `#fff4d6` | Caution pill |
| `warning-text` | `#8a5a00` | Caution label |

## Typography

| Token | Size | Weight | Use |
|---|---|---|---|
| `display` | 48px | 700 | Hero |
| `heading-lg` | 24px | 700 | Section / price |
| `heading` | 20px | 700 | Card headings |
| `lead` | 18px | 500 | Lead copy |
| `body` | 16px | 500 | Body |
| `ui` | 14px | 700 | Buttons, temperature, badges |
| `caption` | 12px | 500 | Metadata |
| `micro` | 10px | 700 | Counts |
| `mono` | 16px | 500 | Voucher codes |

## Layout & shape

- Spacing base 8px; card padding 16px; dense gaps 8–12px
- Desktop ≥768px: feed + sidebar; mobile: single column
- Radius: cards/thumbnails `8px`; pills `9999px` (full)

## Elevation

- Resting cards: hairline only
- Lift: soft `rgba(11,22,44,0.09) 0 4px 12px`

## Signature components

- `temperature-control` — cold down · hot reading · hot up
- `price-lockup` — green price + struck original + mint discount badge
- `button-primary` — green full pill
- `deal-card` — white card on canvas with media + social foot

## Do / Don't

**Do:** keep green for actions/prices; preserve cold/hot pairing; pill interactive chrome; two weights only.  
**Don't:** recolor prices; split temperature colors; heavy shadows on resting cards; third type weight; decorative warm/premium.
